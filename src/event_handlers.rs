use crate::CachedWindow;
use bevy_ecs::{
    change_detection::DetectChanges,
    entity::Entity,
    event::{EventWriter, Events},
    system::{Query, SystemParamItem},
    world::World,
};

pub type HandleSdlWindowEventParams<'w, 's> = (
    Query<'w, 's, (&'static mut bevy_window::Window, &'static mut CachedWindow)>,
    EventWriter<'w, bevy_window::WindowResized>,
);
pub fn handle_sdl_window_event(
    (mut query, mut window_resized): SystemParamItem<HandleSdlWindowEventParams>,
    entity: Entity,
    win_event: sdl2::event::WindowEvent,
) {
    let (mut window, _) = query.get_mut(entity).expect("failed to get Window");
    match win_event {
        sdl2::event::WindowEvent::Resized(width, height)
        | sdl2::event::WindowEvent::SizeChanged(width, height) => {
            println!("resized {width} {height}");
            window
                .resolution
                .set_physical_resolution(width as u32, height as u32);

            window_resized.write(bevy_window::WindowResized {
                window: entity,
                width: window.width(),
                height: window.height(),
            });
        }
        sdl2::event::WindowEvent::Shown => {
            window.visible = true;
        }
        sdl2::event::WindowEvent::Hidden => {
            window.visible = false;
        }
        _ => {}
    }

    let (window, mut cached_window) = query.get_mut(entity).expect("failed to get Window");
    if window.is_changed() {
        *cached_window = CachedWindow(window.clone());
    }
}

pub fn forward_bevy_events(world: &mut World, events: Vec<bevy_window::WindowEvent>) {
    use bevy_window::WindowEvent as BevyWindowEvent;
    for sdl_event in events.iter() {
        match sdl_event.clone() {
            BevyWindowEvent::AppLifecycle(e) => {
                world.write_event(e);
            }
            BevyWindowEvent::CursorEntered(e) => {
                world.write_event(e);
            }
            BevyWindowEvent::CursorLeft(e) => {
                world.write_event(e);
            }
            BevyWindowEvent::CursorMoved(e) => {
                world.write_event(e);
            }
            BevyWindowEvent::FileDragAndDrop(e) => {
                world.write_event(e);
            }
            BevyWindowEvent::Ime(e) => {
                world.write_event(e);
            }
            BevyWindowEvent::RequestRedraw(e) => {
                world.write_event(e);
            }
            BevyWindowEvent::WindowBackendScaleFactorChanged(e) => {
                world.write_event(e);
            }
            BevyWindowEvent::WindowCloseRequested(e) => {
                world.write_event(e);
            }
            BevyWindowEvent::WindowCreated(e) => {
                world.write_event(e);
            }
            BevyWindowEvent::WindowDestroyed(e) => {
                world.write_event(e);
            }
            BevyWindowEvent::WindowFocused(e) => {
                world.write_event(e);
            }
            BevyWindowEvent::WindowMoved(e) => {
                world.write_event(e);
            }
            BevyWindowEvent::WindowOccluded(e) => {
                world.write_event(e);
            }
            BevyWindowEvent::WindowResized(e) => {
                world.write_event(e);
            }
            BevyWindowEvent::WindowScaleFactorChanged(e) => {
                world.write_event(e);
            }
            BevyWindowEvent::WindowThemeChanged(e) => {
                world.write_event(e);
            }
            BevyWindowEvent::MouseButtonInput(e) => {
                world.write_event(e);
            }
            BevyWindowEvent::MouseMotion(e) => {
                world.write_event(e);
            }
            BevyWindowEvent::MouseWheel(e) => {
                world.write_event(e);
            }
            BevyWindowEvent::PinchGesture(e) => {
                world.write_event(e);
            }
            BevyWindowEvent::RotationGesture(e) => {
                world.write_event(e);
            }
            BevyWindowEvent::DoubleTapGesture(e) => {
                world.write_event(e);
            }
            BevyWindowEvent::PanGesture(e) => {
                world.write_event(e);
            }
            BevyWindowEvent::TouchInput(e) => {
                world.write_event(e);
            }
            BevyWindowEvent::KeyboardInput(e) => {
                world.write_event(e);
            }
            BevyWindowEvent::KeyboardFocusLost(e) => {
                world.write_event(e);
            }
        }
    }
    if !events.is_empty() {
        world
            .resource_mut::<Events<bevy_window::WindowEvent>>()
            .write_batch(events);
    }
}
