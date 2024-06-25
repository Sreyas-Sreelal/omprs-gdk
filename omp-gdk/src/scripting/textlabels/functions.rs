use omp_codegen::native;

use std::ffi::c_void;

use crate::{players::Player, vehicles::Vehicle};

use super::{PlayerTextLabel, TextLabel};

native!(TextLabel_Create, text: str, color: u32, x: f32, y: f32, z: f32, drawDistance: f32, virtualWorld: i32, los: bool, id: mut i32, -> struct TextLabel);
native!(TextLabel_Destroy, textlabel: struct TextLabel, -> bool);
native!(TextLabel_FromID, textlabelid: i32, -> struct TextLabel);
native!(TextLabel_GetID, textlabel: struct TextLabel, -> i32);
native!(TextLabel_AttachToPlayer, textlabel: struct TextLabel, player: struct Player, offsetX: f32, offsetY: f32, offsetZ: f32, -> bool);
native!(TextLabel_AttachToVehicle, textlabel: struct TextLabel, vehicle: struct Vehicle, offsetX: f32, offsetY: f32, offsetZ: f32, -> bool);
native!(TextLabel_UpdateText, textlabel: struct TextLabel, color: u32, text: str, -> bool);
native!(TextLabel_IsValid, textlabel: struct TextLabel, -> bool);
native!(TextLabel_IsStreamedIn, player: struct Player, textlabel: struct TextLabel, -> bool);
native!(TextLabel_GetText, textlabel: struct TextLabel, output: mut str, -> bool);
native!(TextLabel_GetColor, textlabel: struct TextLabel, -> u32);
native!(TextLabel_GetPos, textlabel: struct TextLabel, x: mut f32, y: mut f32, z: mut f32, -> bool);
native!(TextLabel_SetDrawDistance, textlabel: struct TextLabel, distance: f32, -> bool);
native!(TextLabel_GetDrawDistance, textlabel: struct TextLabel, -> f32);
native!(TextLabel_GetLOS, textlabel: struct TextLabel, -> bool);
native!(TextLabel_SetLOS, textlabel: struct TextLabel, status: bool, -> bool);
native!(TextLabel_GetVirtualWorld, textlabel: struct TextLabel, -> i32);
native!(TextLabel_SetVirtualWorld, textlabel: struct TextLabel, world: i32, -> bool);
native!(TextLabel_GetAttachedData, textlabel: struct TextLabel, attached_player: mut i32, attached_vehicle: mut i32, -> bool);

// Player TextLabels

native!(PlayerTextLabel_Create, player: struct Player, text: str, color: u32, x: f32, y: f32, z: f32, drawDistance: f32, attachedPlayer: struct Player, attachedVehicle: struct Vehicle, los: bool, id: mut i32, -> struct PlayerTextLabel);
native!(PlayerTextLabel_Destroy, player: struct Player, textlabel: struct PlayerTextLabel, -> bool);
native!(PlayerTextLabel_FromID, player: struct Player, textlabelid: i32, -> struct PlayerTextLabel);
native!(PlayerTextLabel_GetID, player: struct Player, textlabel: struct PlayerTextLabel, -> i32);
native!(PlayerTextLabel_UpdateText, player: struct Player, textlabel: struct PlayerTextLabel, color: u32, text: str, -> bool);
native!(PlayerTextLabel_IsValid, player: struct Player, textlabel: struct PlayerTextLabel, valid: mut bool, -> bool);
native!(PlayerTextLabel_GetText, player: struct Player, textlabel: struct PlayerTextLabel, output: mut str, -> bool);
native!(PlayerTextLabel_GetColor, player: struct Player, textlabel: struct PlayerTextLabel, color: mut u32, -> bool);
native!(PlayerTextLabel_GetPos, player: struct Player, textlabel: struct PlayerTextLabel, x: mut f32, y: mut f32, z: mut f32, -> bool);
native!(PlayerTextLabel_SetDrawDistance, player: struct Player, textlabel: struct PlayerTextLabel, distance: f32, -> bool);
native!(PlayerTextLabel_GetDrawDistance, player: struct Player, textlabel: struct PlayerTextLabel, -> f32);
native!(PlayerTextLabel_GetLOS, player: struct Player, textlabel: struct PlayerTextLabel, -> bool);
native!(PlayerTextLabel_SetLOS, player: struct Player, textlabel: struct PlayerTextLabel, status: bool, -> bool);
native!(PlayerTextLabel_GetVirtualWorld, player: struct Player, -> i32);
native!(PlayerTextLabel_GetAttachedData, player: struct Player, textlabel: struct PlayerTextLabel, attached_player: mut i32, attached_vehicle: mut i32, -> bool);

#[doc(hidden)]
pub fn load_functions() {
    load_function!(TextLabel_Create);
    load_function!(TextLabel_Destroy);
    load_function!(TextLabel_FromID);
    load_function!(TextLabel_GetID);
    load_function!(TextLabel_AttachToPlayer);
    load_function!(TextLabel_AttachToVehicle);
    load_function!(TextLabel_UpdateText);
    load_function!(TextLabel_IsValid);
    load_function!(TextLabel_IsStreamedIn);
    load_function!(TextLabel_GetText);
    load_function!(TextLabel_GetColor);
    load_function!(TextLabel_GetPos);
    load_function!(TextLabel_SetDrawDistance);
    load_function!(TextLabel_GetDrawDistance);
    load_function!(TextLabel_GetLOS);
    load_function!(TextLabel_SetLOS);
    load_function!(TextLabel_GetVirtualWorld);
    load_function!(TextLabel_SetVirtualWorld);
    load_function!(TextLabel_GetAttachedData);

    // Player TextLabels

    load_function!(PlayerTextLabel_Create);
    load_function!(PlayerTextLabel_Destroy);
    load_function!(PlayerTextLabel_FromID);
    load_function!(PlayerTextLabel_GetID);
    load_function!(PlayerTextLabel_UpdateText);
    load_function!(PlayerTextLabel_IsValid);
    load_function!(PlayerTextLabel_GetText);
    load_function!(PlayerTextLabel_GetColor);
    load_function!(PlayerTextLabel_GetPos);
    load_function!(PlayerTextLabel_SetDrawDistance);
    load_function!(PlayerTextLabel_GetDrawDistance);
    load_function!(PlayerTextLabel_GetLOS);
    load_function!(PlayerTextLabel_SetLOS);
    load_function!(PlayerTextLabel_GetVirtualWorld);
    load_function!(PlayerTextLabel_GetAttachedData);
}
