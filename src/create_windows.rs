use crate::{CachedWindow, SDL_WINDOWS, sdl_windows::SendSyncSdlWindow};
use bevy_ecs::{
    entity::Entity,
    message::MessageWriter,
    query::QueryFilter,
    system::{Commands, Query, SystemParamItem},
};
use bevy_window::{
    CursorOptions, RawHandleWrapper, RawHandleWrapperHolder, WindowCreated, WindowWrapper,
};
use crossbeam_channel::Sender;
use sdl2::VideoSubsystem;

pub type WindowReady = (u32, SendSyncSdlWindow);

pub fn build_sdl_window(
    video_subsystem: &VideoSubsystem,
    window: &bevy_window::Window,
    cursor_options: &CursorOptions,
    ready_sender: Sender<WindowReady>,
) {
    let mut window_builder =
        video_subsystem.window(&window.title, window.width() as u32, window.height() as u32);
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
            .set_opacity(0.5)
            .expect("Failed to set window opacity");
    }
    match cursor_options.grab_mode {
        bevy_window::CursorGrabMode::None => sdl_window.set_mouse_grab(false),
        bevy_window::CursorGrabMode::Confined | bevy_window::CursorGrabMode::Locked => {
            sdl_window.set_mouse_grab(true);
        }
    }

    let sdl_window_id = sdl_window.id();
    let _ = ready_sender.send((sdl_window_id, SendSyncSdlWindow(sdl_window)));
}

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
    sender: &Sender<(
        Entity,
        bevy_window::Window,
        CursorOptions,
        Sender<WindowReady>,
    )>,
) {
    for (entity, window, cursor_options, maybe_handle_holder) in &mut created_windows {
        if SDL_WINDOWS.with_borrow(|windows| windows.entity_to_sdl_window.contains_key(&entity)) {
            continue;
        }

        let (ready_sender, ready_receiver) = crossbeam_channel::bounded(1);
        let _ = sender.send((entity, window.clone(), cursor_options.clone(), ready_sender));

        let (sdl_window_id, sdl_window) =
            ready_receiver.recv().expect("Failed to create SDL window");

        let raw_handle_wrapper = RawHandleWrapper::new(&WindowWrapper::new(sdl_window.clone()))
            .expect("Failed to create raw handle wrapper");

        if let Some(handle_holder) = maybe_handle_holder {
            commands.entity(entity).insert(raw_handle_wrapper.clone());
            *handle_holder.0.lock().unwrap() = Some(raw_handle_wrapper);
        }

        commands.entity(entity).insert(CachedWindow(window.clone()));

        window_created_events.write(WindowCreated { window: entity });
        bevy_log::info!("window created {entity}");

        SDL_WINDOWS.with_borrow_mut(|windows| {
            windows.windows.insert(sdl_window_id, sdl_window);
            windows.entity_to_sdl_window.insert(entity, sdl_window_id);
            windows.sdl_window_to_entity.insert(sdl_window_id, entity);
        });
    }
}
