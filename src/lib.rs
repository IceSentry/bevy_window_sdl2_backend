#![allow(missing_docs, reason = "work in progress")]

use core::cell::RefCell;
use std::any::Any;
use std::sync::Arc;

use bevy_app::{App, AppExit, Last, Plugin, PluginsState};
use bevy_ecs::{
    change_detection::DetectChanges,
    component::Component,
    entity::{Entity, EntityHashMap},
    event::EventWriter,
    query::{Added, Changed, QueryFilter},
    system::{Commands, NonSendMarker, Query, SystemParamItem, SystemState},
    world::FromWorld,
};
use bevy_math::{DVec2, UVec2};
use bevy_platform::collections::HashMap;
use bevy_window::{CursorOptions, RawHandleWrapper, RawHandleWrapperHolder, WindowCreated};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use sdl2::{Sdl, VideoSubsystem, event::Event};

thread_local! {
    pub static SDL_WINDOWS: RefCell<SdlWindows> = const { RefCell::new(SdlWindows::new()) };

}

#[derive(Default)]
pub struct SdlWindows {
    /// Stores [`winit`] windows by window identifier.
    pub windows: HashMap<u32, sdl2::video::Window>,
    /// Maps entities to `winit` window identifiers.
    pub entity_to_sdl_window: EntityHashMap<u32>,
    /// Maps `winit` window identifiers to entities.
    pub sdl_window_to_entity: HashMap<u32, Entity>,
    // Many `winit` window functions (e.g. `set_window_icon`) can only be called on the main thread.
    // If they're called on other threads, the program might hang. This marker indicates that this
    // type is not thread-safe and will be `!Send` and `!Sync`.
    _not_send_sync: core::marker::PhantomData<*const ()>,
}
impl SdlWindows {
    pub const fn new() -> Self {
        Self {
            windows: HashMap::new(),
            entity_to_sdl_window: EntityHashMap::new(),
            sdl_window_to_entity: HashMap::new(),
            _not_send_sync: core::marker::PhantomData,
        }
    }
}

pub struct Sdl2WindowBackendPlugin;
impl Plugin for Sdl2WindowBackendPlugin {
    fn build(&self, app: &mut App) {
        let sdl_context = sdl2::init().expect("failed to init sdl");
        app.set_runner(|app| sdl_runner(app, sdl_context))
            .add_systems(Last, changed_bevy_windows);
    }
}

fn sdl_runner(mut app: App, sdl_context: Sdl) -> AppExit {
    if app.plugins_state() == PluginsState::Ready {
        app.finish();
        app.cleanup();
    }
    let mut video_subsystem = sdl_context
        .video()
        .expect("failed to init sdl video subsystem");

    let mut event_pump = sdl_context
        .event_pump()
        .expect("failed to get sdl event_pump");

    'running: loop {
        // Process new windows before checking sdl events
        let mut create_window =
            SystemState::<CreateWindowParams<Added<bevy_window::Window>>>::from_world(
                app.world_mut(),
            );
        create_windows(create_window.get_mut(app.world_mut()), &mut video_subsystem);
        create_window.apply(app.world_mut());

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                Event::Window {
                    window_id,
                    win_event,
                    ..
                } => {
                    SDL_WINDOWS.with_borrow(|windows| {
                        let entity = windows
                            .sdl_window_to_entity
                            .get(&window_id)
                            .expect("Window entity not found");
                        let mut window_event_state =
                            SystemState::<HandleSdlWindowEventParams>::from_world(app.world_mut());
                        handle_sdl_window_event(
                            window_event_state.get_mut(app.world_mut()),
                            *entity,
                            win_event,
                        );
                    });
                }
                _ => {
                    // dbg!(e);
                }
            }
        }

        app.update();
    }
    app.world_mut().clear_all();
    AppExit::Success
}

type CreateWindowParams<'w, 's, F = ()> = (
    Commands<'w, 's>,
    Query<
        'w,
        's,
        (
            Entity,
            &'static mut bevy_window::Window,
            &'static CursorOptions,
            Option<&'static RawHandleWrapperHolder>,
        ),
        F,
    >,
    EventWriter<'w, WindowCreated>,
);

fn create_windows<F: QueryFilter + 'static>(
    (mut commands, mut created_windows, mut window_created_events): SystemParamItem<
        CreateWindowParams<F>,
    >,
    video_subsystem: &mut VideoSubsystem,
) {
    SDL_WINDOWS.with_borrow_mut(|windows| {
        for (entity, window, cursor_options, maybe_handle_holder) in &mut created_windows {
            if windows.entity_to_sdl_window.get(&entity).is_some() {
                continue;
            }

            let mut window_builder = video_subsystem.window(&window.title, 1240, 720);
            if window.resizable {
                window_builder.resizable();
            }
            if !window.visible {
                window_builder.hidden();
            }
            if !window.decorations {
                window_builder.borderless();
            }
            match window.mode {
                bevy_window::WindowMode::Windowed => {}
                bevy_window::WindowMode::BorderlessFullscreen(..) => {
                    window_builder.borderless();
                    window_builder.fullscreen_desktop();
                }
                bevy_window::WindowMode::Fullscreen(..) => {
                    window_builder.fullscreen();
                }
            }

            #[cfg(target_os = "macos")]
            window_builder.metal_view();

            let mut sdl_window = window_builder
                .build()
                .map_err(|e| e.to_string())
                .expect("failed to build window");
            if window.transparent {
                sdl_window
                    .set_opacity(0.0)
                    .expect("Failed to set window opacity");
            }
            match cursor_options.grab_mode {
                bevy_window::CursorGrabMode::None => sdl_window.set_mouse_grab(false),
                bevy_window::CursorGrabMode::Confined | bevy_window::CursorGrabMode::Locked => {
                    sdl_window.set_mouse_grab(true);
                }
            }

            if let Some(handle_holder) = maybe_handle_holder {
                let handle_wrapper = unsafe {
                    std::mem::transmute::<FakeRawHandleWrapper, RawHandleWrapper>(
                        FakeRawHandleWrapper {
                            // I don't need the window reference stuff but RawHandleWrapper
                            // forces me to use it which creates a bunch of lifetime issues
                            // with SDL
                            _window: Arc::new(()),
                            window_handle: sdl_window.window_handle().unwrap().as_raw(),
                            display_handle: sdl_window.display_handle().unwrap().as_raw(),
                        },
                    )
                };

                commands.entity(entity).insert(handle_wrapper.clone());
                *handle_holder.0.lock().unwrap() = Some(handle_wrapper);
            }
            let sdl_window_id = sdl_window.id();
            windows.windows.insert(sdl_window_id, sdl_window);
            windows.entity_to_sdl_window.insert(entity, sdl_window_id);
            windows.sdl_window_to_entity.insert(sdl_window_id, entity);

            commands.entity(entity).insert(CachedWindow(window.clone()));

            window_created_events.write(WindowCreated { window: entity });
        }
    });
}

