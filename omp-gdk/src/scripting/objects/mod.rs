pub mod events;
pub mod functions;

use std::ffi::c_void;

use crate::{players::Player, types::colour::Colour, types::vector::Vector3, vehicles::Vehicle};

pub use functions::load_functions;

pub struct Object {
    handle: *const c_void,
}

impl Object {
    pub fn get_handle(&self) -> *const c_void {
        self.handle
    }

    pub fn new(handle: *const c_void) -> Self {
        Self { handle }
    }

    /// Creates an object at specified coordinates in the game world.
    pub fn create(
        modelid: i32,
        position: Vector3,
        rotation: Vector3,
        draw_distance: f32,
    ) -> Option<Object> {
        let mut _id = 0;
        functions::Object_Create(
            modelid,
            position.x,
            position.y,
            position.z,
            rotation.x,
            rotation.y,
            rotation.z,
            draw_distance,
            &mut _id,
        )
    }

    /// Destroys (removes) an object that was created using create method.
    pub fn destroy(&self) -> bool {
        functions::Object_Destroy(self)
    }

    /// Attach an object to a vehicle.
    pub fn attach_to_vehicle(&self, vehicle: &Vehicle, offset: Vector3, rotation: Vector3) -> bool {
        functions::Object_AttachToVehicle(
            self, vehicle, offset.x, offset.y, offset.z, rotation.x, rotation.y, rotation.z,
        )
    }

    /// Attach an object to an object.
    pub fn attach_to_object(
        &self,
        obj_attached_to: &Object,
        offset: Vector3,
        rotation: Vector3,
        sync_rotation: bool,
    ) -> bool {
        functions::Object_AttachToObject(
            self,
            obj_attached_to,
            offset.x,
            offset.y,
            offset.z,
            rotation.x,
            rotation.y,
            rotation.z,
            sync_rotation,
        )
    }

    /// Attach a player object to a player.
    pub fn attach_to_player(&self, player: &Player, offset: Vector3, rotation: Vector3) -> bool {
        functions::Object_AttachToPlayer(
            self, player, offset.x, offset.y, offset.z, rotation.x, rotation.y, rotation.z,
        )
    }

    ///  Set the position of an object.
    pub fn set_pos(&self, position: Vector3) -> bool {
        functions::Object_SetPos(self, position.x, position.y, position.z)
    }

    ///  Locate an object.
    pub fn get_pos(&self) -> Vector3 {
        let mut pos = Vector3::default();
        functions::Object_GetPos(self, &mut pos.x, &mut pos.y, &mut pos.z);
        pos
    }

    /// Set the rotation of an object.
    pub fn set_rotation(&self, rotation: Vector3) -> bool {
        functions::Object_SetRot(self, rotation.x, rotation.y, rotation.z)
    }

    /// Get the rotation of an object.
    pub fn get_rotation(&self) -> Vector3 {
        let mut rotation = Vector3::default();
        functions::Object_GetRot(self, &mut rotation.x, &mut rotation.y, &mut rotation.z);
        rotation
    }

    /// Get the model ID of an object
    pub fn get_model(&self) -> i32 {
        functions::Object_GetModel(self)
    }

    /// Disable collisions between players' cameras and the specified object.
    pub fn set_no_camera_collision(&self) -> bool {
        functions::Object_SetNoCameraCollision(self)
    }

    /// Move an object to a new position with a set speed. Players/vehicles will 'surf' the object as it moves.
    pub fn move_object(&self, data: ObjectMoveData) -> i32 {
        functions::Object_Move(
            self,
            data.targetPos.x,
            data.targetPos.y,
            data.targetPos.z,
            data.speed,
            data.targetRot.x,
            data.targetRot.y,
            data.targetRot.z,
        )
    }

    /// Stop an object from moving.
    pub fn stop(&self) -> bool {
        functions::Object_Stop(self)
    }

    /// Check if the object is moving.
    pub fn is_moving(&self) -> bool {
        functions::Object_IsMoving(self)
    }

    /// Replace the texture of an object with the texture from another model in the game.
    pub fn set_material(
        &self,
        material_index: i32,
        model_id: i32,
        texture_library: &str,
        texture_name: &str,
        material_colour: Colour,
    ) -> bool {
        functions::Object_SetMaterial(
            self,
            material_index,
            model_id,
            texture_library,
            texture_name,
            material_colour.argb(),
        )
    }

