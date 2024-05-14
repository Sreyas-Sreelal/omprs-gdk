use omp_codegen::native;
use std::ffi::c_void;

use crate::{players::Player, types::vector::Vector3};

use super::Pickup;

native!(CreatePickup, model: isize, pickup_type: isize, position: Vector3, virtualWorld: isize, -> struct Pickup);
native!(AddStaticPickup, model: isize, pickup_type: isize, position: Vector3, virtualWorld: isize, -> struct Pickup);
native!(DestroyPickup, pickup: struct Pickup);
native!(IsPickupStreamedIn, pickup: struct Pickup, player: struct Player, -> bool);
native!(GetPickupPos, pickup: struct Pickup, pos: mut Vector3);
native!(GetPickupModel, pickup: struct Pickup, -> isize);
native!(GetPickupType, pickup: struct Pickup, -> isize);
native!(GetPickupVirtualWorld, pickup: struct Pickup, -> isize);
native!(SetPickupPos, pickup: struct Pickup, pos: Vector3, update: bool);
native!(SetPickupModel, pickup: struct Pickup, model: isize, update: bool);
native!(SetPickupType, pickup: struct Pickup, pickup_type: isize, update: bool);
native!(SetPickupVirtualWorld, pickup: struct Pickup, virtualworld: isize);
native!(ShowPickupForPlayer, pickup: struct Pickup,player: struct Player);
native!(HidePickupForPlayer, pickup: struct Pickup, player: struct Player);
native!(IsPickupHiddenForPlayer, pickup: struct Pickup, player: struct Player, -> bool);
native!(GetPickupID, pickup: struct Pickup, -> isize);
native!(GetPickupFromID, pickupid: isize, -> struct Pickup);

#[doc(hidden)]
pub fn load_functions() {
    load_function!(CreatePickup);
    load_function!(AddStaticPickup);
    load_function!(DestroyPickup);
    load_function!(IsPickupStreamedIn);
    load_function!(GetPickupPos);
    load_function!(GetPickupModel);
    load_function!(GetPickupType);
    load_function!(GetPickupVirtualWorld);
    load_function!(SetPickupPos);
    load_function!(SetPickupModel);
    load_function!(SetPickupType);
    load_function!(SetPickupVirtualWorld);
    load_function!(ShowPickupForPlayer);
    load_function!(HidePickupForPlayer);
    load_function!(IsPickupHiddenForPlayer);
    load_function!(GetPickupID);
    load_function!(GetPickupFromID);
}
