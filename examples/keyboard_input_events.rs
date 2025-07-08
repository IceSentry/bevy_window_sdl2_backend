//! Prints out all keyboard events.

use bevy::{input::keyboard::KeyboardInput, prelude::*, winit::WinitPlugin};
use bevy_window_sdl2_backend::Sdl2WindowBackendPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.build().disable::<WinitPlugin>(),
            Sdl2WindowBackendPlugin,
        ))
        .add_systems(Update, print_keyboard_event_system)
        .run();
}

/// This system prints out all keyboard events as they come in
fn print_keyboard_event_system(mut keyboard_input_events: EventReader<KeyboardInput>) {
    for event in keyboard_input_events.read() {
        info!("{:?}", event);
    }
}