    /// Replace the texture of an object with text.
    pub fn set_material_text(
        &self,
        text: &str,
        material_index: i32,
        material_size: ObjectMaterialSize,
        fontface: &str,
        fontsize: i32,
        bold: bool,
        font_colour: Colour,
        background_colour: Colour,
        textalignment: ObjectMaterialTextAlign,
    ) -> bool {
        functions::Object_SetMaterialText(
            self,
            text,
            material_index,
            material_size as i32,
            fontface,
            fontsize,
            bold,
            font_colour.argb(),
            background_colour.argb(),
            textalignment as i32,
        )
    }

    /// Allows camera collisions with newly created objects to be disabled by default.
    pub fn set_objects_default_camera_collision(disable: bool) -> bool {
        functions::Object_SetDefaultCameraCollision(disable)
    }

    /// Get the draw distance of an object.
    pub fn get_draw_distance(&self) -> f32 {
        functions::Object_GetDrawDistance(self)
    }

    /// Get the move speed of an object.
    pub fn get_move_speed(&self) -> f32 {
        functions::Object_GetMoveSpeed(self)
    }

    /// Get the move data of an object.
    pub fn get_move_data(&self) -> ObjectMoveData {
        let mut data = ObjectMoveData::default();
        data.speed = functions::Object_GetMoveSpeed(self);
        functions::Object_GetMovingTargetPos(
            self,
            &mut data.targetPos.x,
            &mut data.targetPos.y,
            &mut data.targetPos.z,
        );
        functions::Object_GetMovingTargetPos(
            self,
            &mut data.targetRot.x,
            &mut data.targetRot.y,
            &mut data.targetRot.z,
        );
        data
    }

    /// Get the attachment data of an object.
    pub fn get_attached_data(&self) -> ObjectAttachmentData {
        let mut data = ObjectAttachmentData::default();
        let (mut pid, mut oid, mut vid): (i32, i32, i32) = (-1, -1, -1);

        functions::Object_GetAttachedData(self, &mut pid, &mut oid, &mut vid);
        if pid != 65535 {
            data.ID = pid;
            data.attachment_type = ObjectAttachmentType::Player;
        } else if oid != 65535 {
            data.ID = oid;
            data.attachment_type = ObjectAttachmentType::Object;
        } else if vid != 65535 {
            data.ID = vid;
            data.attachment_type = ObjectAttachmentType::Vehicle;
        } else {
            data.attachment_type = ObjectAttachmentType::None;
        }

        data.syncRotation = functions::Object_GetSyncRotation(self);
        functions::Object_GetAttachedOffset(
            self,
            &mut data.offset.x,
            &mut data.offset.y,
            &mut data.offset.z,
            &mut data.rotation.x,
            &mut data.rotation.y,
            &mut data.rotation.z,
        );
        data
    }

    /// Checks if a slot of object material is used.
    pub fn is_material_slot_used(&self, material_index: i32) -> bool {
        functions::Object_IsMaterialSlotUsed(self, material_index)
    }

    /// Get object's material data
    pub fn get_material_data(&self, material_index: i32) -> ObjectMaterialData {
        let (
            mut modelid,
            mut texture_library,
            mut texture_name,
            mut material_colour,
            mut text,
            mut material_size,
            mut font_face,
            mut font_size,
            mut bold,
            mut font_colour,
            mut background_colour,
            mut text_alignment,
        ): (
            i32,
            String,
            String,
            i32,
            String,
            i32,
            String,
            i32,
            bool,
            i32,
            i32,
            i32,
        ) = Default::default();

        functions::Object_GetMaterial(
            self,
            material_index,
            &mut modelid,
            &mut texture_library,
            16,
            &mut texture_name,
            16,
            &mut material_colour,
        );

        functions::Object_GetMaterialText(
            self,
            material_index,
            &mut text,
            16,
            &mut material_size,
            &mut font_face,
            16,
            &mut font_size,
            &mut bold,
            &mut font_colour,
            &mut background_colour,
            &mut text_alignment,
        );

        ObjectMaterialData::new(
            modelid,
            texture_library,
            texture_name,
            Colour::from_argb(material_colour as u32),
            text,
            material_size,
            font_face,
            font_size,
            bold,
            Colour::from_argb(font_colour as u32),
            Colour::from_argb(background_colour as u32),
            text_alignment,
        )
    }

