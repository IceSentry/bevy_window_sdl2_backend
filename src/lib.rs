#![allow(missing_docs, reason = "work in progress")]

use core::cell::RefCell;
use std::time::Duration;

use bevy_app::{App, AppExit, Last, Plugin, PluginsState, Update};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::{Added, Changed, With},
    resource::Resource,
    system::{NonSendMarker, Query, ResMut, SystemState},
    world::{FromWorld, Mut},
};
use bevy_math::UVec2;
use bevy_window::{CursorIcon, Window};
use create_windows::CreateWindowParams;
use create_windows::create_windows;
use sdl_windows::SdlWindows;
use sdl2::Sdl;
use window_event_handler::forward_bevy_window_events;

use crate::sdl2_event_handler::{HandleEventState, handle_sdl_event};

mod converters;
mod create_windows;
mod sdl2_event_handler;
mod sdl_windows;
mod window_event_handler;

thread_local! {
    pub static SDL_WINDOWS: RefCell<SdlWindows> = const { RefCell::new(SdlWindows::new()) };
}

pub struct Sdl2WindowBackendPlugin;
impl Plugin for Sdl2WindowBackendPlugin {
    fn build(&self, app: &mut App) {
        let sdl_context = sdl2::init().expect("failed to init sdl");
        app.set_runner(|app| sdl_runner(app, sdl_context))
            .add_systems(Last, set_cursor)
            .add_systems(Last, changed_bevy_windows);

        app.insert_resource(Sdl2FrameLimiter {
            enabled: false,
            render_target: std::time::Instant::now(),
            target_framerate: None,
            target_frame_time: None,
            display_resfresh_rate: 0,
        });

        app.add_systems(Update, get_display_refresh_rate);
    }
}

#[derive(Resource)]
pub struct Sdl2FrameLimiter {
    pub enabled: bool,
    pub render_target: std::time::Instant,
    pub target_framerate: Option<i32>,
    pub target_frame_time: Option<std::time::Duration>,
    pub display_resfresh_rate: i32,
}

impl Default for Sdl2FrameLimiter {
    fn default() -> Self {
        Self {
            enabled: false,
            render_target: std::time::Instant::now(),
            target_framerate: None,
            target_frame_time: None,
            display_resfresh_rate: 0,
        }
    }
}

impl Sdl2FrameLimiter {
    fn set_framerate(&mut self, target_framerate: i32) {
        self.target_framerate = Some(target_framerate);
        self.target_frame_time = Some(Duration::from_nanos(
            1_000_000_000 / target_framerate as u64,
        ));
    }

    fn set_display_refresh_rate(&mut self, display_resfresh_rate: i32) {
        self.display_resfresh_rate = display_resfresh_rate;
    }
}

fn get_display_refresh_rate(
    mut limiter: ResMut<Sdl2FrameLimiter>,
    windows: Query<Entity, With<Window>>,
    _marker: NonSendMarker,
) {
    let refresh_rate = windows
        .iter()
        .filter_map(|e| {
            SDL_WINDOWS.with_borrow(|windows| {
                let window = windows.get_window(e)?;
                let display_mode = window.display_mode().ok()?;
                Some(display_mode.refresh_rate)
            })
        })
        // TODO not sure if min makes sense here. I think we should look for the window that is
        // currently active and use the refresh_rate from that
        .min()
        .expect("Failed to find refresh rate");
    limiter.set_framerate(refresh_rate);
    limiter.set_display_refresh_rate(refresh_rate);
}

fn sdl_runner(mut app: App, sdl_context: Sdl) -> AppExit {
    if app.plugins_state() == PluginsState::Ready {
        app.finish();
        app.cleanup();
    }
    let mut video_subsystem = sdl_context
        .video()
        .expect("failed to init sdl video subsystem");

    let mut event_pump = sdl_context
        .event_pump()
        .expect("failed to get sdl event_pump");

    let mut bevy_window_events: Vec<bevy_window::WindowEvent> = vec![];
    'running: loop {
        // Process new windows before checking sdl events
        let mut create_window =
            SystemState::<CreateWindowParams<Added<bevy_window::Window>>>::from_world(
                app.world_mut(),
            );
        create_windows(create_window.get_mut(app.world_mut()), &mut video_subsystem);
        create_window.apply(app.world_mut());
        app.world_mut()
            .resource_scope(|_world, mut limiter: Mut<Sdl2FrameLimiter>| {
                if limiter.enabled
                    && limiter.render_target <= std::time::Instant::now()
                    && let Some(target_frame_time) = limiter.target_frame_time
                {
                    limiter.render_target += target_frame_time;
                }
            });

        for event in event_pump.poll_iter() {
            match handle_sdl_event(&mut app, event, &mut bevy_window_events) {
                HandleEventState::Exit => break 'running,
                HandleEventState::Continue => {}
            }

            // Forward events
            forward_bevy_window_events(app.world_mut(), std::mem::take(&mut bevy_window_events));
        }

        app.update();

        app.world_mut()
            .resource_scope(|_world, mut limiter: Mut<Sdl2FrameLimiter>| {
                if limiter.enabled {
                    let now = std::time::Instant::now();
                    if limiter.render_target <= now
                        && let Some(target_frame_time) = limiter.target_frame_time
                    {
                        limiter.render_target = now + target_frame_time;
                    }
                    spin_sleep::sleep_until(limiter.render_target);
                }
            });
    }
    app.world_mut().clear_all();
    AppExit::Success
}

