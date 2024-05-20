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
        model: isize,
        pickup_type: isize,
        position: Vector3,
        virtual_world: isize,
    ) -> Option<Pickup> {
        functions::CreatePickup(model, pickup_type, position, virtual_world)
    }

    /// This function adds a 'static' pickup to the game.
    pub fn create_static(
        model: isize,
        pickup_type: isize,
        position: Vector3,
        virtual_world: isize,
    ) -> Option<Pickup> {
        functions::AddStaticPickup(model, pickup_type, position, virtual_world)
    }

    /// Destroys a pickup created with CreatePickup.
    pub fn destroy(&self) {
        functions::DestroyPickup(self)
    }

    /// Checks if a pickup is streamed in for a specific player.
    pub fn is_streamed_in(&self, player: &Player) -> bool {
        functions::IsPickupStreamedIn(self, player)
    }

    /// Gets the coordinates of a pickup.
    pub fn get_pos(&self) -> Vector3 {
        let mut pos = Vector3::default();
        functions::GetPickupPos(self, &mut pos);
        pos
    }

    /// Gets the model ID of a pickup.
    pub fn get_model(&self) -> isize {
        functions::GetPickupModel(self)
    }

    /// Gets the type of a pickup.
    pub fn get_type(&self) -> isize {
        functions::GetPickupType(self)
    }

    /// Gets the virtual world ID of a pickup.
    pub fn get_virtual_world(&self) -> isize {
        functions::GetPickupVirtualWorld(self)
    }

    /// Sets the position of a pickup.
    pub fn set_pos(&self, pos: Vector3, update: bool) {
        functions::SetPickupPos(self, pos, update)
    }

    /// Sets the model of a pickup.
    pub fn set_model(&self, model: isize, update: bool) {
        functions::SetPickupModel(self, model, update)
    }

    /// Sets the type of a pickup.
    pub fn set_type(&self, pickup_type: isize, update: bool) {
        functions::SetPickupType(self, pickup_type, update)
    }

    /// Sets the virtual world ID of a pickup.
    pub fn set_virtual_world(&self, virtualworld: isize) {
        functions::SetPickupVirtualWorld(self, virtualworld)
    }

    /// Shows a pickup for a specific player.
    pub fn show_for_player(&self, player: &Player) {
        functions::ShowPickupForPlayer(self, player)
    }

    /// Hides a pickup for a specific player.
    pub fn hide_for_player(&self, player: &Player) {
        functions::HidePickupForPlayer(self, player)
    }

    /// Checks if a pickup is hidden for a specific player.
    pub fn is_hidden_for_player(&self, player: &Player) -> bool {
        functions::IsPickupHiddenForPlayer(self, player)
    }

    /// Get id of the pickup
    pub fn get_id(&self) -> isize {
        functions::GetPickupID(self)
    }

    /// Get a pickup object from an id
    pub fn get_from_id(pickupid: isize) -> Option<Pickup> {
        functions::GetPickupFromID(pickupid)
    }
}