    /// Check if collisions between players' cameras and the specified object is disabled.
    pub fn is_no_camera_collision(&self) -> bool {
        functions::Object_IsObjectNoCameraCollision(self)
    }

    /// Get object's id
    pub fn get_id(&self) -> i32 {
        functions::Object_GetID(self)
    }

    /// Get Object from an id
    pub fn from_id(id: i32) -> Option<Object> {
        functions::Object_FromID(id)
    }
}

pub struct PlayerObject {
    handle: *const c_void,
    pub player: Player,
}

impl PlayerObject {
    pub fn get_handle(&self) -> *const c_void {
        self.handle
    }

    pub fn new(handle: *const c_void, player: Player) -> Self {
        Self { handle, player }
    }

    /// Attach a player object to a vehicle.
    pub fn attach_player_object_to_vehicle(
        &self,
        vehicle: &Vehicle,
        offset: Vector3,
        rotation: Vector3,
    ) -> bool {
        functions::PlayerObject_AttachToVehicle(
            &self.player,
            self,
            vehicle,
            offset.x,
            offset.y,
            offset.z,
            rotation.x,
            rotation.y,
            rotation.z,
        )
    }
    /// Attach PlayerObject to another player
    pub fn attach_player_object_to_player(
        &self,
        player_attached_to: &Player,
        offset: Vector3,
        rotation: Vector3,
    ) -> bool {
        functions::PlayerObject_AttachToPlayer(
            &self.player,
            self,
            player_attached_to,
            offset.x,
            offset.y,
            offset.z,
            rotation.x,
            rotation.y,
            rotation.z,
        )
    }
    /// You can use this function to attach player-objects to other player-objects.
    pub fn attach_player_object_to_object(
        &self,
        attached_to: &PlayerObject,
        offset: Vector3,
        rotation: Vector3,
    ) -> bool {
        functions::PlayerObject_AttachToObject(
            &self.player,
            self,
            attached_to,
            offset.x,
            offset.y,
            offset.z,
            rotation.x,
            rotation.y,
            rotation.z,
        )
    }
    /// Sets the position of a player-object to the specified coordinates.
    pub fn set_player_object_pos(&self, position: Vector3) -> bool {
        functions::PlayerObject_SetPos(&self.player, self, position.x, position.y, position.z)
    }
    /// Get the position of a player object (CreatePlayerObject).
    pub fn get_player_object_pos(&self) -> Vector3 {
        let mut position = Vector3::default();
        functions::PlayerObject_GetPos(
            &self.player,
            self,
            &mut position.x,
            &mut position.y,
            &mut position.z,
        );
        position
    }
    /// Set the rotation of an object on the X, Y and Z axis.
    pub fn set_player_object_rotation(&self, rotation: Vector3) -> bool {
        functions::PlayerObject_SetRot(&self.player, self, rotation.x, rotation.y, rotation.z)
    }
    /// Use this function to get the object's current rotation.
    pub fn get_player_object_rotation(&self) -> Vector3 {
        let mut rotation = Vector3::default();
        functions::PlayerObject_GetRot(
            &self.player,
            self,
            &mut rotation.x,
            &mut rotation.y,
            &mut rotation.z,
        );
        rotation
    }
    /// Retrieve the model ID of a player-object.
    pub fn get_player_object_model(&self) -> i32 {
        functions::PlayerObject_GetModel(&self.player, self)
    }
    /// Toggles a player object camera collision.
    pub fn set_player_object_no_camera_collision(&self) -> bool {
        functions::PlayerObject_SetNoCameraCollision(&self.player, self)
    }
    /// Move a player object with a set speed.
    pub fn move_player_object(&self, data: ObjectMoveData) -> i32 {
        functions::PlayerObject_Move(
            &self.player,
            self,
            data.targetPos.x,
            data.targetPos.y,
            data.targetPos.z,
            data.speed,
            data.targetRot.x,
            data.targetRot.y,
            data.targetRot.z,
        )
    }
    /// Stop a moving player-object after MovePlayerObject has been used.
    pub fn stop_player_object(&self) -> bool {
        functions::PlayerObject_Stop(&self.player, self)
    }
    /// Checks if the given player objectid is moving.
    pub fn is_player_object_moving(&self) -> bool {
        functions::PlayerObject_IsMoving(&self.player, self)
    }

