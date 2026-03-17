use std::time::Duration;

use bevy_app::prelude::*;
use bevy_ecs::{prelude::*, system::NonSendMarker};
use bevy_window::Window;

use crate::SDL_WINDOWS;

pub struct Sdl2FrameLimiterPlugin;

impl Plugin for Sdl2FrameLimiterPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.insert_resource(Sdl2FrameLimiter {
            enabled: false,
            render_target: std::time::Instant::now(),
            target_framerate: None,
            target_frame_time: None,
            display_resfresh_rate: 0,
        })
        .add_systems(Update, get_display_refresh_rate);
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
    if let Some(refresh_rate) = windows
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
    {
        limiter.set_framerate(refresh_rate);
        limiter.set_display_refresh_rate(refresh_rate);
    } else {
        bevy_log::warn!("Failed to get display refresh rate");
    }
}

pub(crate) fn framerate_limiter(app: &mut App) {
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
