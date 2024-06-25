use omp_codegen::native;
use std::ffi::c_void;

use crate::players::Player;

use super::Pickup;

native!(Pickup_Create, model: i32, pickup_type: i32, x: f32, y: f32, z: f32, virtualWorld: i32, id: mut i32, -> struct Pickup);
native!(Pickup_AddStatic, model: i32, pickup_type: i32, x: f32, y: f32, z: f32, virtualWorld: i32, -> bool);
native!(Pickup_Destroy, pickup: struct Pickup, -> bool);
native!(Pickup_FromID, pickupid: i32, -> struct Pickup);
native!(Pickup_GetID, pickup: struct Pickup, -> i32);
native!(Pickup_IsValid, pickup: struct Pickup, -> bool);
native!(Pickup_IsStreamedIn, player: struct Player, pickup: struct Pickup, -> bool);
native!(Pickup_GetPos, pickup: struct Pickup, x: mut f32, y: mut f32, z: mut f32, -> bool);
native!(Pickup_GetModel, pickup: struct Pickup, -> i32);
native!(Pickup_GetType, pickup: struct Pickup, -> i32);
native!(Pickup_GetVirtualWorld, pickup: struct Pickup, -> i32);
native!(Pickup_SetPos, pickup: struct Pickup, x: f32, y: f32, z: f32, update: bool, -> bool);
native!(Pickup_SetModel, pickup: struct Pickup, model: i32, update: bool, -> bool);
native!(Pickup_SetType, pickup: struct Pickup, pickup_type: i32, update: bool, -> bool);
native!(Pickup_SetVirtualWorld, pickup: struct Pickup, virtualworld: i32, -> bool);
native!(Pickup_ShowForPlayer, player: struct Player, pickup: struct Pickup, -> bool);
native!(Pickup_HideForPlayer, player: struct Player, pickup: struct Pickup, -> bool);
native!(Pickup_IsHiddenForPlayer, player: struct Player, pickup: struct Pickup, -> bool);

#[doc(hidden)]
pub fn load_functions() {
    load_function!(Pickup_Create);
    load_function!(Pickup_AddStatic);
    load_function!(Pickup_Destroy);
    load_function!(Pickup_FromID);
    load_function!(Pickup_GetID);
    load_function!(Pickup_IsValid);
    load_function!(Pickup_IsStreamedIn);
    load_function!(Pickup_GetPos);
    load_function!(Pickup_GetModel);
    load_function!(Pickup_GetType);
    load_function!(Pickup_GetVirtualWorld);
    load_function!(Pickup_SetPos);
    load_function!(Pickup_SetModel);
    load_function!(Pickup_SetType);
    load_function!(Pickup_SetVirtualWorld);
    load_function!(Pickup_ShowForPlayer);
    load_function!(Pickup_HideForPlayer);
    load_function!(Pickup_IsHiddenForPlayer);
}
