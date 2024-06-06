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
        modelid: isize,
        position: Vector3,
        rotation: Vector3,
        draw_distance: f32,
    ) -> Option<Object> {
        functions::CreateObject(modelid, position, rotation, draw_distance)
    }

    /// Destroys (removes) an object that was created using create method.
    pub fn destroy(&self) -> bool {
        functions::DestroyObject(self)
    }

    /// Attach an object to a vehicle.
    pub fn attach_to_vehicle(&self, vehicle: &Vehicle, offset: Vector3, rotation: Vector3) {
        functions::AttachObjectToVehicle(self, vehicle, offset, rotation)
    }

    /// Attach an object to an object.
    pub fn attach_to_object(
        &self,
        obj_attached_to: &Object,
        offset: Vector3,
        rotation: Vector3,
        sync_rotation: bool,
    ) {
        functions::AttachObjectToObject(self, obj_attached_to, offset, rotation, sync_rotation)
    }

    /// Attach a player object to a player.
    pub fn attach_to_player(&self, player: &Player, offset: Vector3, rotation: Vector3) {
        functions::AttachObjectToPlayer(self, player, offset, rotation)
    }

    ///  Set the position of an object.
    pub fn set_pos(&self, position: Vector3) {
        functions::SetObjectPos(self, position)
    }

    ///  Locate an object.
    pub fn get_pos(&self) -> Vector3 {
        let mut pos = Vector3::default();
        functions::GetObjectPos(self, &mut pos);
        pos
    }

    /// Set the rotation of an object.
    pub fn set_rotation(&self, rotation: Vector3) {
        functions::SetObjectRot(self, rotation)
    }

    /// Get the rotation of an object.
    pub fn get_rotation(&self) -> Vector3 {
        let mut rotation = Vector3::default();
        functions::GetObjectRot(self, &mut rotation);
        rotation
    }

    /// Get the model ID of an object
    pub fn get_model(&self) -> isize {
        functions::GetObjectModel(self)
    }

    /// Disable collisions between players' cameras and the specified object.
    pub fn set_no_camera_collision(&self) {
        functions::SetObjectNoCameraCol(self)
    }

    /// Move an object to a new position with a set speed. Players/vehicles will 'surf' the object as it moves.
    pub fn move_object(&self, data: ObjectMoveData) -> isize {
        functions::MoveObject(self, data)
    }

    /// Stop an object from moving.
    pub fn stop(&self) {
        functions::StopObject(self)
    }

    /// Check if the object is moving.
    pub fn is_moving(&self) -> bool {
        functions::IsObjectMoving(self)
    }

    /// Replace the texture of an object with the texture from another model in the game.
    pub fn set_material(
        &self,
        material_index: isize,
        model_id: isize,
        texture_library: &str,
        texture_name: &str,
        material_colour: Colour,
    ) {
        functions::SetObjectMaterial(
            self,
            material_index,
            model_id,
            texture_library,
            texture_name,
            material_colour,
        )
    }

    /// Replace the texture of an object with text.
    pub fn set_material_text(
        &self,
        text: &str,
        material_index: isize,
        material_size: ObjectMaterialSize,
        fontface: &str,
        fontsize: isize,
        bold: bool,
        font_colour: Colour,
        background_colour: Colour,
        textalignment: ObjectMaterialTextAlign,
    ) {
        functions::SetObjectMaterialText(
            self,
            text,
            material_index,
            material_size,
            fontface,
            fontsize,
            bold,
            font_colour,
            background_colour,
            textalignment,
        )
    }

    /// Allows camera collisions with newly created objects to be disabled by default.
    pub fn set_objects_default_camera_collision(disable: bool) {
        functions::SetObjectsDefaultCameraCol(disable)
    }

    /// Get the draw distance of an object.
    pub fn get_draw_distance(&self) -> f32 {
        functions::GetObjectDrawDistance(self)
    }

    /// Get the move speed of an object.
    pub fn get_move_speed(&self) -> f32 {
        functions::GetObjectMoveSpeed(self)
    }

    /// Get the move data of an object.
    pub fn get_move_data(&self) -> ObjectMoveData {
        let mut data = ObjectMoveData::default();
        functions::GetObjectMoveData(self, &mut data);
        data
    }

    /// Get the attachment data of an object.
    pub fn get_attached_data(&self) -> ObjectAttachmentData {
        let mut data = ObjectAttachmentData::default();
        functions::GetObjectAttachedData(self, &mut data);
        data
    }

    /// Checks if a slot of object material is used.
    pub fn is_material_slot_used(&self, material_index: isize) -> bool {
        functions::IsObjectMaterialSlotUsed(self, material_index)
    }

    /// Get object's material data
    pub fn get_material_data(&self, material_index: isize) -> ObjectMaterialData {
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
            isize,
            String,
            String,
            Colour,
            String,
            isize,
            String,
            isize,
            bool,
            Colour,
            Colour,
            isize,
        ) = Default::default();

        functions::GetObjectMaterialData(
            self,
            material_index,
            &mut modelid,
            &mut texture_library,
            &mut texture_name,
            &mut material_colour,
            &mut text,
            &mut material_size,
            &mut font_face,
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
            material_colour,
            text,
            material_size,
            font_face,
            font_size,
            bold,
            font_colour,
            background_colour,
            text_alignment,
        )
    }

    /// Check if collisions between players' cameras and the specified object is disabled.
    pub fn is_no_camera_collision(&self) -> bool {
        functions::IsObjectNoCameraCol(self)
    }

    /// Get object's id
    pub fn get_id(&self) -> isize {
        functions::GetObjectID(self)
    }

    /// Get Object from an id
    pub fn from_id(id: isize) -> Option<Object> {
        functions::GetObjectFromID(id)
    }
}

