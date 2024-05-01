pub mod events;
pub mod functions;

use std::ffi::c_void;

use crate::{colour::Colour, players::Player, vector::Vector3, vehicles::Vehicle};

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

    pub fn create(
        modelid: isize,
        position: Vector3,
        rotation: Vector3,
        draw_distance: f32,
    ) -> Option<Object> {
        functions::CreateObject(modelid, position, rotation, draw_distance)
    }
    pub fn destroy(&self) -> bool {
        functions::DestroyObject(self)
    }
    pub fn attach_to_vehicle(&self, vehicle: &Vehicle, offset: Vector3, rotation: Vector3) {
        functions::AttachObjectToVehicle(self, vehicle, offset, rotation)
    }
    pub fn attach_to_object(
        &self,
        obj_attached_to: &Object,
        offset: Vector3,
        rotation: Vector3,
        sync_rotation: bool,
    ) {
        functions::AttachObjectToObject(self, obj_attached_to, offset, rotation, sync_rotation)
    }
    pub fn attach_to_player(&self, player: &Player, offset: Vector3, rotation: Vector3) {
        functions::AttachObjectToPlayer(self, player, offset, rotation)
    }
    pub fn set_pos(&self, position: Vector3) {
        functions::SetObjectPos(self, position)
    }
    pub fn get_pos(&self) -> Vector3 {
        let mut pos = Vector3::default();
        functions::GetObjectPos(self, &mut pos);
        pos
    }
    pub fn set_rotation(&self, rotation: Vector3) {
        functions::SetObjectRot(self, rotation)
    }
    pub fn get_rotation(&self) -> Vector3 {
        let mut rotation = Vector3::default();
        functions::GetObjectRot(self, &mut rotation);
        rotation
    }
    pub fn get_model(&self) -> isize {
        functions::GetObjectModel(self)
    }
    pub fn set_no_camera_collision(&self) {
        functions::SetObjectNoCameraCol(self)
    }
    pub fn is_valid(&self) -> bool {
        functions::IsValidObject(self)
    }
    pub fn move_object(&self, data: ObjectMoveData) -> isize {
        functions::MoveObject(self, data)
    }
    pub fn stop(&self) {
        functions::StopObject(self)
    }
    pub fn is_moving(&self) -> bool {
        functions::IsObjectMoving(self)
    }
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
    pub fn set_objects_default_camera_collision(disable: bool) {
        functions::SetObjectsDefaultCameraCol(disable)
    }
    pub fn get_draw_distance(&self) -> f32 {
        functions::GetObjectDrawDistance(self)
    }
    pub fn get_move_speed(&self) -> f32 {
        functions::GetObjectMoveSpeed(self)
    }
    pub fn get_move_data(&self) -> ObjectMoveData {
        let mut data = ObjectMoveData::default();
        functions::GetObjectMoveData(self, &mut data);
        data
    }
    pub fn get_attached_data(&self) -> ObjectAttachmentData {
        let mut data = ObjectAttachmentData::default();
        functions::GetObjectAttachedData(self, &mut data);
        data
    }
    pub fn is_material_slot_used(&self, material_index: isize) -> bool {
        functions::IsObjectMaterialSlotUsed(self, material_index)
    }
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
    pub fn is_no_camera_collision(&self) -> bool {
        functions::IsObjectNoCameraCol(self)
    }
    pub fn get_id(&self) -> isize {
        functions::GetObjectID(self)
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

    pub fn attach_player_object_to_vehicle(
        &self,
        vehicle: &Vehicle,
        offset: Vector3,
        rotation: Vector3,
    ) {
        functions::AttachPlayerObjectToVehicle(self, vehicle, offset, rotation)
    }
    pub fn attach_player_object_to_player(
        &self,
        player_attached_to: &Player,
        offset: Vector3,
        rotation: Vector3,
    ) {
        functions::AttachPlayerObjectToPlayer(self, player_attached_to, offset, rotation)
    }
    pub fn attach_player_object_to_object(
        &self,
        attached_to: &PlayerObject,
        offset: Vector3,
        rotation: Vector3,
    ) {
        functions::AttachPlayerObjectToObject(self, attached_to, offset, rotation)
    }
    pub fn set_player_object_pos(&self, position: Vector3) {
        functions::SetPlayerObjectPos(self, position)
    }
    pub fn get_player_object_pos(&self) -> Vector3 {
        let mut position = Vector3::default();
        functions::GetPlayerObjectPos(self, &mut position);
        position
    }
    pub fn set_player_object_rotation(&self, rotation: Vector3) {
        functions::SetPlayerObjectRot(self, rotation)
    }
    pub fn get_player_object_rotation(&self) -> Vector3 {
        let mut rotation = Vector3::default();
        functions::GetPlayerObjectRot(self, &mut rotation);
        rotation
    }
    pub fn get_player_object_model(&self) -> isize {
        functions::GetPlayerObjectModel(self)
    }
    pub fn set_player_object_no_camera_collision(&self) {
        functions::SetPlayerObjectNoCameraCol(self)
    }
    pub fn move_player_object(&self, data: ObjectMoveData) -> isize {
        functions::MovePlayerObject(self, data)
    }
    pub fn stop_player_object(&self) {
        functions::StopPlayerObject(self)
    }
    pub fn is_player_object_moving(&self) -> bool {
        functions::IsPlayerObjectMoving(self)
    }

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
    pub fn get_player_object_draw_distance(&self) -> f32 {
        functions::GetPlayerObjectDrawDistance(self)
    }
    pub fn get_player_object_move_speed(&self) -> f32 {
        functions::GetPlayerObjectMoveSpeed(self)
    }
    pub fn get_player_object_moving_data(&self) -> ObjectMoveData {
        let mut data = ObjectMoveData::default();
        functions::GetPlayerObjectMovingData(self, &mut data);
        data
    }
    pub fn get_player_object_attached_data(&self) -> ObjectAttachmentData {
        let mut data = ObjectAttachmentData::default();
        functions::GetPlayerObjectAttachedData(self, &mut data);
        data
    }
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
    pub fn is_player_object_no_camera_collision(&self) -> bool {
        functions::IsPlayerObjectNoCameraCol(self)
    }
    pub fn get_player_object_id(&self) -> isize {
        functions::GetPlayerObjectID(self)
    }
}

#[repr(C)]
#[derive(Default)]
pub struct ObjectMoveData {
    targetPos: Vector3,
    targetRot: Vector3,
    speed: f32,
}

#[repr(C)]
#[derive(Default)]
pub enum ObjectAttachmentType {
    #[default]
    None,
    Vehicle,
    Object,
    Player,
}

#[repr(C)]
#[derive(Default)]
pub struct ObjectAttachmentData {
    attachment_type: ObjectAttachmentType,
    syncRotation: bool,
    ID: isize,
    offset: Vector3,
    rotation: Vector3,
}

#[repr(C)]
pub enum ObjectMaterialTextAlign {
    Left,
    Center,
    Right,
}

#[repr(C)]
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

pub enum ObjectMaterialType {
    None,
    Default,
    Text,
}

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
pub enum ObjectEditResponse {
    Cancel,
    Final,
    Update,
}

#[repr(C)]
pub struct ObjectAttachmentSlotData {
    model: isize,
    bone: isize,
    offset: Vector3,
    rotation: Vector3,
    scale: Vector3,
    colour1: Colour,
    colour2: Colour,
}