    /// Replace the texture of a player-object with the texture from another model in the game.
    pub fn set_player_object_material(
        &self,
        material_index: i32,
        model_id: i32,
        texture_library: &str,
        texture_name: &str,
        material_colour: Colour,
    ) -> bool {
        functions::PlayerObject_SetMaterial(
            &self.player,
            self,
            material_index,
            model_id,
            texture_library,
            texture_name,
            material_colour.argb(),
        )
    }
    /// Replace the texture of a player object with text.
    pub fn set_player_object_material_text(
        &self,
        text: &str,
        material_index: i32,
        material_size: i32,
        fontface: &str,
        fontsize: i32,
        bold: bool,
        font_colour: Colour,
        background_colour: Colour,
        textalignment: ObjectMaterialTextAlign,
    ) -> bool {
        functions::PlayerObject_SetMaterialText(
            &self.player,
            self,
            text,
            material_index,
            material_size,
            fontface,
            fontsize,
            bold,
            font_colour.argb(),
            background_colour.argb(),
            textalignment as i32,
        )
    }
    /// Get the draw distance of a player-object.
    pub fn get_player_object_draw_distance(&self) -> f32 {
        functions::PlayerObject_GetDrawDistance(&self.player, self)
    }
    /// Get the move speed of a player-object.
    pub fn get_player_object_move_speed(&self) -> f32 {
        functions::PlayerObject_GetMoveSpeed(&self.player, self)
    }
    pub fn get_player_object_moving_data(&self) -> ObjectMoveData {
        let mut data = ObjectMoveData::default();

        data.speed = functions::PlayerObject_GetMoveSpeed(&self.player, self);
        functions::PlayerObject_GetMovingTargetPos(
            &self.player,
            self,
            &mut data.targetPos.x,
            &mut data.targetPos.y,
            &mut data.targetPos.z,
        );
        functions::PlayerObject_GetMovingTargetRot(
            &self.player,
            self,
            &mut data.targetRot.x,
            &mut data.targetRot.y,
            &mut data.targetRot.z,
        );
        data
    }
    /// Get the attachment data of a player-object.
    pub fn get_player_object_attached_data(&self) -> ObjectAttachmentData {
        let mut data = ObjectAttachmentData::default();
        let (mut pid, mut oid, mut vid): (i32, i32, i32) = (-1, -1, -1);

        functions::PlayerObject_GetAttachedData(&self.player, self, &mut pid, &mut oid, &mut vid);
        if pid != 65535 {
            data.ID = pid;
            data.attachment_type = ObjectAttachmentType::Player;
        } else if oid != 65535 {
            data.ID = oid;
            data.attachment_type = ObjectAttachmentType::Object;
        } else if vid != 65535 {
            data.ID = vid;
            data.attachment_type = ObjectAttachmentType::Vehicle;
        } else {
            data.attachment_type = ObjectAttachmentType::None;
        }

        data.syncRotation = functions::PlayerObject_GetSyncRotation(&self.player, self);
        functions::PlayerObject_GetAttachedOffset(
            &self.player,
            self,
            &mut data.offset.x,
            &mut data.offset.y,
            &mut data.offset.z,
            &mut data.rotation.x,
            &mut data.rotation.y,
            &mut data.rotation.z,
        );
        data
    }
    /// Checks if a slot of player-object material is used.
    pub fn is_player_object_material_slot_used(&self, material_index: i32) -> bool {
        functions::PlayerObject_IsMaterialSlotUsed(&self.player, self, material_index)
    }
    pub fn get_player_object_material_data(&self, material_index: i32) -> ObjectMaterialData {
        let (
            mut modelid,
            mut texture_library,
            mut texture_name,
            mut material_colour,
            mut text,
            mut material_size,
            mut font_face,
            mut font_size,
            mut bold,
            mut font_colour,
            mut background_colour,
            mut text_alignment,
        ): (
            i32,
            String,
            String,
            i32,
            String,
            i32,
            String,
            i32,
            bool,
            i32,
            i32,
            i32,
        ) = Default::default();

        functions::PlayerObject_GetMaterial(
            &self.player,
            self,
            material_index,
            &mut modelid,
            &mut texture_library,
            16,
            &mut texture_name,
            16,
            &mut material_colour,
        );

        functions::PlayerObject_GetMaterialText(
            &self.player,
            self,
            material_index,
            &mut text,
            16,
            &mut material_size,
            &mut font_face,
            16,
            &mut font_size,
            &mut bold,
            &mut font_colour,
            &mut background_colour,
            &mut text_alignment,
        );

        ObjectMaterialData::new(
            modelid,
            texture_library,
            texture_name,
            Colour::from_argb(material_colour as u32),
            text,
            material_size,
            font_face,
            font_size,
            bold,
            Colour::from_argb(font_colour as u32),
            Colour::from_argb(background_colour as u32),
            text_alignment,
        )
    }
    /// Check if collisions between players' cameras and the specified player object is disabled.
    pub fn is_player_object_no_camera_collision(&self) -> bool {
        functions::PlayerObject_IsNoCameraCollision(&self.player, self)
    }