pub struct PlayerObject {
    handle: *const c_void,
}

impl PlayerObject {
    pub fn get_handle(&self) -> *const c_void {
        self.handle
    }

    pub fn new(handle: *const c_void) -> Self {
        Self { handle }
    }

    /// Attach a player object to a vehicle.
    pub fn attach_player_object_to_vehicle(
        &self,
        vehicle: &Vehicle,
        offset: Vector3,
        rotation: Vector3,
    ) {
        functions::AttachPlayerObjectToVehicle(self, vehicle, offset, rotation)
    }
    /// Attach PlayerObject to another player
    pub fn attach_player_object_to_player(
        &self,
        player_attached_to: &Player,
        offset: Vector3,
        rotation: Vector3,
    ) {
        functions::AttachPlayerObjectToPlayer(self, player_attached_to, offset, rotation)
    }
    /// You can use this function to attach player-objects to other player-objects.
    pub fn attach_player_object_to_object(
        &self,
        attached_to: &PlayerObject,
        offset: Vector3,
        rotation: Vector3,
    ) {
        functions::AttachPlayerObjectToObject(self, attached_to, offset, rotation)
    }
    /// Sets the position of a player-object to the specified coordinates.
    pub fn set_player_object_pos(&self, position: Vector3) {
        functions::SetPlayerObjectPos(self, position)
    }
    /// Get the position of a player object (CreatePlayerObject).
    pub fn get_player_object_pos(&self) -> Vector3 {
        let mut position = Vector3::default();
        functions::GetPlayerObjectPos(self, &mut position);
        position
    }
    /// Set the rotation of an object on the X, Y and Z axis.
    pub fn set_player_object_rotation(&self, rotation: Vector3) {
        functions::SetPlayerObjectRot(self, rotation)
    }
    /// Use this function to get the object's current rotation.
    pub fn get_player_object_rotation(&self) -> Vector3 {
        let mut rotation = Vector3::default();
        functions::GetPlayerObjectRot(self, &mut rotation);
        rotation
    }
    /// Retrieve the model ID of a player-object.
    pub fn get_player_object_model(&self) -> isize {
        functions::GetPlayerObjectModel(self)
    }
    /// Toggles a player object camera collision.
    pub fn set_player_object_no_camera_collision(&self) {
        functions::SetPlayerObjectNoCameraCol(self)
    }
    /// Move a player object with a set speed.
    pub fn move_player_object(&self, data: ObjectMoveData) -> isize {
        functions::MovePlayerObject(self, data)
    }
    /// Stop a moving player-object after MovePlayerObject has been used.
    pub fn stop_player_object(&self) {
        functions::StopPlayerObject(self)
    }
    /// Checks if the given player objectid is moving.
    pub fn is_player_object_moving(&self) -> bool {
        functions::IsPlayerObjectMoving(self)
    }

