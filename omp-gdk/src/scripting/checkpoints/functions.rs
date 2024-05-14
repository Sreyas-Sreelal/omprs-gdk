use super::RaceCheckpointType;
use crate::{players::Player, types::vector::Vector3};
use omp_codegen::native;
use std::ffi::c_void;

native!(SetPlayerCheckpoint, player: struct Player,centrePosition: Vector3, radius: f32);
native!(DisablePlayerCheckpoint, player: struct Player);
native!(IsPlayerInCheckpoint, player: struct Player, -> bool);
native!(SetPlayerRaceCheckpoint, player: struct Player,race_check_point_type: RaceCheckpointType, centrePosition: Vector3, nextPosition: Vector3, radius: f32);
native!(DisablePlayerRaceCheckpoint, player: struct Player);
native!(IsPlayerInRaceCheckpoint, player: struct Player, -> bool);
native!(IsPlayerCheckpointActive, player: struct Player, -> bool);
native!(GetPlayerCheckpoint, player: struct Player, centrePosition: mut Vector3, radius: mut f32);
native!(IsPlayerRaceCheckpointActive, player: struct Player, -> bool);
native!(GetPlayerRaceCheckpoint, player: struct Player,centrePosition: mut Vector3, nextPosition: mut Vector3, radius: mut f32);

#[doc(hidden)]
pub fn load_functions() {
    load_function!(SetPlayerCheckpoint);
    load_function!(DisablePlayerCheckpoint);
    load_function!(IsPlayerInCheckpoint);
    load_function!(SetPlayerRaceCheckpoint);
    load_function!(DisablePlayerRaceCheckpoint);
    load_function!(IsPlayerInRaceCheckpoint);
    load_function!(IsPlayerCheckpointActive);
    load_function!(GetPlayerCheckpoint);
    load_function!(IsPlayerRaceCheckpointActive);
    load_function!(GetPlayerRaceCheckpoint);
}