    /// Get player object's id
    pub fn get_id(&self) -> i32 {
        functions::PlayerObject_GetID(&self.player, self)
    }
}

/// Object moving information
#[repr(C)]
#[derive(Default, Clone, Copy, Debug)]
pub struct ObjectMoveData {
    /// The position the object moving to.
    pub targetPos: Vector3,
    /// The final rotation of the object.
    pub targetRot: Vector3,
    /// The speed at which to move the object (units per second).
    pub speed: f32,
}

/// The type of attachment attached to Object
#[repr(C)]
#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub enum ObjectAttachmentType {
    #[default]
    None,
    Vehicle,
    Object,
    Player,
}

/// Data information of attachment
#[repr(C)]
#[derive(Default, Clone, Copy, Debug)]
pub struct ObjectAttachmentData {
    /// Type of attachement
    pub attachment_type: ObjectAttachmentType,
    /// the sync rotation of the object
    pub syncRotation: bool,
    /// ID of the object (use from_id methods to create an instance of these)
    pub ID: i32,
    /// Attachement offset
    pub offset: Vector3,
    /// Attachement rotation
    pub rotation: Vector3,
}

/// Alignment of Object material text
#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ObjectMaterialTextAlign {
    Left,
    Center,
    Right,
}

/// Size of Object material
#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ObjectMaterialSize {
    Size32x32 = 10,
    Size64x32 = 20,
    Size64x64 = 30,
    Size128x32 = 40,
    Size128x64 = 50,
    Size128x128 = 60,
    Size256x32 = 70,
    Size256x64 = 80,
    Size256x128 = 90,
    Size256x256 = 100,
    Size512x64 = 110,
    Size512x128 = 120,
    Size512x256 = 130,
    Size512x512 = 140,
}

/// Kind of Object Material
#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ObjectMaterialType {
    None,
    Default,
    Text,
}

/// Object Material Data
#[derive(PartialEq, Clone, Debug)]
pub struct ObjectMaterialData {
    pub modelid: i32,
    pub textureLibrary: String,
    pub textureName: String,
    pub materialColour: Colour,
    pub text: String,
    pub materialSize: i32,
    pub fontFace: String,
    pub fontSize: i32,
    pub bold: bool,
    pub fontColour: Colour,
    pub backgroundColour: Colour,
    pub textAlignment: i32,
}

impl ObjectMaterialData {
    pub fn new(
        modelid: i32,
        textureLibrary: String,
        textureName: String,
        materialColour: Colour,
        text: String,
        materialSize: i32,
        fontFace: String,
        fontSize: i32,
        bold: bool,
        fontColour: Colour,
        backgroundColour: Colour,
        textAlignment: i32,
    ) -> Self {
        Self {
            modelid,
            textureLibrary,
            textureName,
            materialColour,
            text,
            materialSize,
            fontFace,
            fontSize,
            bold,
            fontColour,
            backgroundColour,
            textAlignment,
        }
    }
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ObjectEditResponse {
    Cancel,
    Final,
    Update,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ObjectAttachmentSlotData {
    pub model: i32,
    pub bone: i32,
    pub offset: Vector3,
    pub rotation: Vector3,
    pub scale: Vector3,
    pub colour1: Colour,
    pub colour2: Colour,
}
