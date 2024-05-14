use omp_codegen::native;

use super::PlayerClass;
use crate::players::{Player, WeaponSlots};
use crate::types::vector::Vector3;
use std::ffi::c_void;

native!(CreateClass,team: u8, skin: isize, spawnPosition: Vector3, angle: f32, slots: WeaponSlots, ->isize);
native!(SetSpawnInfo,player: struct Player,player_class: PlayerClass);
native!(GetSpawnInfo,player: struct Player, data: mut PlayerClass);
native!(GetAvailableClasses, ->isize);
native!(GetClassData,classid: isize, data: mut PlayerClass, ->bool);
native!(EditClassData,classid: isize, data: PlayerClass);

#[doc(hidden)]
pub fn load_functions() {
    load_function!(CreateClass);
    load_function!(SetSpawnInfo);
    load_function!(GetSpawnInfo);
    load_function!(GetAvailableClasses);
    load_function!(GetClassData);
    load_function!(EditClassData);
}