/// The cached state of the window so we can check which properties were changed from within the app.
#[derive(Debug, Clone, Component)]
pub(crate) struct CachedWindow(bevy_window::Window);

fn changed_bevy_windows(
    mut changed_windows: Query<
        (Entity, &mut bevy_window::Window, &mut CachedWindow),
        Changed<bevy_window::Window>,
    >,
    _non_send_marker: NonSendMarker,
) {
    SDL_WINDOWS.with_borrow_mut(|windows| {
        for (entity, window, mut cache) in &mut changed_windows {
            let Some(sdl_window) = windows
                .entity_to_sdl_window
                .get(&entity)
                .and_then(|window_id| windows.windows.get_mut(window_id))
            else {
                continue;
            };
            if window.title != cache.0.title {
                sdl_window
                    .set_title(&window.title)
                    .expect("Failed to set window title");
            }
            if window.visible != cache.0.visible {
                if window.visible {
                    sdl_window.show();
                } else {
                    sdl_window.hide();
                }
            }
            if window.resizable != cache.0.resizable {
                sdl_window.set_resizable(window.resizable);
            }
            if window.resolution != cache.0.resolution {
                let mut physical_size = UVec2::new(
                    window.resolution.physical_width(),
                    window.resolution.physical_height(),
                );

                let cached_physical_size =
                    UVec2::new(cache.0.physical_width(), cache.0.physical_height());

                let base_scale_factor = window.resolution.base_scale_factor();

                // Note: this may be different from `winit`'s base scale factor if
                // `scale_factor_override` is set to Some(f32)
                let scale_factor = window.scale_factor();
                let cached_scale_factor = cache.0.scale_factor();

                // Check and update `winit`'s physical size only if the window is not maximized
                if scale_factor != cached_scale_factor && !sdl_window.is_maximized() {
                    let logical_size =
                        if let Some(cached_factor) = cache.0.resolution.scale_factor_override() {
                            physical_size.as_dvec2() / cached_factor as f64
                        } else {
                            physical_size.as_dvec2() / base_scale_factor as f64
                        };

                    // Scale factor changed, updating physical and logical size
                    if let Some(forced_factor) = window.resolution.scale_factor_override() {
                        // This window is overriding the OS-suggested DPI, so its physical size
                        // should be set based on the overriding value. Its logical size already
                        // incorporates any resize constraints.
                        physical_size = (logical_size * forced_factor as f64).as_uvec2();
                    } else {
                        physical_size = (logical_size * base_scale_factor as f64).as_uvec2();
                    }
                }

                if physical_size != cached_physical_size {
                    sdl_window
                        .set_size(physical_size.x, physical_size.y)
                        .expect("Failed to set window size");
                }
            }
            *cache = CachedWindow(window.clone());
        }
    });
}

thread_local! {
    static ACTIVE_CURSOR: RefCell<Option<sdl2::mouse::Cursor>> = RefCell::new(None);
}

fn set_cursor(
    q: Query<(Entity, &bevy_window::Window, &CursorIcon)>,
    has_changed: Query<(), Changed<CursorIcon>>,
    _marker: NonSendMarker,
) {
    for (entity, window, cursor_icon) in &q {
        // We only want to update the cursor on the window that has a cursor
        if window.physical_cursor_position().is_none() {
            continue;
        }
        ACTIVE_CURSOR.with_borrow_mut(|active_cursor| {
            let set_cursor = |active_cursor: &mut Option<sdl2::mouse::Cursor>,
                              cursor_icon: &CursorIcon| {
                let sdl_cursor = match cursor_icon {
                    CursorIcon::Custom(_custom_cursor) => {
                        bevy_log::warn_once!("Custom cursor icon are not supported");
                        return;
                    }
                    CursorIcon::System(system_cursor_icon) => {
                        if let Some(sys_cursor) = map_bevy_system_cursor_to_sdl(system_cursor_icon)
                        {
                            sdl2::mouse::Cursor::from_system(sys_cursor)
                        } else {
                            return;
                        }
                    }
                }
                .expect("Failed to create cursor");
                sdl_cursor.set();
                *active_cursor = Some(sdl_cursor);
            };
            if active_cursor.is_some() {
                if has_changed.get(entity).is_ok() {
                    set_cursor(active_cursor, cursor_icon)
                }
            } else {
                // This should only happen once on startup
                set_cursor(active_cursor, cursor_icon)
            }
        });
        return;
    }
}

fn map_bevy_system_cursor_to_sdl(
    system_cursor: &bevy_window::SystemCursorIcon,
) -> Option<sdl2::mouse::SystemCursor> {
    Some(match system_cursor {
        bevy_window::SystemCursorIcon::Default => sdl2::mouse::SystemCursor::Arrow,
        bevy_window::SystemCursorIcon::Crosshair => sdl2::mouse::SystemCursor::Crosshair,
        bevy_window::SystemCursorIcon::Text => sdl2::mouse::SystemCursor::IBeam,
        cursor_icon => {
            bevy_log::warn_once!("{cursor_icon:?} is not supported");
            return None;
        }
    })
}
