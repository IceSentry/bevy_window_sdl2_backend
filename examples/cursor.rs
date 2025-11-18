use bevy::{prelude::*, winit::WinitPlugin};
use bevy_window::{CursorIcon, SystemCursorIcon, Window};
use bevy_window_sdl2_backend::Sdl2WindowBackendPlugin;

fn main() {
    App::new()
        .add_plugins((
            // Add the plugin
            Sdl2WindowBackendPlugin,
            // Make sure to disable the WinitPlugin
            DefaultPlugins.build().disable::<WinitPlugin>(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, toggle_plugin)
        .run();
}

#[derive(Component)]
struct CursorText;

fn toggle_plugin(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut text: Single<&mut TextSpan, With<CursorText>>,
    window: Single<(Entity, &CursorIcon), With<Window>>,
) {
    if input.just_pressed(KeyCode::Space) {
        let (entity, curr_icon) = *window;
        let new_icon = if let CursorIcon::System(sys_icon) = curr_icon
            && *sys_icon == SystemCursorIcon::Crosshair
        {
            CursorIcon::System(SystemCursorIcon::Default)
        } else {
            CursorIcon::System(SystemCursorIcon::Crosshair)
        };
        text.0 = format!("{:?}", new_icon.clone());
        commands.entity(entity).insert(new_icon);
    }
}

/// set up the scene
fn setup(mut commands: Commands, window: Single<Entity, With<Window>>) {
    let cursor_icon = CursorIcon::System(SystemCursorIcon::Crosshair);
    commands.entity(*window).insert(cursor_icon.clone());
    commands.spawn(Camera2d);

    // UI
    let text_font = TextFont {
        font_size: 50.,
        ..default()
    };
    commands
        .spawn(Text::default())
        .with_child((TextSpan::new("CursorIcon: "), text_font.clone()))
        .with_child((
            TextSpan::new(format!("{:?}", cursor_icon)),
            text_font.clone(),
            CursorText,
        ))
        .with_child((
            TextSpan::new("\nPress space to toggle icon"),
            text_font.clone(),
        ));
}
