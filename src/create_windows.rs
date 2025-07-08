use std::any::Any;
use std::sync::Arc;

use crate::{CachedWindow, SDL_WINDOWS};
use bevy_ecs::{
    entity::Entity,
    event::EventWriter,
    query::QueryFilter,
    system::{Commands, Query, SystemParamItem},
};
use bevy_window::{CursorOptions, RawHandleWrapper, RawHandleWrapperHolder, WindowCreated};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use sdl2::VideoSubsystem;

pub type CreateWindowParams<'w, 's, F = ()> = (
    Commands<'w, 's>,
    Query<
        'w,
        's,
        (
            Entity,
            &'static mut bevy_window::Window,
            &'static CursorOptions,
            Option<&'static RawHandleWrapperHolder>,
        ),
        F,
    >,
    EventWriter<'w, WindowCreated>,
);
pub fn create_windows<F: QueryFilter + 'static>(
    (mut commands, mut created_windows, mut window_created_events): SystemParamItem<
        CreateWindowParams<F>,
    >,
    video_subsystem: &mut VideoSubsystem,
) {
    SDL_WINDOWS.with_borrow_mut(|windows| {
        for (entity, window, cursor_options, maybe_handle_holder) in &mut created_windows {
            if windows.entity_to_sdl_window.get(&entity).is_some() {
                continue;
            }

            let mut window_builder = video_subsystem.window(&window.title, 1240, 720);
            if window.resizable {
                window_builder.resizable();
            }
            if !window.visible {
                window_builder.hidden();
            }
            if !window.decorations {
                window_builder.borderless();
            }
            match window.mode {
                bevy_window::WindowMode::Windowed => {}
                bevy_window::WindowMode::BorderlessFullscreen(..) => {
                    window_builder.borderless();
                    window_builder.fullscreen_desktop();
                }
                bevy_window::WindowMode::Fullscreen(..) => {
                    window_builder.fullscreen();
                }
            }

            #[cfg(target_os = "macos")]
            window_builder.metal_view();

            let mut sdl_window = window_builder
                .build()
                .map_err(|e| e.to_string())
                .expect("failed to build window");
            if window.transparent {
                sdl_window
                    .set_opacity(0.0)
                    .expect("Failed to set window opacity");
            }
            match cursor_options.grab_mode {
                bevy_window::CursorGrabMode::None => sdl_window.set_mouse_grab(false),
                bevy_window::CursorGrabMode::Confined | bevy_window::CursorGrabMode::Locked => {
                    sdl_window.set_mouse_grab(true);
                }
            }

            if let Some(handle_holder) = maybe_handle_holder {
                let handle_wrapper = unsafe {
                    std::mem::transmute::<FakeRawHandleWrapper, RawHandleWrapper>(
                        FakeRawHandleWrapper {
                            // I don't need the window reference stuff but RawHandleWrapper
                            // forces me to use it which creates a bunch of lifetime issues
                            // with SDL
                            _window: Arc::new(()),
                            window_handle: sdl_window.window_handle().unwrap().as_raw(),
                            display_handle: sdl_window.display_handle().unwrap().as_raw(),
                        },
                    )
                };

                commands.entity(entity).insert(handle_wrapper.clone());
                *handle_holder.0.lock().unwrap() = Some(handle_wrapper);
            }
            let sdl_window_id = sdl_window.id();
            windows.windows.insert(sdl_window_id, sdl_window);
            windows.entity_to_sdl_window.insert(entity, sdl_window_id);
            windows.sdl_window_to_entity.insert(sdl_window_id, entity);

            commands.entity(entity).insert(CachedWindow(window.clone()));

            window_created_events.write(WindowCreated { window: entity });
        }
    });
}

/// RawHandleWrapper has private fields that we need to set
/// This struct is a copy of it so we can create it and then transmute it
struct FakeRawHandleWrapper {
    _window: Arc<dyn Any + Send + Sync>,
    #[allow(unused)]
    window_handle: raw_window_handle::RawWindowHandle,
    #[allow(unused)]
    display_handle: raw_window_handle::RawDisplayHandle,
}
