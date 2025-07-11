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
use bevy_math::UVec2;
use converters::{convert_sdl_keycode, convert_sdl_scancode};
use create_windows::CreateWindowParams;
use create_windows::create_windows;
use event_handlers::{HandleSdlWindowEventParams, forward_bevy_events, handle_sdl_window_event};
use sdl_windows::SdlWindows;
use sdl2::{Sdl, event::Event};

mod converters;
mod create_windows;
mod event_handlers;
mod sdl_windows;

thread_local! {
    pub static SDL_WINDOWS: RefCell<SdlWindows> = const { RefCell::new(SdlWindows::new()) };
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
                            .get_window_entity(window_id)
                            .expect("Window entity not found");
                        let mut window_event_state =
                            SystemState::<HandleSdlWindowEventParams>::from_world(app.world_mut());
                        handle_sdl_window_event(
                            window_event_state.get_mut(app.world_mut()),
                            entity,
                            win_event,
                        );
                    });
                }
                Event::KeyDown {
                    timestamp: _,
                    window_id,
                    keycode,
                    scancode,
                    keymod: _,
                    repeat,
                } => {
                    SDL_WINDOWS.with_borrow(|windows| {
                        let entity = windows
                            .get_window_entity(window_id)
                            .expect("Window entity not found");
                        let Some(key_code) = scancode.and_then(convert_sdl_scancode) else {
                            return;
                        };
                        let Some(keycode) = keycode else { return };
                        bevy_window_events.push(bevy_window::WindowEvent::KeyboardInput(
                            bevy_input::keyboard::KeyboardInput {
                                key_code,
                                logical_key: convert_sdl_keycode(keycode),
                                state: bevy_input::ButtonState::Pressed,
                                text: None,
                                repeat,
                                window: entity,
                            },
                        ));
                    });
                }
                Event::KeyUp {
                    timestamp: _,
                    window_id,
                    keycode,
                    scancode,
                    keymod: _,
                    repeat,
                } => {
                    SDL_WINDOWS.with_borrow(|windows| {
                        let entity = windows
                            .get_window_entity(window_id)
                            .expect("Window entity not found");
                        let Some(key_code) = scancode.and_then(convert_sdl_scancode) else {
                            return;
                        };
                        let Some(keycode) = keycode else { return };
                        bevy_window_events.push(bevy_window::WindowEvent::KeyboardInput(
                            bevy_input::keyboard::KeyboardInput {
                                key_code,
                                logical_key: convert_sdl_keycode(keycode),
                                state: bevy_input::ButtonState::Released,
                                text: None,
                                repeat,
                                window: entity,
                            },
                        ));
                    });
                }
                _ => {
                    // dbg!(e);
                }
            }

            // Forward events
            forward_bevy_events(app.world_mut(), std::mem::take(&mut bevy_window_events));
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
