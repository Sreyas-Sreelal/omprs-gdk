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
    /// Creates a 3D Text Label at a specific location in the world.
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
    /// Delete a 3D text label (created with Create3DTextLabel).
    pub fn delete(&self) {
        functions::Delete3DTextLabel(self)
    }
    /// Attach a 3D text label to a player.
    pub fn attach_to_player(&self, player: &Player, offset: Vector3) {
        functions::Attach3DTextLabelToPlayer(self, player, offset)
    }
    /// Attaches a 3D Text Label to a specific vehicle.
    pub fn attach_to_vehicle(&self, vehicle: &Vehicle, offset: Vector3) {
        functions::Attach3DTextLabelToVehicle(self, vehicle, offset)
    }
    /// Updates a 3D Text Label text and color.
    pub fn update_text(&self, colour: Colour, text: &str) {
        functions::Update3DTextLabelText(self, colour, text)
    }
    /// Checks if a 3D text label is streamed in for a player.
    pub fn is_streamed_in(&self, player: &Player) -> bool {
        functions::Is3DTextLabelStreamedIn(self, player)
    }
    /// Gets the 3D text label text.
    pub fn get_text(&self) -> String {
        let mut output = String::new();
        functions::Get3DTextLabelText(self, &mut output);
        output
    }
    /// Gets the 3D text label color.
    pub fn get_color(&self) -> Colour {
        let mut colour = Colour::default();
        functions::Get3DTextLabelColor(self, &mut colour);
        colour
    }
    /// Gets the 3D text label position.
    pub fn get_pos(&self) -> Vector3 {
        let mut out = Vector3::default();
        functions::Get3DTextLabelPos(self, &mut out);
        out
    }
    /// Sets the 3D text label draw distance.
    pub fn set_draw_distance(&self, distance: f32) {
        functions::Set3DTextLabelDrawDistance(self, distance)
    }
    /// Gets the 3D text label draw distance.
    pub fn get_draw_distance(&self) -> f32 {
        functions::Get3DTextLabelDrawDistance(self)
    }
    /// Gets the 3D text label line-of-sight.
    pub fn get_los(&self) -> bool {
        functions::Get3DTextLabelLOS(self)
    }
    /// Sets the 3D text label line-of-sight.
    pub fn set_los(&self, status: bool) {
        functions::Set3DTextLabelLOS(self, status)
    }
    /// Gets the 3D text label virtual world id.
    pub fn get_virtual_world(&self) -> isize {
        functions::Get3DTextLabelVirtualWorld(self)
    }
    /// Sets the 3D text label virtual world id.
    pub fn set_virtual_world(&self, world: isize) {
        functions::Set3DTextLabelVirtualWorld(self, world)
    }
    /// Gets the 3D text label attached data.
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

    /// Updates a player 3D Text Label's text and color.
    pub fn update_text(&self, colour: Colour, text: &str) {
        functions::UpdatePlayer3DTextLabelText(self, colour, text)
    }
    /// Gets the player's 3D text label text.
    pub fn get_text(&self) -> String {
        let mut output = String::new();
        functions::GetPlayer3DTextLabelText(self, &mut output);
        output
    }
    /// Gets the player's 3D text label color.
    pub fn get_color(&self) -> Colour {
        let mut colour = Colour::default();
        functions::GetPlayer3DTextLabelColor(self, &mut colour);
        colour
    }
    /// Gets the player's 3D text label position.
    pub fn get_pos(&self) -> Vector3 {
        let mut out = Vector3::default();
        functions::GetPlayer3DTextLabelPos(self, &mut out);
        out
    }
    pub fn set_draw_distance(&self, distance: f32) {
        functions::SetPlayer3DTextLabelDrawDistance(self, distance)
    }
    /// Gets the player's 3D text label draw distance.
    pub fn get_draw_distance(&self) -> f32 {
        functions::GetPlayer3DTextLabelDrawDistance(self)
    }
    /// Gets the player's 3D text label line-of-sight.
    pub fn get_los(&self) -> bool {
        functions::GetPlayer3DTextLabelLOS(self)
    }
    pub fn set_los(&self, status: bool) {
        functions::SetPlayer3DTextLabelLOS(self, status)
    }
    /// Gets the player's 3D text label attached data.
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
#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub struct TextLabelAttachmentData {
    pub playerID: isize,
    pub vehicleID: isize,
}
