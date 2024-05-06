use omp_codegen::callback;

use crate::{players::Player, types::vector::Vector3};

use super::{Object, ObjectAttachmentSlotData, ObjectEditResponse, PlayerObject};

callback!(onObjectMoved, object: Object);
callback!(onPlayerObjectMoved,player:Player,object:PlayerObject);
callback!(onPlayerEditObject, player:Player,object:Object, response: ObjectEditResponse, offset: Vector3, rotation: Vector3);
callback!(onPlayerObjectEdited, player:Player,object:PlayerObject,response: ObjectEditResponse, offset: Vector3, rotation: Vector3);
callback!(onPlayerEditAttachedObject, player: Player, index:isize, saved: bool, data: ObjectAttachmentSlotData);
callback!(onPlayerSelectObject, player:Player,object:Object, model:isize, position: Vector3);
callback!(onPlayerObjectSelected, player:Player,object:PlayerObject,  model:isize, position: Vector3);
