use bevy::app::App;
use bevy::diagnostic::{FrameCount, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::winit::WinitPlugin;
use bevy::{DefaultPlugins, diagnostic::LogDiagnosticsPlugin};
use bevy_window::{CursorGrabMode, CursorOptions, PresentMode, Window, WindowPlugin};
use bevy_window_sdl2_backend::Sdl2WindowBackendPlugin;

fn main() {
    App::new()
        .add_plugins((
            // Add the plugin
            Sdl2WindowBackendPlugin,
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "window_settings SDL2".into(),
                        visible: false,
                        present_mode: PresentMode::AutoVsync,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                // Make sure to disable the WinitPlugin
                .disable::<WinitPlugin>(),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (make_visible, toggle_vsync, change_title, toggle_cursor),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3d::default());
}

fn make_visible(mut window: Single<&mut Window>, frames: Res<FrameCount>) {
    // The delay may be different for your app or system.
    if frames.0 == 3 {
        // At this point the gpu is ready to show the app so we can make the window visible.
        // Alternatively, you could toggle the visibility in Startup.
        // It will work, but it will have one white frame before it starts rendering
        window.visible = true;
    }
}

/// This system toggles the vsync mode when pressing the button V.
/// You'll see fps increase displayed in the console.
fn toggle_vsync(input: Res<ButtonInput<KeyCode>>, mut window: Single<&mut Window>) {
    if input.just_pressed(KeyCode::KeyV) {
        window.present_mode = if matches!(window.present_mode, PresentMode::AutoVsync) {
            PresentMode::AutoNoVsync
        } else {
            PresentMode::AutoVsync
        };
        info!("PRESENT_MODE: {:?}", window.present_mode);
    }
}

/// This system will then change the title during execution
fn change_title(mut window: Single<&mut Window>, time: Res<Time>) {
    window.title = format!(
        "Seconds since startup: {}",
        time.elapsed().as_secs_f32().round()
    );
}

fn toggle_cursor(mut cursor_options: Single<&mut CursorOptions>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        cursor_options.visible = !cursor_options.visible;
        cursor_options.grab_mode = match cursor_options.grab_mode {
            CursorGrabMode::None => CursorGrabMode::Locked,
            CursorGrabMode::Locked | CursorGrabMode::Confined => CursorGrabMode::None,
        };
    }
}
