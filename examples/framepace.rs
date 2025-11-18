use bevy::{color::palettes, prelude::*, winit::WinitPlugin};
use bevy_window::{CursorIcon, PresentMode, SystemCursorIcon, Window, WindowResolution};
use bevy_window_sdl2_backend::{Sdl2FrameLimiter, Sdl2WindowBackendPlugin};

fn main() {
    App::new()
        .add_plugins((
            // Add the plugin
            Sdl2WindowBackendPlugin,
            // Make sure to disable the WinitPlugin
            DefaultPlugins
                .build()
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "SDL2 framepace".into(),
                        present_mode: PresentMode::AutoVsync,
                        resolution: WindowResolution::new(1920, 1080)
                            .with_scale_factor_override(1.0),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .disable::<WinitPlugin>(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (toggle_plugin, update_ui, update_cursor))
        .run();
}

#[derive(Component)]
struct EnableText;

fn toggle_plugin(
    input: Res<ButtonInput<KeyCode>>,
    mut limiter: ResMut<Sdl2FrameLimiter>,
    mut window: Single<&mut Window>,
) {
    if input.just_pressed(KeyCode::Space) {
        limiter.enabled = !limiter.enabled;
    }

    if input.just_pressed(KeyCode::KeyT) {
        if window.present_mode == PresentMode::AutoVsync {
            window.present_mode = PresentMode::AutoNoVsync;
        } else {
            window.present_mode = PresentMode::AutoVsync;
        }
    }
}

fn update_ui(
    mut text: Single<&mut TextSpan, With<EnableText>>,
    limiter: Res<Sdl2FrameLimiter>,
    window: Single<&Window>,
) {
    text.0 = format!(
        "{} pres mode: {:?}",
        if limiter.enabled {
            format!("{}", limiter.target_framerate.unwrap())
        } else {
            "Unlimited".to_owned()
        },
        window.present_mode
    );
}

pub fn update_cursor(window: Single<&Window>, mut gizmos: Gizmos) {
    if let Some(pos) = window.cursor_position() {
        let pos = Vec2::new(pos.x - window.width() / 2.0, window.height() / 2.0 - pos.y);
        gizmos.circle_2d(pos, 10.0, palettes::basic::GREEN);
    }
}

/// set up the scene
fn setup(mut commands: Commands, window: Single<Entity, With<Window>>) {
    commands
        .entity(*window)
        .insert(CursorIcon::System(SystemCursorIcon::Crosshair));
    commands.spawn(Camera2d);

    // UI
    let text_font = TextFont {
        font_size: 42.,
        ..default()
    };
    commands
        .spawn(Text::default())
        .with_child((TextSpan::new("Frame limit: "), text_font.clone()))
        .with_child((TextSpan::new(""), text_font.clone(), EnableText))
        .with_child((
            TextSpan::new("\nPress space to toggle frame limiter"),
            text_font.clone(),
        ))
        .with_child((TextSpan::new("\nPress T to toggle vsync"), text_font));
}
