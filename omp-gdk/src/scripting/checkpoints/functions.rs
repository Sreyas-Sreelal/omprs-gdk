use crate::players::Player;
use omp_codegen::native;
use std::ffi::c_void;

native!(Checkpoint_Set, player: struct Player, x: f32, y: f32, z: f32, radius: f32, -> bool);
native!(Checkpoint_Disable, player: struct Player, -> bool);
native!(Checkpoint_IsPlayerIn, player: struct Player, -> bool);
native!(Checkpoint_IsActive, player: struct Player, -> bool);
native!(Checkpoint_Get, player: struct Player, x: mut f32, y: mut f32, z: mut f32, radius: mut f32, -> bool);
native!(RaceCheckpoint_Set, player: struct Player, cp_type: i32, x: f32, y: f32, z: f32, nextX: f32, nextY: f32, nextZ: f32, radius: f32, -> bool);
native!(RaceCheckpoint_Disable, player: struct Player, -> bool);
native!(RaceCheckpoint_IsPlayerIn, player: struct Player, -> bool);
native!(RaceCheckpoint_IsActive, player: struct Player, -> bool);
native!(RaceCheckpoint_Get, player: struct Player, x: mut f32, y: mut f32, z: mut f32, nextX: mut f32, nextY: mut f32, nextZ: mut f32, radius: mut f32, -> bool);

#[doc(hidden)]
pub fn load_functions() {
    load_function!(Checkpoint_Set);
    load_function!(Checkpoint_Disable);
    load_function!(Checkpoint_IsPlayerIn);
    load_function!(Checkpoint_IsActive);
    load_function!(Checkpoint_Get);
    load_function!(RaceCheckpoint_Set);
    load_function!(RaceCheckpoint_Disable);
    load_function!(RaceCheckpoint_IsPlayerIn);
    load_function!(RaceCheckpoint_IsActive);
    load_function!(RaceCheckpoint_Get);
}
