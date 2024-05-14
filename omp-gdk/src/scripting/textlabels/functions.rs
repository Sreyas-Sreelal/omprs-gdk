use omp_codegen::native;

use std::ffi::c_void;

use crate::{players::Player, types::colour::Colour, types::vector::Vector3, vehicles::Vehicle};

use super::{PlayerTextLabel, TextLabel, TextLabelAttachmentData};

native!(Create3DTextLabel, text: str, colour: Colour, position: Vector3, drawDistance: f32, virtualWorld: isize, los: bool, -> struct TextLabel);
native!(Delete3DTextLabel, textlabel: struct TextLabel);
native!(Attach3DTextLabelToPlayer, textlabel: struct TextLabel, player: struct Player, offset: Vector3);
native!(Attach3DTextLabelToVehicle, textlabel: struct TextLabel, vehicle: struct Vehicle, offset: Vector3);
native!(Update3DTextLabelText, textlabel: struct TextLabel, colour: Colour, text: str);
native!(Is3DTextLabelStreamedIn, textlabel: struct TextLabel, player: struct Player, -> bool);
native!(Get3DTextLabelText, textlabel: struct TextLabel, output: mut str);
native!(Get3DTextLabelColor, textlabel: struct TextLabel,colour: mut Colour);
native!(Get3DTextLabelPos, textlabel: struct TextLabel, out: mut Vector3);
native!(Set3DTextLabelDrawDistance, textlabel: struct TextLabel, distance: f32);
native!(Get3DTextLabelDrawDistance, textlabel: struct TextLabel, -> f32);
native!(Get3DTextLabelLOS, textlabel: struct TextLabel, -> bool);
native!(Set3DTextLabelLOS, textlabel: struct TextLabel, status: bool);
native!(Get3DTextLabelVirtualWorld, textlabel: struct TextLabel, -> isize);
native!(Set3DTextLabelVirtualWorld, textlabel: struct TextLabel, world: isize);
native!(Get3DTextLabelAttachedData, textlabel: struct TextLabel, data:mut TextLabelAttachmentData);
native!(Get3DTextLabelID, textlabel: struct TextLabel, -> isize);
native!(Get3DTextLabelFromID, id: isize, -> struct TextLabel);

// Player TextLabels

native!(CreatePlayer3DTextLabelOnPlayer, player: struct Player, attachedPlayer: struct Player, text: str, colour: Colour, position: Vector3, drawDistance: f32, los: bool, -> struct PlayerTextLabel);
native!(CreatePlayer3DTextLabelOnVehicle, player: struct Player, attachedVehicle: struct Vehicle, text: str, colour: Colour, position: Vector3, drawDistance: f32, los: bool, -> struct PlayerTextLabel);
native!(CreatePlayer3DTextLabel, player: struct Player, text: str, colour: Colour, position: Vector3, drawDistance: f32, los: bool, -> struct PlayerTextLabel);
native!(DeletePlayer3DTextLabel, player: struct Player, textlabel: struct PlayerTextLabel);
native!(UpdatePlayer3DTextLabelText, textlabel: struct PlayerTextLabel, colour: Colour, text: str);
native!(GetPlayer3DTextLabelText, textlabel: struct PlayerTextLabel, output: mut str);
native!(GetPlayer3DTextLabelColor, textlabel: struct PlayerTextLabel,colour: mut Colour);
native!(GetPlayer3DTextLabelPos, textlabel: struct PlayerTextLabel, out: mut Vector3);
native!(SetPlayer3DTextLabelDrawDistance, textlabel: struct PlayerTextLabel, distance: f32);
native!(GetPlayer3DTextLabelDrawDistance, textlabel: struct PlayerTextLabel, -> f32);
native!(GetPlayer3DTextLabelLOS, textlabel: struct PlayerTextLabel, -> bool);
native!(SetPlayer3DTextLabelLOS, textlabel: struct PlayerTextLabel, status: bool);
native!(GetPlayer3DTextLabelAttachedData, textlabel: struct PlayerTextLabel, data:mut TextLabelAttachmentData);
native!(GetPlayer3DTextLabelID, textlabel: struct PlayerTextLabel, -> isize);
native!(GetPlayer3DTextLabelFromID, player: struct Player,id: isize, -> struct PlayerTextLabel);

#[doc(hidden)]
pub fn load_functions() {
    load_function!(Create3DTextLabel);
    load_function!(Delete3DTextLabel);
    load_function!(Attach3DTextLabelToPlayer);
    load_function!(Attach3DTextLabelToVehicle);
    load_function!(Update3DTextLabelText);
    load_function!(Is3DTextLabelStreamedIn);
    load_function!(Get3DTextLabelText);
    load_function!(Get3DTextLabelColor);
    load_function!(Get3DTextLabelPos);
    load_function!(Set3DTextLabelDrawDistance);
    load_function!(Get3DTextLabelDrawDistance);
    load_function!(Get3DTextLabelLOS);
    load_function!(Set3DTextLabelLOS);
    load_function!(Get3DTextLabelVirtualWorld);
    load_function!(Set3DTextLabelVirtualWorld);
    load_function!(Get3DTextLabelAttachedData);
    load_function!(Get3DTextLabelID);
    load_function!(Get3DTextLabelFromID);

    // Player TextLabels

    load_function!(CreatePlayer3DTextLabelOnPlayer);
    load_function!(CreatePlayer3DTextLabelOnVehicle);
    load_function!(CreatePlayer3DTextLabel);
    load_function!(DeletePlayer3DTextLabel);
    load_function!(UpdatePlayer3DTextLabelText);
    load_function!(GetPlayer3DTextLabelText);
    load_function!(GetPlayer3DTextLabelColor);
    load_function!(GetPlayer3DTextLabelPos);
    load_function!(SetPlayer3DTextLabelDrawDistance);
    load_function!(GetPlayer3DTextLabelDrawDistance);
    load_function!(GetPlayer3DTextLabelLOS);
    load_function!(SetPlayer3DTextLabelLOS);
    load_function!(GetPlayer3DTextLabelAttachedData);
    load_function!(GetPlayer3DTextLabelID);
    load_function!(GetPlayer3DTextLabelFromID);
}
