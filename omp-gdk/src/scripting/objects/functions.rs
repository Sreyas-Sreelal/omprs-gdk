use omp_codegen::native;
use std::ffi::c_void;

use crate::{players::Player, types::colour::Colour, types::vector::Vector3, vehicles::Vehicle};

use super::{
    Object, ObjectAttachmentData, ObjectMaterialSize, ObjectMaterialTextAlign, ObjectMoveData,
    PlayerObject,
};

native!(CreateObject, modelid: isize, position: Vector3, rotation: Vector3, drawDistance: f32, -> struct Object);
native!(DestroyObject, object: struct Object, -> bool);
native!(AttachObjectToVehicle, object: struct Object, vehicle: struct Vehicle, offset: Vector3, rotation: Vector3);
native!(AttachObjectToObject, object: struct Object, objAttachedTo: struct Object, offset: Vector3, rotation: Vector3, syncRotation: bool);
native!(AttachObjectToPlayer, object: struct Object, player: struct Player, offset: Vector3, rotation: Vector3);
native!(SetObjectPos, object: struct Object, position: Vector3);
native!(GetObjectPos, object: struct Object, position: mut Vector3);
native!(SetObjectRot, object: struct Object, rotation: Vector3);
native!(GetObjectRot, object: struct Object, rotation: mut Vector3);
native!(GetObjectModel, object: struct Object, -> isize);
native!(SetObjectNoCameraCol, object: struct Object);
native!(MoveObject, object: struct Object, data: ObjectMoveData, -> isize);
native!(StopObject, object: struct Object);
native!(IsObjectMoving, object: struct Object, -> bool);
native!(EditObject, player: struct Player, object: struct Object);
native!(SelectObject, player: struct Player);
native!(EndObjectEditing, player: struct Player);
native!(SetObjectMaterial, object: struct Object, materialIndex: isize, modelId: isize, textureLibrary: str, textureName: str, materialColour: Colour);
native!(SetObjectMaterialText, object: struct Object, text: str, materialIndex: isize, materialSize: ObjectMaterialSize , fontface: str, fontsize: isize, bold: bool, fontColour: Colour, backgroundColour: Colour, textalignment: ObjectMaterialTextAlign);
native!(SetObjectsDefaultCameraCol, disable: bool);
native!(GetObjectDrawDistance, object: struct Object, -> f32);
native!(GetObjectMoveSpeed, object: struct Object, -> f32);
native!(GetObjectMoveData, object: struct Object, data: mut ObjectMoveData);
native!(GetObjectAttachedData, object: struct Object, data: mut ObjectAttachmentData);
native!(IsObjectMaterialSlotUsed, object: struct Object, materialIndex: isize, -> bool);
native!(GetObjectMaterialData, object: struct Object, materialIndex: isize, modelid: mut isize, textureLibrary: mut str, textureName: mut str, materialColour: mut Colour, text: mut str, materialSize: mut isize, fontFace: mut str, fontSize: mut isize, bold: mut bool, fontColour: mut Colour, backgroundColour: mut Colour, textAlignment: mut isize);
native!(IsObjectNoCameraCol, object: struct Object, -> bool);
native!(GetObjectID, object: struct Object, -> isize);
native!(GetObjectFromID, id: isize, -> struct Object);

// player object functions

