use std::sync::{Arc, Mutex};

use crate::{CachedWindow, SDL_WINDOWS, SendSyncSdlWindow};
use bevy_ecs::{
    entity::Entity,
    message::MessageWriter,
    query::QueryFilter,
    system::{Commands, Query, ResMut, SystemParamItem},
};
use bevy_window::{
    CursorOptions, RawHandleWrapper, RawHandleWrapperHolder, WindowCreated, WindowWrapper,
};
use crossbeam_channel::{Receiver, Sender};

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
    MessageWriter<'w, WindowCreated>,
);

pub fn create_windows<F: QueryFilter + 'static>(
    (mut commands, mut created_windows, mut window_created_events): SystemParamItem<
        CreateWindowParams<F>,
    >,
    sender: &Sender<(Entity, bevy_window::Window, CursorOptions)>,
    receiver: &Receiver<(u32, Arc<Mutex<SendSyncSdlWindow>>)>,
) {
    for (entity, window, cursor_options, maybe_handle_holder) in &mut created_windows {
        if SDL_WINDOWS.with_borrow(|windows| windows.entity_to_sdl_window.contains_key(&entity)) {
            continue;
        }

        // TODO send ready channel through this channel so we always have the right one
        let _ = sender.send((entity, window.clone(), cursor_options.clone()));

        let (sdl_window_id, sdl_window) = receiver.recv().expect("Failed to create SDL window");

        let raw_handle_wrapper = {
            let guard = sdl_window.lock().expect("SDL window mutex poisoned");
            RawHandleWrapper::new(&WindowWrapper::new(SendSyncSdlWindow(
                // Clone the inner sdl2::video::Window (it's Clone via Rc clone).
                guard.0.clone(),
            )))
            .expect("Failed to create raw handle wrapper")
        };

        if let Some(handle_holder) = maybe_handle_holder {
            commands.entity(entity).insert(raw_handle_wrapper.clone());
            *handle_holder.0.lock().unwrap() = Some(raw_handle_wrapper);
        }

        commands.entity(entity).insert(CachedWindow(window.clone()));

        window_created_events.write(WindowCreated { window: entity });
        bevy_log::info!("window created {entity}");

        SDL_WINDOWS.with_borrow_mut(|windows| {
            windows
                .windows
                .insert(sdl_window_id, Arc::clone(&sdl_window));
            windows.entity_to_sdl_window.insert(entity, sdl_window_id);
            windows.sdl_window_to_entity.insert(sdl_window_id, entity);
        });
    }
}
