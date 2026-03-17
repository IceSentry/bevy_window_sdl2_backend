use core::cell::RefCell;

use bevy_ecs::{
    entity::Entity,
    query::Changed,
    system::{NonSendMarker, Query},
};
use bevy_window::{CursorIcon, Window};

thread_local! {
    static ACTIVE_CURSOR: RefCell<Option<sdl2::mouse::Cursor>> = RefCell::new(None);
}

pub(crate) fn set_cursor(
    q: Query<(Entity, &Window, &CursorIcon)>,
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
                    CursorIcon::System(system_cursor_icon) => {
                        if let Some(sys_cursor) = map_bevy_system_cursor_to_sdl(system_cursor_icon)
                        {
                            sdl2::mouse::Cursor::from_system(sys_cursor)
                        } else {
                            return;
                        }
                    }
                    #[allow(unreachable_patterns)]
                    _ => {
                        bevy_log::warn_once!("Custom cursor icon are not supported");
                        return;
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