native!(CreatePlayerObject, player: struct Player, modelid: isize, position: Vector3 , rotation: Vector3 , drawDistance: f32, -> struct PlayerObject);
native!(DestroyPlayerObject, player: struct Player, object: struct PlayerObject);
native!(AttachPlayerObjectToVehicle, object: struct PlayerObject, vehicle: struct Vehicle, offset: Vector3 , rotation: Vector3);
native!(AttachPlayerObjectToPlayer, object: struct PlayerObject, player_attached_to: struct Player, offset: Vector3 , rotation: Vector3);
native!(AttachPlayerObjectToObject, object: struct PlayerObject, attachedTo: struct PlayerObject, offset: Vector3 , rotation: Vector3);
native!(SetPlayerObjectPos, object: struct PlayerObject, position: Vector3);
native!(GetPlayerObjectPos, object: struct PlayerObject, position: mut Vector3);
native!(SetPlayerObjectRot, object: struct PlayerObject, rotation: Vector3);
native!(GetPlayerObjectRot, object: struct PlayerObject, rotation: mut Vector3);
native!(GetPlayerObjectModel, object: struct PlayerObject, -> isize);
native!(SetPlayerObjectNoCameraCol, object: struct PlayerObject);
native!(MovePlayerObject, object: struct PlayerObject, data: ObjectMoveData, -> isize);
native!(StopPlayerObject, object: struct PlayerObject);
native!(IsPlayerObjectMoving, object: struct PlayerObject, -> bool);
native!(EditPlayerObject, player: struct Player, object: struct PlayerObject);
native!(SetPlayerObjectMaterial, object: struct PlayerObject, materialIndex: isize, modelId: isize, textureLibrary: str, textureName: str, materialColour: Colour);
native!(SetPlayerObjectMaterialText, object: struct PlayerObject, text: str, materialIndex: isize, materialSize: isize, fontface: str, fontsize: isize,  bold: bool, fontColour: Colour , backgroundColour: Colour ,  textalignment: ObjectMaterialTextAlign);
native!(GetPlayerObjectDrawDistance, object: struct PlayerObject, -> f32);
native!(GetPlayerObjectMoveSpeed, object: struct PlayerObject, -> f32);
native!(GetPlayerObjectMovingData, object: struct PlayerObject, data: mut ObjectMoveData);
native!(GetPlayerObjectAttachedData, object: struct PlayerObject, data: mut ObjectAttachmentData);
native!(IsPlayerObjectMaterialSlotUsed, object: struct PlayerObject, materialIndex: isize, -> bool);
native!(GetPlayerObjectMaterialData, object: struct PlayerObject, materialIndex: isize, modelid: mut isize, textureLibrary: mut str, textureName: mut str, materialColour: mut Colour , text: mut str, materialSize: mut isize, fontFace: mut str, fontSize: mut isize, bold:mut bool, fontColour: mut Colour , backgroundColour: mut Colour , textAlignment: mut isize);
native!(IsPlayerObjectNoCameraCol, object: struct PlayerObject, -> bool);
native!(GetPlayerObjectID, object: struct PlayerObject, -> isize);
native!(GetPlayerObjectFromID, player: struct Player, id: isize, -> struct PlayerObject);

#[doc(hidden)]
pub fn load_functions() {
    load_function!(CreateObject);
    load_function!(DestroyObject);
    load_function!(AttachObjectToVehicle);
    load_function!(AttachObjectToObject);
    load_function!(AttachObjectToPlayer);
    load_function!(SetObjectPos);
    load_function!(GetObjectPos);
    load_function!(SetObjectRot);
    load_function!(GetObjectRot);
    load_function!(GetObjectModel);
    load_function!(SetObjectNoCameraCol);
    load_function!(MoveObject);
    load_function!(StopObject);
    load_function!(IsObjectMoving);
    load_function!(EditObject);
    load_function!(SelectObject);
    load_function!(EndObjectEditing);
    load_function!(SetObjectMaterial);
    load_function!(SetObjectMaterialText);
    load_function!(SetObjectsDefaultCameraCol);
    load_function!(GetObjectDrawDistance);
    load_function!(GetObjectMoveSpeed);
    load_function!(GetObjectMoveData);
    load_function!(GetObjectAttachedData);
    load_function!(IsObjectMaterialSlotUsed);
    load_function!(GetObjectMaterialData);
    load_function!(IsObjectNoCameraCol);
    load_function!(GetObjectID);
    load_function!(GetObjectFromID);

    // player object functions

    load_function!(CreatePlayerObject);
    load_function!(DestroyPlayerObject);
    load_function!(AttachPlayerObjectToVehicle);
    load_function!(AttachPlayerObjectToPlayer);
    load_function!(AttachPlayerObjectToObject);
    load_function!(SetPlayerObjectPos);
    load_function!(GetPlayerObjectPos);
    load_function!(SetPlayerObjectRot);
    load_function!(GetPlayerObjectRot);
    load_function!(GetPlayerObjectModel);
    load_function!(SetPlayerObjectNoCameraCol);
    load_function!(MovePlayerObject);
    load_function!(StopPlayerObject);
    load_function!(IsPlayerObjectMoving);
    load_function!(EditPlayerObject);
    load_function!(SetPlayerObjectMaterial);
    load_function!(SetPlayerObjectMaterialText);
    load_function!(GetPlayerObjectDrawDistance);
    load_function!(GetPlayerObjectMoveSpeed);
    load_function!(GetPlayerObjectMovingData);
    load_function!(GetPlayerObjectAttachedData);
    load_function!(IsPlayerObjectMaterialSlotUsed);
    load_function!(GetPlayerObjectMaterialData);
    load_function!(IsPlayerObjectNoCameraCol);
    load_function!(GetPlayerObjectID);
    load_function!(GetPlayerObjectFromID);
}
