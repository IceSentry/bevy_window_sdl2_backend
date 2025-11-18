use crate::converters::{convert_sdl_keycode, convert_sdl_scancode, convert_sdl_touch_event};
use crate::window_event_handler::{HandleSdlWindowEventParams, handle_sdl_window_event};
use crate::{SDL_WINDOWS, converters::convert_sdl_mouse_btn};
use bevy_app::App;
use bevy_ecs::{system::SystemState, world::FromWorld};
use bevy_log::error;
use bevy_math::{DVec2, Vec2};
use sdl2::event::Event;

pub(crate) enum HandleEventState {
    Continue,
    Exit,
}

pub(crate) fn handle_sdl_event(
    app: &mut App,
    event: Event,
    bevy_window_events: &mut Vec<bevy_window::WindowEvent>,
) -> HandleEventState {
    match event {
        Event::Quit { .. } => {
            return HandleEventState::Exit;
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
                let Some(button) = convert_sdl_mouse_btn(mouse_btn) else {
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
                let Some(button) = convert_sdl_mouse_btn(mouse_btn) else {
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
                let position = (physical_position / win.resolution.scale_factor() as f64).as_vec2();
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
                bevy_window_events.push(bevy_window::WindowEvent::Ime(bevy_window::Ime::Commit {
                    window: entity,
                    value: text.clone(),
                }));
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
                bevy_window_events.push(bevy_window::WindowEvent::Ime(bevy_window::Ime::Preedit {
                    window: entity,
                    value: text.clone(),
                    cursor: Some((start as usize, (start + length) as usize)),
                }));
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
    HandleEventState::Continue
}