    /// Replace the texture of a player-object with the texture from another model in the game.
    pub fn set_player_object_material(
        &self,
        material_index: isize,
        model_id: isize,
        texture_library: &str,
        texture_name: &str,
        material_colour: Colour,
    ) {
        functions::SetPlayerObjectMaterial(
            self,
            material_index,
            model_id,
            texture_library,
            texture_name,
            material_colour,
        )
    }
    /// Replace the texture of a player object with text.
    pub fn set_player_object_material_text(
        &self,
        text: &str,
        material_index: isize,
        material_size: isize,
        fontface: &str,
        fontsize: isize,
        bold: bool,
        font_colour: Colour,
        background_colour: Colour,
        textalignment: ObjectMaterialTextAlign,
    ) {
        functions::SetPlayerObjectMaterialText(
            self,
            text,
            material_index,
            material_size,
            fontface,
            fontsize,
            bold,
            font_colour,
            background_colour,
            textalignment,
        )
    }
    /// Get the draw distance of a player-object.
    pub fn get_player_object_draw_distance(&self) -> f32 {
        functions::GetPlayerObjectDrawDistance(self)
    }
    /// Get the move speed of a player-object.
    pub fn get_player_object_move_speed(&self) -> f32 {
        functions::GetPlayerObjectMoveSpeed(self)
    }
    pub fn get_player_object_moving_data(&self) -> ObjectMoveData {
        let mut data = ObjectMoveData::default();
        functions::GetPlayerObjectMovingData(self, &mut data);
        data
    }
    /// Get the attachment data of a player-object.
    pub fn get_player_object_attached_data(&self) -> ObjectAttachmentData {
        let mut data = ObjectAttachmentData::default();
        functions::GetPlayerObjectAttachedData(self, &mut data);
        data
    }
    /// Checks if a slot of player-object material is used.
    pub fn is_player_object_material_slot_used(&self, material_index: isize) -> bool {
        functions::IsPlayerObjectMaterialSlotUsed(self, material_index)
    }
    pub fn get_player_object_material_data(&self, material_index: isize) -> ObjectMaterialData {
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
            isize,
            String,
            String,
            Colour,
            String,
            isize,
            String,
            isize,
            bool,
            Colour,
            Colour,
            isize,
        ) = Default::default();

        functions::GetPlayerObjectMaterialData(
            self,
            material_index,
            &mut modelid,
            &mut texture_library,
            &mut texture_name,
            &mut material_colour,
            &mut text,
            &mut material_size,
            &mut font_face,
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
            material_colour,
            text,
            material_size,
            font_face,
            font_size,
            bold,
            font_colour,
            background_colour,
            text_alignment,
        )
    }
    /// Check if collisions between players' cameras and the specified player object is disabled.
    pub fn is_player_object_no_camera_collision(&self) -> bool {
        functions::IsPlayerObjectNoCameraCol(self)
    }

    /// Get PlayerObject from an id
    pub fn get_player_object_id(&self) -> isize {
        functions::GetPlayerObjectID(self)
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
    pub ID: isize,
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
    pub modelid: isize,
    pub textureLibrary: String,
    pub textureName: String,
    pub materialColour: Colour,
    pub text: String,
    pub materialSize: isize,
    pub fontFace: String,
    pub fontSize: isize,
    pub bold: bool,
    pub fontColour: Colour,
    pub backgroundColour: Colour,
    pub textAlignment: isize,
}

impl ObjectMaterialData {
    pub fn new(
        modelid: isize,
        textureLibrary: String,
        textureName: String,
        materialColour: Colour,
        text: String,
        materialSize: isize,
        fontFace: String,
        fontSize: isize,
        bold: bool,
        fontColour: Colour,
        backgroundColour: Colour,
        textAlignment: isize,
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
#[derive(Clone, Copy, Debug)]
pub struct ObjectAttachmentSlotData {
    pub model: isize,
    pub bone: isize,
    pub offset: Vector3,
    pub rotation: Vector3,
    pub scale: Vector3,
    pub colour1: Colour,
    pub colour2: Colour,
}
