use bevy_ecs::entity::{Entity, EntityHashMap};
use bevy_platform::collections::HashMap;

#[derive(Default)]
pub struct SdlWindows {
    /// Stores [`winit`] windows by window identifier.
    pub windows: HashMap<u32, sdl2::video::Window>,
    /// Maps entities to `winit` window identifiers.
    pub entity_to_sdl_window: EntityHashMap<u32>,
    /// Maps `winit` window identifiers to entities.
    pub sdl_window_to_entity: HashMap<u32, Entity>,
    // Many `winit` window functions (e.g. `set_window_icon`) can only be called on the main thread.
    // If they're called on other threads, the program might hang. This marker indicates that this
    // type is not thread-safe and will be `!Send` and `!Sync`.
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

    /// Get the winit window that is associated with our entity.
    pub fn get_window(&self, entity: Entity) -> Option<&sdl2::video::Window> {
        self.entity_to_sdl_window
            .get(&entity)
            .and_then(|winit_id| self.windows.get(winit_id))
    }

    /// Get the entity associated with the winit window id.
    ///
    /// This is mostly just an intermediary step between us and winit.
    pub fn get_window_entity(&self, sdl_window_id: u32) -> Option<Entity> {
        self.sdl_window_to_entity.get(&sdl_window_id).cloned()
    }
}
