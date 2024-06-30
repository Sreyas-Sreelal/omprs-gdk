use omp_codegen::native;

use super::Class;
use crate::players::PlayerWeapon;
use std::ffi::c_void;

native!(Class_Add, team: u8, skin: i32, x: f32, y: f32, z: f32, angle: f32, weapon1: u8, ammo1: u32, weapon2: u8, ammo2: u32, weapon3: u8, ammo3: u32, id: mut i32, -> struct Class);
native!(Class_FromID, classid: i32, -> struct Class);
native!(Class_GetID, cls: struct Class, -> i32);
native!(Class_Count, -> i32);
native!(Class_GetData, classptr: struct Class, teamid: mut u8, skin: mut i32, x: mut f32, y: mut f32, z: mut f32, angle: mut f32, weapon1: mut u8, weapon1_ammo: mut u32, weapon2: mut u8, weapon2_ammo: mut u32, weapon3: mut u8, weapon3_ammo: mut u32, -> bool);
native!(Class_Edit, classptr: struct Class, teamid: u8, skin: i32, x: f32, y: f32, z: f32, angle: f32, weapon1: PlayerWeapon, ammo1: u32, weapon2: PlayerWeapon, ammo2: u32, weapon3: PlayerWeapon, ammo3: u32, -> bool);

#[doc(hidden)]
pub fn load_functions() {
    load_function!(Class_Add);
    load_function!(Class_FromID);
    load_function!(Class_GetID);
    load_function!(Class_Count);
    load_function!(Class_GetData);
    load_function!(Class_Edit);
}