struct FakeRawHandleWrapper {
    _window: Arc<dyn Any + Send + Sync>,
    #[allow(unused)]
    window_handle: raw_window_handle::RawWindowHandle,
    #[allow(unused)]
    display_handle: raw_window_handle::RawDisplayHandle,
}

type HandleSdlWindowEventParams<'w, 's> = (
    Query<'w, 's, (&'static mut bevy_window::Window, &'static mut CachedWindow)>,
    EventWriter<'w, bevy_window::WindowResized>,
);
fn handle_sdl_window_event(
    (mut query, mut window_resized): SystemParamItem<HandleSdlWindowEventParams>,
    entity: Entity,
    win_event: sdl2::event::WindowEvent,
) {
    let (mut window, _) = query.get_mut(entity).expect("failed to get Window");
    match win_event {
        sdl2::event::WindowEvent::Resized(width, height)
        | sdl2::event::WindowEvent::SizeChanged(width, height) => {
            println!("resized {width} {height}");
            window
                .resolution
                .set_physical_resolution(width as u32, height as u32);

            window_resized.write(bevy_window::WindowResized {
                window: entity,
                width: window.width(),
                height: window.height(),
            });
        }
        sdl2::event::WindowEvent::Shown => {
            window.visible = true;
        }
        sdl2::event::WindowEvent::Hidden => {
            window.visible = false;
        }
        _ => {}
    }

    let (window, mut cached_window) = query.get_mut(entity).expect("failed to get Window");
    if window.is_changed() {
        *cached_window = CachedWindow(window.clone());
    }
}

/// The cached state of the window so we can check which properties were changed from within the app.
#[derive(Debug, Clone, Component)]
pub(crate) struct CachedWindow(bevy_window::Window);

fn changed_bevy_windows(
    mut changed_windows: Query<
        (Entity, &mut bevy_window::Window, &mut CachedWindow),
        Changed<bevy_window::Window>,
    >,
    _non_send_marker: NonSendMarker,
) {
    SDL_WINDOWS.with_borrow_mut(|windows| {
        for (entity, window, mut cache) in &mut changed_windows {
            let Some(sdl_window) = windows
                .entity_to_sdl_window
                .get(&entity)
                .and_then(|window_id| windows.windows.get_mut(window_id))
            else {
                continue;
            };
            if window.title != cache.0.title {
                sdl_window
                    .set_title(&window.title)
                    .expect("Failed to set window title");
            }
            if window.visible != cache.0.visible {
                if window.visible {
                    sdl_window.show();
                } else {
                    sdl_window.hide();
                }
            }
            if window.resizable != cache.0.resizable {
                sdl_window.set_resizable(window.resizable);
            }
            if window.resolution != cache.0.resolution {
                let mut physical_size = UVec2::new(
                    window.resolution.physical_width(),
                    window.resolution.physical_height(),
                );

                let cached_physical_size =
                    UVec2::new(cache.0.physical_width(), cache.0.physical_height());

                let base_scale_factor = window.resolution.base_scale_factor();

                // Note: this may be different from `winit`'s base scale factor if
                // `scale_factor_override` is set to Some(f32)
                let scale_factor = window.scale_factor();
                let cached_scale_factor = cache.0.scale_factor();

                // Check and update `winit`'s physical size only if the window is not maximized
                if scale_factor != cached_scale_factor && !sdl_window.is_maximized() {
                    let logical_size =
                        if let Some(cached_factor) = cache.0.resolution.scale_factor_override() {
                            physical_size.as_dvec2() / cached_factor as f64
                        } else {
                            physical_size.as_dvec2() / base_scale_factor as f64
                        };

                    // Scale factor changed, updating physical and logical size
                    if let Some(forced_factor) = window.resolution.scale_factor_override() {
                        // This window is overriding the OS-suggested DPI, so its physical size
                        // should be set based on the overriding value. Its logical size already
                        // incorporates any resize constraints.
                        physical_size = (logical_size * forced_factor as f64).as_uvec2();
                    } else {
                        physical_size = (logical_size * base_scale_factor as f64).as_uvec2();
                    }
                }

                if physical_size != cached_physical_size {
                    sdl_window
                        .set_size(physical_size.x, physical_size.y)
                        .expect("Failed to set window size");
                }
            }
            *cache = CachedWindow(window.clone());
        }
    });
}
