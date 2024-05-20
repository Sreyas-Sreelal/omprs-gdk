use crate::types::vector::Vector3;

pub mod events;
pub mod functions;

pub use functions::load_functions;

/// Types Of Race Checkpoints
#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum RaceCheckpointType {
    /// Normal, must have nextPosition, else it shows as RACE_FINISH
    Normal = 0,
    /// Finish checkpoint, must have no nextPosition, else it shows as RACE_NORMAL
    Finish,
    /// Nothing (Only the checkpoint without anything on it)     
    Nothing,
    /// Normal Air Checkpoint
    AirNormal,
    /// Air Finish Checkpoint
    AirFinish,
    /// Air (rotates and stops)
    AirOne,
    /// Air (increases, decreases and disappears)
    AirTwo,
    /// Air (swings down and up)
    AirThree,
    /// Air (swings up and down)
    AirFour,
    /// None
    None,
}

/// Player race checkpoint information
pub struct PlayerRaceCheckPointData {
    /// position of checkpoint
    pub center_pos: Vector3,
    /// coordinates of next checkpoint, also used for arrow direction
    pub next_pos: Vector3,
    /// radius of the checkpoint
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

/// Player checkpoint information
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct PlayerCheckPointData {
    /// position of checkpoint
    pub center_pos: Vector3,
    /// radius of the checkpoint
    pub radius: f32,
}

impl PlayerCheckPointData {
    pub fn new(center_pos: Vector3, radius: f32) -> Self {
        Self { center_pos, radius }
    }
}
