use bevy::DefaultPlugins;
use bevy::app::App;
use bevy::app::PluginGroup;
use bevy::prelude::*;
use bevy::winit::WinitPlugin;
use bevy_window::PresentMode;
use bevy_window::WindowResolution;
use bevy_window_sdl2_backend::Sdl2WindowBackendPlugin;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            // Add the plugin
            Sdl2WindowBackendPlugin,
            // Make sure to disable the WinitPlugin
            DefaultPlugins
                .build()
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "SDL2 3d_scene".into(),
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
        .add_systems(Update, rotate_cube)
        .run()
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Rotate,
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

#[derive(Component)]
struct Rotate;

fn rotate_cube(mut query: Query<&mut Transform, With<Rotate>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs());
    }
}
