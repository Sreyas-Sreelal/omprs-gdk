use std::ffi::c_void;

pub mod events;
pub mod functions;

pub use functions::load_functions;

use crate::{players::Player, vector::Vector3};

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

    pub fn create(
        model: isize,
        pickup_type: isize,
        position: Vector3,
        virtual_world: isize,
    ) -> Option<Pickup> {
        functions::CreatePickup(model, pickup_type, position, virtual_world)
    }

    pub fn create_static(
        model: isize,
        pickup_type: isize,
        position: Vector3,
        virtual_world: isize,
    ) -> Option<Pickup> {
        functions::AddStaticPickup(model, pickup_type, position, virtual_world)
    }

    pub fn destroy(&self) {
        functions::DestroyPickup(self)
    }

    pub fn is_streamed_in(&self, player: &Player) -> bool {
        functions::IsPickupStreamedIn(self, player)
    }

    pub fn get_pos(&self) -> Vector3 {
        let mut pos = Vector3::default();
        functions::GetPickupPos(self, &mut pos);
        pos
    }

    pub fn get_model(&self) -> isize {
        functions::GetPickupModel(self)
    }

    pub fn get_type(&self) -> isize {
        functions::GetPickupType(self)
    }

    pub fn get_virtual_world(&self) -> isize {
        functions::GetPickupVirtualWorld(self)
    }

    pub fn set_pos(&self, pos: Vector3, update: bool) {
        functions::SetPickupPos(self, pos, update)
    }

    pub fn set_model(&self, model: isize, update: bool) {
        functions::SetPickupModel(self, model, update)
    }

    pub fn set_type(&self, pickup_type: isize, update: bool) {
        functions::SetPickupType(self, pickup_type, update)
    }

    pub fn set_virtual_world(&self, virtualworld: isize) {
        functions::SetPickupVirtualWorld(self, virtualworld)
    }

    pub fn show_for_player(&self, player: &Player) {
        functions::ShowPickupForPlayer(self, player)
    }

    pub fn hide_for_player(&self, player: &Player) {
        functions::HidePickupForPlayer(self, player)
    }

    pub fn is_hidden_for_player(&self, player: &Player) -> bool {
        functions::IsPickupHiddenForPlayer(self, player)
    }

    pub fn get_id(&self) -> isize {
        functions::GetPickupID(self)
    }

    pub fn get_from_id(pickupid: isize) -> Option<Pickup> {
        functions::GetPickupFromID(pickupid)
    }
}
