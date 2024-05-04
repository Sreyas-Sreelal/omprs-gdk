use crate::types::vector::Vector3;

pub mod events;
pub mod functions;

pub use functions::load_functions;

#[repr(C)]
#[derive(PartialEq)]
pub enum RaceCheckpointType {
    Normal = 0, // Must have nextPosition, else it shows as RACE_FINISH
    Finish,     // Must have no nextPosition, else it shows as RACE_NORMAL
    Nothing,
    AirNormal,
    AirFinish,
    AirOne,
    AirTwo,
    AirThree,
    AirFour,
    None,
}

pub struct PlayerRaceCheckPointData {
    pub center_pos: Vector3,
    pub next_pos: Vector3,
    pub radius: f32,
}

impl PlayerRaceCheckPointData {
    pub fn new(center_pos: Vector3, next_pos: Vector3, radius: f32) -> Self {
        Self {
            center_pos,
            next_pos,
            radius,
        }
    }
}

pub struct PlayerCheckPointData {
    pub center_pos: Vector3,
    pub radius: f32,
}

impl PlayerCheckPointData {
    pub fn new(center_pos: Vector3, radius: f32) -> Self {
        Self { center_pos, radius }
    }
}
