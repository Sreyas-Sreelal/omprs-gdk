pub mod functions;

use std::ffi::c_void;

use crate::{players::Player, types::colour::Colour, types::vector::Vector3, vehicles::Vehicle};

pub use functions::load_functions;

pub struct TextLabel {
    handle: *const c_void,
}

impl TextLabel {
    pub fn get_handle(&self) -> *const c_void {
        self.handle
    }
    pub fn new(handle: *const c_void) -> Self {
        Self { handle }
    }
    pub fn create(
        text: &str,
        colour: Colour,
        position: Vector3,
        drawDistance: f32,
        virtualWorld: isize,
        los: bool,
    ) -> Option<TextLabel> {
        functions::Create3DTextLabel(text, colour, position, drawDistance, virtualWorld, los)
    }
    pub fn delete(&self) {
        functions::Delete3DTextLabel(self)
    }
    pub fn attach_to_player(&self, player: &Player, offset: Vector3) {
        functions::Attach3DTextLabelToPlayer(self, player, offset)
    }
    pub fn attach_to_vehicle(&self, vehicle: &Vehicle, offset: Vector3) {
        functions::Attach3DTextLabelToVehicle(self, vehicle, offset)
    }
    pub fn update_text(&self, colour: Colour, text: &str) {
        functions::Update3DTextLabelText(self, colour, text)
    }
    pub fn is_streamed_in(&self, player: &Player) -> bool {
        functions::Is3DTextLabelStreamedIn(self, player)
    }
    pub fn get_text(&self) -> String {
        let mut output = String::new();
        functions::Get3DTextLabelText(self, &mut output);
        output
    }
    pub fn get_color(&self) -> Colour {
        let mut colour = Colour::default();
        functions::Get3DTextLabelColor(self, &mut colour);
        colour
    }
    pub fn get_pos(&self) -> Vector3 {
        let mut out = Vector3::default();
        functions::Get3DTextLabelPos(self, &mut out);
        out
    }
    pub fn set_draw_distance(&self, distance: f32) {
        functions::Set3DTextLabelDrawDistance(self, distance)
    }
    pub fn get_draw_distance(&self) -> f32 {
        functions::Get3DTextLabelDrawDistance(self)
    }
    pub fn get_los(&self) -> bool {
        functions::Get3DTextLabelLOS(self)
    }
    pub fn set_los(&self, status: bool) {
        functions::Set3DTextLabelLOS(self, status)
    }
    pub fn get_virtual_world(&self) -> isize {
        functions::Get3DTextLabelVirtualWorld(self)
    }
    pub fn set_virtual_world(&self, world: isize) {
        functions::Set3DTextLabelVirtualWorld(self, world)
    }
    pub fn get_attached_data(&self) -> TextLabelAttachmentData {
        let mut data = TextLabelAttachmentData::default();
        functions::Get3DTextLabelAttachedData(self, &mut data);
        data
    }
    pub fn get_id(&self) -> isize {
        functions::Get3DTextLabelID(self)
    }
    pub fn from_id(id: isize) -> Option<TextLabel> {
        functions::Get3DTextLabelFromID(id)
    }
}

pub struct PlayerTextLabel {
    handle: *const c_void,
}

impl PlayerTextLabel {
    pub fn get_handle(&self) -> *const c_void {
        self.handle
    }
    pub fn new(handle: *const c_void) -> Self {
        Self { handle }
    }

    pub fn update_text(&self, colour: Colour, text: &str) {
        functions::UpdatePlayer3DTextLabelText(self, colour, text)
    }
    pub fn get_text(&self) -> String {
        let mut output = String::new();
        functions::GetPlayer3DTextLabelText(self, &mut output);
        output
    }
    pub fn get_color(&self) -> Colour {
        let mut colour = Colour::default();
        functions::GetPlayer3DTextLabelColor(self, &mut colour);
        colour
    }
    pub fn get_pos(&self) -> Vector3 {
        let mut out = Vector3::default();
        functions::GetPlayer3DTextLabelPos(self, &mut out);
        out
    }
    pub fn set_draw_distance(&self, distance: f32) {
        functions::SetPlayer3DTextLabelDrawDistance(self, distance)
    }
    pub fn get_draw_distance(&self) -> f32 {
        functions::GetPlayer3DTextLabelDrawDistance(self)
    }
    pub fn get_los(&self) -> bool {
        functions::GetPlayer3DTextLabelLOS(self)
    }
    pub fn set_los(&self, status: bool) {
        functions::SetPlayer3DTextLabelLOS(self, status)
    }
    pub fn get_attached_data(&self) -> TextLabelAttachmentData {
        let mut data = TextLabelAttachmentData::default();
        functions::GetPlayer3DTextLabelAttachedData(self, &mut data);
        data
    }
    pub fn get_id(&self) -> isize {
        functions::GetPlayer3DTextLabelID(self)
    }
    pub fn get_from_id(id: isize, player: &Player) -> Option<PlayerTextLabel> {
        functions::GetPlayer3DTextLabelFromID(player, id)
    }
}

#[repr(C)]
#[derive(Default)]
pub struct TextLabelAttachmentData {
    playerID: isize,
    vehicleID: isize,
}
