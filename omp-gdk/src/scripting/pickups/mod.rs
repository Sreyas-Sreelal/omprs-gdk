use std::ffi::c_void;

pub mod events;
pub mod functions;

pub use functions::load_functions;

use crate::{players::Player, types::vector::Vector3};

pub struct Pickup {
    handle: *const c_void,
}

#[deny(non_snake_case)]
impl Pickup {
    pub fn get_handle(&self) -> *const c_void {
        self.handle
    }

    pub fn new(handle: *const c_void) -> Self {
        Self { handle }
    }

    /// This function does exactly the same as AddStaticPickup, except it returns a pickup ID which can be used to destroy it afterwards and be tracked using OnPlayerPickUpPickup.
    pub fn create(
        model: i32,
        pickup_type: i32,
        position: Vector3,
        virtual_world: i32,
    ) -> Option<Pickup> {
        let mut _id = -1;
        functions::Pickup_Create(
            model,
            pickup_type,
            position.x,
            position.y,
            position.z,
            virtual_world,
            &mut _id,
        )
    }

    /// This function adds a 'static' pickup to the game.
    pub fn create_static(
        model: i32,
        pickup_type: i32,
        position: Vector3,
        virtual_world: i32,
    ) -> bool {
        functions::Pickup_AddStatic(
            model,
            pickup_type,
            position.x,
            position.y,
            position.z,
            virtual_world,
        )
    }

    /// Destroys a pickup created with CreatePickup.
    pub fn destroy(&self) -> bool {
        functions::Pickup_Destroy(self)
    }

    /// Checks if a pickup is streamed in for a specific player.
    pub fn is_streamed_in(&self, player: &Player) -> bool {
        functions::Pickup_IsStreamedIn(player, self)
    }

    /// Gets the coordinates of a pickup.
    pub fn get_pos(&self) -> Vector3 {
        let mut pos = Vector3::default();
        functions::Pickup_GetPos(self, &mut pos.x, &mut pos.y, &mut pos.z);
        pos
    }

    /// Gets the model ID of a pickup.
    pub fn get_model(&self) -> i32 {
        functions::Pickup_GetModel(self)
    }

    /// Gets the type of a pickup.
    pub fn get_type(&self) -> i32 {
        functions::Pickup_GetType(self)
    }

    /// Gets the virtual world ID of a pickup.
    pub fn get_virtual_world(&self) -> i32 {
        functions::Pickup_GetVirtualWorld(self)
    }

    /// Sets the position of a pickup.
    pub fn set_pos(&self, pos: Vector3, update: bool) -> bool {
        functions::Pickup_SetPos(self, pos.x, pos.y, pos.z, update)
    }

    /// Sets the model of a pickup.
    pub fn set_model(&self, model: i32, update: bool) -> bool {
        functions::Pickup_SetModel(self, model, update)
    }

    /// Sets the type of a pickup.
    pub fn set_type(&self, pickup_type: i32, update: bool) -> bool {
        functions::Pickup_SetType(self, pickup_type, update)
    }

    /// Sets the virtual world ID of a pickup.
    pub fn set_virtual_world(&self, virtualworld: i32) -> bool {
        functions::Pickup_SetVirtualWorld(self, virtualworld)
    }

    /// Shows a pickup for a specific player.
    pub fn show_for_player(&self, player: &Player) -> bool {
        functions::Pickup_ShowForPlayer(player, self)
    }

    /// Hides a pickup for a specific player.
    pub fn hide_for_player(&self, player: &Player) -> bool {
        functions::Pickup_HideForPlayer(player, self)
    }

    /// Checks if a pickup is hidden for a specific player.
    pub fn is_hidden_for_player(&self, player: &Player) -> bool {
        functions::Pickup_IsHiddenForPlayer(player, self)
    }

    /// Get id of the pickup
    pub fn get_id(&self) -> i32 {
        functions::Pickup_GetID(self)
    }

    /// Get a pickup object from an id
    pub fn get_from_id(pickupid: i32) -> Option<Pickup> {
        functions::Pickup_FromID(pickupid)
    }
}
