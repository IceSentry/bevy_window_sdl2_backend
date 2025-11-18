#![allow(missing_docs, reason = "work in progress")]

use core::cell::RefCell;

use bevy_app::{App, AppExit, Last, Plugin, PluginsState};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::{Added, Changed},
    system::{NonSendMarker, Query, SystemState},
    world::FromWorld,
};
use bevy_log::error;
use bevy_math::{DVec2, UVec2, Vec2};
use bevy_window::CursorIcon;
use converters::{convert_sdl_keycode, convert_sdl_scancode, convert_sdl_touch_event};
use create_windows::CreateWindowParams;
use create_windows::create_windows;
use sdl_windows::SdlWindows;
use sdl2::{Sdl, event::Event};
use window_event_handler::{
    HandleSdlWindowEventParams, forward_bevy_window_events, handle_sdl_window_event,
};

use crate::sdl2_event_handler::{HandleEventState, handle_sdl_event};

mod converters;
mod create_windows;
mod sdl2_event_handler;
mod sdl_windows;
mod window_event_handler;


thread_local! {
    pub static SDL_WINDOWS: RefCell<SdlWindows> = const { RefCell::new(SdlWindows::new()) };
}

pub struct Sdl2WindowBackendPlugin;
impl Plugin for Sdl2WindowBackendPlugin {
    fn build(&self, app: &mut App) {
        let sdl_context = sdl2::init().expect("failed to init sdl");
        app.set_runner(|app| sdl_runner(app, sdl_context))
            .add_systems(Last, set_cursor)
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

    let mut bevy_window_events: Vec<bevy_window::WindowEvent> = vec![];
    'running: loop {
        // Process new windows before checking sdl events
        let mut create_window =
            SystemState::<CreateWindowParams<Added<bevy_window::Window>>>::from_world(
                app.world_mut(),
            );
        create_windows(create_window.get_mut(app.world_mut()), &mut video_subsystem);
        create_window.apply(app.world_mut());

        for event in event_pump.poll_iter() {
            match handle_sdl_event(&mut app, event, &mut bevy_window_events) {
                HandleEventState::Exit => break 'running,
                HandleEventState::Continue => {}
            }

            // Forward events
            forward_bevy_window_events(app.world_mut(), std::mem::take(&mut bevy_window_events));
        }

        app.update();
    }
    app.world_mut().clear_all();
    AppExit::Success
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

thread_local! {
    static ACTIVE_CURSOR: RefCell<Option<sdl2::mouse::Cursor>> = RefCell::new(None);
}

fn set_cursor(
    q: Query<(Entity, &bevy_window::Window, &CursorIcon)>,
    has_changed: Query<(), Changed<CursorIcon>>,
    _marker: NonSendMarker,
) {
    for (entity, window, cursor_icon) in &q {
        // We only want to update the cursor on the window that has a cursor
        if window.physical_cursor_position().is_none() {
            continue;
        }
        ACTIVE_CURSOR.with_borrow_mut(|active_cursor| {
            let set_cursor = |active_cursor: &mut Option<sdl2::mouse::Cursor>,
                              cursor_icon: &CursorIcon| {
                let sdl_cursor = match cursor_icon {
                    CursorIcon::Custom(_custom_cursor) => {
                        bevy_log::warn_once!("Custom cursor icon are not supported");
                        return;
                    }
                    CursorIcon::System(system_cursor_icon) => {
                        if let Some(sys_cursor) = map_bevy_system_cursor_to_sdl(system_cursor_icon)
                        {
                            sdl2::mouse::Cursor::from_system(sys_cursor)
                        } else {
                            return;
                        }
                    }
                }
                .expect("Failed to create cursor");
                sdl_cursor.set();
                *active_cursor = Some(sdl_cursor);
            };
            if active_cursor.is_some() {
                if has_changed.get(entity).is_ok() {
                    set_cursor(active_cursor, cursor_icon)
                }
            } else {
                // This should only happen once on startup
                set_cursor(active_cursor, cursor_icon)
            }
        });
        return;
    }
}

fn map_bevy_system_cursor_to_sdl(
    system_cursor: &bevy_window::SystemCursorIcon,
) -> Option<sdl2::mouse::SystemCursor> {
    Some(match system_cursor {
        bevy_window::SystemCursorIcon::Default => sdl2::mouse::SystemCursor::Arrow,
        bevy_window::SystemCursorIcon::Crosshair => sdl2::mouse::SystemCursor::Crosshair,
        bevy_window::SystemCursorIcon::Text => sdl2::mouse::SystemCursor::IBeam,
        cursor_icon => {
            bevy_log::warn_once!("{cursor_icon:?} is not supported");
            return None;
        }
    })
}
