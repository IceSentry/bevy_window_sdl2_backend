use crate::CachedWindow;
use bevy_ecs::{
    change_detection::DetectChanges,
    entity::Entity,
    event::{EventWriter, Events},
    system::{Query, SystemParamItem},
    world::World,
};
use bevy_log::warn;

pub type HandleSdlWindowEventParams<'w, 's> = (
    Query<'w, 's, (&'static mut bevy_window::Window, &'static mut CachedWindow)>,
    EventWriter<'w, bevy_window::WindowResized>,
    EventWriter<'w, bevy_window::WindowMoved>,
    EventWriter<'w, bevy_window::CursorEntered>,
    EventWriter<'w, bevy_window::CursorLeft>,
    EventWriter<'w, bevy_window::WindowFocused>,
    EventWriter<'w, bevy_window::WindowCloseRequested>,
);

pub fn handle_sdl_window_event(
    (
        mut query,
        mut window_resized,
        mut window_moved,
        mut cursor_entered,
        mut cursor_left,
        mut window_focused,
        mut window_close_requested,
    ): SystemParamItem<HandleSdlWindowEventParams>,
    entity: Entity,
    win_event: sdl2::event::WindowEvent,
) {
    use sdl2::event::WindowEvent as SdlWindowEvent;
    let (mut window, _) = query.get_mut(entity).expect("failed to get Window");
    match win_event {
        SdlWindowEvent::Resized(width, height) | SdlWindowEvent::SizeChanged(width, height) => {
            // WARN SDL sends these events twice for some reason, not sure if that's a problem
            window
                .resolution
                .set_physical_resolution(width as u32, height as u32);
            window_resized.write(bevy_window::WindowResized {
                window: entity,
                width: window.width(),
                height: window.height(),
            });
        }
        SdlWindowEvent::Moved(x, y) => {
            window_moved.write(bevy_window::WindowMoved {
                window: entity,
                position: bevy_math::IVec2::new(x, y),
            });
        }
        SdlWindowEvent::Shown => {
            window.visible = true;
        }
        SdlWindowEvent::Hidden => {
            window.visible = false;
        }
        SdlWindowEvent::Maximized => {
            window.set_maximized(true);
        }
        SdlWindowEvent::Minimized => {
            window.set_minimized(true);
        }
        SdlWindowEvent::Enter => {
            cursor_entered.write(bevy_window::CursorEntered { window: entity });
        }
        SdlWindowEvent::Leave => {
            window.set_cursor_position(None);
            cursor_left.write(bevy_window::CursorLeft { window: entity });
        }
        SdlWindowEvent::FocusGained => {
            window.focused = true;
            window_focused.write(bevy_window::WindowFocused {
                window: entity,
                focused: true,
            });
        }
        SdlWindowEvent::FocusLost => {
            window.focused = false;
            window_focused.write(bevy_window::WindowFocused {
                window: entity,
                focused: false,
            });
        }
        SdlWindowEvent::Close => {
            window_close_requested.write(bevy_window::WindowCloseRequested { window: entity });
        }
        event => {
            warn!("sdl WindowEvent {event:?} not handled");
        }
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
