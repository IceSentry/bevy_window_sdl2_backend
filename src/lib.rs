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
                    window_id,
                    keycode,
                    scancode,
                    repeat,
                    ..
                } => {
                    SDL_WINDOWS.with_borrow(|windows| {
                        let entity = windows
                            .get_window_entity(window_id)
                            .expect("Window entity not found");
                        let Some(key_code) = scancode.and_then(convert_sdl_scancode) else {
                            return;
                        };
                        let Some(logical_key) = keycode.and_then(convert_sdl_keycode) else {
                            return;
                        };
                        bevy_window_events.push(bevy_window::WindowEvent::KeyboardInput(
                            bevy_input::keyboard::KeyboardInput {
                                key_code,
                                logical_key,
                                state: bevy_input::ButtonState::Pressed,
                                text: None,
                                repeat,
                                window: entity,
                            },
                        ));
                    });
                }
                Event::KeyUp {
                    window_id,
                    keycode,
                    scancode,
                    repeat,
                    ..
                } => {
                    SDL_WINDOWS.with_borrow(|windows| {
                        let entity = windows
                            .get_window_entity(window_id)
                            .expect("Window entity not found");
                        let Some(key_code) = scancode.and_then(convert_sdl_scancode) else {
                            return;
                        };
                        let Some(logical_key) = keycode.and_then(convert_sdl_keycode) else {
                            return;
                        };
                        bevy_window_events.push(bevy_window::WindowEvent::KeyboardInput(
                            bevy_input::keyboard::KeyboardInput {
                                key_code,
                                logical_key,
                                state: bevy_input::ButtonState::Released,
                                text: None,
                                repeat,
                                window: entity,
                            },
                        ));
                    });
                }
                Event::MouseButtonDown {
                    window_id,
                    mouse_btn,
                    ..
                } => {
                    SDL_WINDOWS.with_borrow(|windows| {
                        let entity = windows
                            .get_window_entity(window_id)
                            .expect("Window entity not found");
                        let Some(button) = converters::convert_sdl_mouse_btn(mouse_btn) else {
                            error!("Unknown mouse button: {:?}", mouse_btn);
                            return;
                        };
                        bevy_window_events.push(bevy_window::WindowEvent::MouseButtonInput(
                            bevy_input::mouse::MouseButtonInput {
                                button,
                                state: bevy_input::ButtonState::Pressed,
                                window: entity,
                            },
                        ));
                    });
                }
                Event::MouseButtonUp {
                    window_id,
                    mouse_btn,
                    ..
                } => {
                    SDL_WINDOWS.with_borrow(|windows| {
                        let entity = windows
                            .get_window_entity(window_id)
                            .expect("Window entity not found");
                        let Some(button) = converters::convert_sdl_mouse_btn(mouse_btn) else {
                            error!("Unknown mouse button: {:?}", mouse_btn);
                            return;
                        };
                        bevy_window_events.push(bevy_window::WindowEvent::MouseButtonInput(
                            bevy_input::mouse::MouseButtonInput {
                                button,
                                state: bevy_input::ButtonState::Released,
                                window: entity,
                            },
                        ));
                    });
                }
                Event::MouseMotion {
                    window_id,
                    x,
                    y,
                    xrel,
                    yrel,
                    ..
                } => {
                    bevy_window_events.push(bevy_window::WindowEvent::MouseMotion(
                        bevy_input::mouse::MouseMotion {
                            delta: Vec2::new(xrel as f32, yrel as f32),
                        },
                    ));
                    SDL_WINDOWS.with_borrow(|windows| {
                        let entity = windows
                            .get_window_entity(window_id)
                            .expect("Window entity not found");
                        let mut win = app
                            .world_mut()
                            .get_mut::<bevy_window::Window>(entity)
                            .expect("Failed to get window");
                        let physical_position = DVec2::new(x as f64, y as f64);

                        let last_position = win.physical_cursor_position();
                        let delta = last_position.map(|last_pos| {
                            (physical_position.as_vec2() - last_pos) / win.resolution.scale_factor()
                        });

                        win.set_physical_cursor_position(Some(physical_position));
                        let position =
                            (physical_position / win.resolution.scale_factor() as f64).as_vec2();
                        bevy_window_events.push(bevy_window::WindowEvent::CursorMoved(
                            bevy_window::CursorMoved {
                                delta,
                                window: entity,
                                position,
                            },
                        ));
                    });
                }
                Event::MouseWheel {
                    window_id,
                    precise_x,
                    precise_y,
                    ..
                } => {
                    SDL_WINDOWS.with_borrow(|windows| {
                        let entity = windows
                            .get_window_entity(window_id)
                            .expect("Window entity not found");
                        bevy_window_events.push(bevy_window::WindowEvent::MouseWheel(
                            bevy_input::mouse::MouseWheel {
                                unit: bevy_input::mouse::MouseScrollUnit::Line,
                                x: precise_x,
                                y: precise_y,
                                window: entity,
                            },
                        ));
                    });
                }
                Event::TextInput {
                    window_id,
                    ref text,
                    ..
                } => {
                    SDL_WINDOWS.with_borrow(|windows| {
                        let Some(entity) = windows.get_window_entity(window_id) else {
                            return;
                        };
                        bevy_window_events.push(bevy_window::WindowEvent::Ime(
                            bevy_window::Ime::Commit {
                                window: entity,
                                value: text.clone(),
                            },
                        ));
                    });
                }
                Event::TextEditing {
                    window_id,
                    ref text,
                    start,
                    length,
                    ..
                } => {
                    SDL_WINDOWS.with_borrow(|windows| {
                        let Some(entity) = windows.get_window_entity(window_id) else {
                            return;
                        };
                        bevy_window_events.push(bevy_window::WindowEvent::Ime(
                            bevy_window::Ime::Preedit {
                                window: entity,
                                value: text.clone(),
                                cursor: Some((start as usize, (start + length) as usize)),
                            },
                        ));
                    });
                }
                Event::DropFile {
                    window_id,
                    ref filename,
                    ..
                } => {
                    SDL_WINDOWS.with_borrow(|windows| {
                        let Some(entity) = windows.get_window_entity(window_id) else {
                            return;
                        };
                        bevy_window_events.push(bevy_window::WindowEvent::FileDragAndDrop(
                            bevy_window::FileDragAndDrop::DroppedFile {
                                window: entity,
                                path_buf: std::path::PathBuf::from(filename),
                            },
                        ));
                    });
                }
                Event::FingerDown {
                    finger_id,
                    x,
                    y,
                    pressure,
                    ..
                } => {
                    SDL_WINDOWS.with_borrow(|windows| {
                        let Some(entity) = windows.get_window_entity(0) else {
                            return;
                        };
                        bevy_window_events.push(bevy_window::WindowEvent::TouchInput(
                            convert_sdl_touch_event(
                                bevy_input::touch::TouchPhase::Started,
                                finger_id,
                                x,
                                y,
                                pressure,
                                entity,
                            ),
                        ));
                    });
                }
                Event::FingerUp {
                    finger_id,
                    x,
                    y,
                    pressure,
                    ..
                } => {
                    SDL_WINDOWS.with_borrow(|windows| {
                        let Some(entity) = windows.get_window_entity(0) else {
                            return;
                        };
                        bevy_window_events.push(bevy_window::WindowEvent::TouchInput(
                            convert_sdl_touch_event(
                                bevy_input::touch::TouchPhase::Ended,
                                finger_id,
                                x,
                                y,
                                pressure,
                                entity,
                            ),
                        ));
                    });
                }
                Event::FingerMotion {
                    finger_id,
                    x,
                    y,
                    pressure,
                    ..
                } => {
                    SDL_WINDOWS.with_borrow(|windows| {
                        let Some(entity) = windows.get_window_entity(0) else {
                            return;
                        };
                        bevy_window_events.push(bevy_window::WindowEvent::TouchInput(
                            convert_sdl_touch_event(
                                bevy_input::touch::TouchPhase::Moved,
                                finger_id,
                                x,
                                y,
                                pressure,
                                entity,
                            ),
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
