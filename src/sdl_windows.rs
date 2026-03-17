use std::sync::{Arc, Mutex};

use bevy_ecs::entity::{Entity, EntityHashMap};
use bevy_platform::collections::HashMap;
use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle,
};

/// Newtype over `sdl2::video::Window` that is `Send + Sync`.
///
/// `sdl2::video::Window` contains an `Rc<WindowContext>` which prevents it
/// from being sent across threads at the Rust type-system level. However,
/// SDL2's C-level window operations are internally thread-safe (SDL uses its
/// own mutexes). The `Rc` is purely a Rust-side refcount for the wrapper; no
/// SDL2 internals depend on it being single-threaded.
///
/// # Safety
///
/// The caller must ensure this value is not dropped while another thread is
/// using a raw handle or lock derived from it. In practice this is guaranteed
/// by wrapping it in `Arc<Mutex<SendSyncSdlWindow>>` — the `Arc` keeps the
/// window alive and the `Mutex` serialises access.
pub struct SendSyncSdlWindow(pub sdl2::video::Window);

// SAFETY: see type-level doc above.
unsafe impl Send for SendSyncSdlWindow {}
unsafe impl Sync for SendSyncSdlWindow {}

impl HasWindowHandle for SendSyncSdlWindow {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        self.0.window_handle()
    }
}

impl HasDisplayHandle for SendSyncSdlWindow {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        self.0.display_handle()
    }
}

#[derive(Default)]
pub struct SdlWindows {
    /// SDL windows keyed by SDL window ID.
    pub windows: HashMap<u32, Arc<Mutex<SendSyncSdlWindow>>>,
    /// Maps Bevy entity to SDL window ID.
    pub entity_to_sdl_window: EntityHashMap<u32>,
    /// Maps SDL window ID to Bevy entity.
    pub sdl_window_to_entity: HashMap<u32, Entity>,
    // Opt out of Send + Sync so this type is confined to one thread.
    _not_send_sync: core::marker::PhantomData<*const ()>,
}

impl SdlWindows {
    pub const fn new() -> Self {
        Self {
            windows: HashMap::new(),
            entity_to_sdl_window: EntityHashMap::new(),
            sdl_window_to_entity: HashMap::new(),
            _not_send_sync: core::marker::PhantomData,
        }
    }

    /// Get the SDL window associated with a Bevy entity.
    pub fn get_window(&self, entity: Entity) -> Option<&Arc<Mutex<SendSyncSdlWindow>>> {
        self.entity_to_sdl_window
            .get(&entity)
            .and_then(|id| self.windows.get(id))
    }

    /// Get the Bevy entity associated with an SDL window ID.
    pub fn get_window_entity(&self, sdl_window_id: u32) -> Option<Entity> {
        self.sdl_window_to_entity.get(&sdl_window_id).cloned()
    }
}
