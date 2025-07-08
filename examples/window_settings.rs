use bevy::DefaultPlugins;
use bevy::app::App;
use bevy::diagnostic::FrameCount;
use bevy::prelude::*;
use bevy::winit::WinitPlugin;
use bevy_window::{Window, WindowPlugin};
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
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                // Make sure to disable the WinitPlugin
                .disable::<WinitPlugin>(),
        ))
        .add_systems(Update, (make_visible, change_title))
        .run();
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

/// This system will then change the title during execution
fn change_title(mut window: Single<&mut Window>, time: Res<Time>) {
    window.title = format!(
        "Seconds since startup: {}",
        time.elapsed().as_secs_f32().round()
    );
}
