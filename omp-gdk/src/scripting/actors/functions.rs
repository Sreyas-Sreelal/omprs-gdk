use crate::actors::*;
use crate::players::Player;
use omp_codegen::native;
use std::ffi::c_void;

native!(Actor_Create, model: i32, x: f32, y: f32, z: f32, rot: f32, id: mut i32, -> struct Actor);
native!(Actor_Destroy, actor: struct Actor, -> bool);
native!(Actor_FromID, actorid: i32, -> struct Actor);
native!(Actor_GetID, actor: struct Actor, -> i32);
native!(Actor_IsStreamedInFor, actor: struct Actor, player: struct Player, -> bool);
native!(Actor_SetVirtualWorld, actor: struct Actor, vw: i32, -> bool);
native!(Actor_GetVirtualWorld, actor: struct Actor, -> i32);
native!(Actor_ApplyAnimation, actor: struct Actor, name: str, library: str, delta: f32, anim_loop: bool, lockX: bool, lockY: bool, freeze: bool, time: i32, -> bool);
native!(Actor_ClearAnimations, actor: struct Actor, -> bool);
native!(Actor_SetPos, actor: struct Actor, x: f32, y: f32, z: f32, -> bool);
native!(Actor_GetPos, actor: struct Actor, x: mut f32, y: mut f32, z: mut f32, -> bool);
native!(Actor_SetFacingAngle, actor: struct Actor, angle: f32, -> bool);
native!(Actor_GetFacingAngle, actor: struct Actor, -> f32);
native!(Actor_SetHealth, actor: struct Actor, hp: f32, -> bool);
native!(Actor_GetHealth, actor: struct Actor, -> f32);
native!(Actor_SetInvulnerable, actor: struct Actor, toggle: bool, -> bool);
native!(Actor_IsInvulnerable, actor: struct Actor, -> bool);
native!(Actor_IsValid, actor: struct Actor, -> bool);
native!(Actor_SetSkin, actor: struct Actor, skin: i32, -> bool);
native!(Actor_GetSkin, actor: struct Actor, -> i32);
native!(Actor_GetAnimation, actor: struct Actor, library: mut str, library_len:usize, name: mut str, name_len:usize, delta: mut f32, anim_loop: mut bool, lockX: mut bool, lockY: mut bool, freeze: mut bool, time: mut i32, -> bool);
native!(Actor_GetSpawnInfo, actor: struct Actor, x: mut f32, y: mut f32, z: mut f32, angle: mut f32, skin: mut i32, -> bool);

#[doc(hidden)]
pub fn load_functions() {
    load_function!(Actor_Create);
    load_function!(Actor_Destroy);
    load_function!(Actor_FromID);
    load_function!(Actor_GetID);
    load_function!(Actor_IsStreamedInFor);
    load_function!(Actor_SetVirtualWorld);
    load_function!(Actor_GetVirtualWorld);
    load_function!(Actor_ApplyAnimation);
    load_function!(Actor_ClearAnimations);
    load_function!(Actor_SetPos);
    load_function!(Actor_GetPos);
    load_function!(Actor_SetFacingAngle);
    load_function!(Actor_GetFacingAngle);
    load_function!(Actor_SetHealth);
    load_function!(Actor_GetHealth);
    load_function!(Actor_SetInvulnerable);
    load_function!(Actor_IsInvulnerable);
    load_function!(Actor_IsValid);
    load_function!(Actor_SetSkin);
    load_function!(Actor_GetSkin);
    load_function!(Actor_GetAnimation);
    load_function!(Actor_GetSpawnInfo);
}
