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
        virtualWorld: i32,
        los: bool,
    ) -> Option<TextLabel> {
        let mut _id = 0;
        functions::TextLabel_Create(
            text,
            colour.rgba(),
            position.x,
            position.y,
            position.z,
            drawDistance,
            virtualWorld,
            los,
            &mut _id,
        )
    }
    /// Delete a 3D text label (created with Create3DTextLabel).
    pub fn delete(&self) -> bool {
        functions::TextLabel_Destroy(self)
    }
    /// Attach a 3D text label to a player.
    pub fn attach_to_player(&self, player: &Player, offset: Vector3) -> bool {
        functions::TextLabel_AttachToPlayer(self, player, offset.x, offset.y, offset.z)
    }
    /// Attaches a 3D Text Label to a specific vehicle.
    pub fn attach_to_vehicle(&self, vehicle: &Vehicle, offset: Vector3) -> bool {
        functions::TextLabel_AttachToVehicle(self, vehicle, offset.x, offset.y, offset.z)
    }
    /// Updates a 3D Text Label text and color.
    pub fn update_text(&self, colour: Colour, text: &str) -> bool {
        functions::TextLabel_UpdateText(self, colour.rgba(), text)
    }
    /// Checks if a 3D text label is streamed in for a player.
    pub fn is_streamed_in(&self, player: &Player) -> bool {
        functions::TextLabel_IsStreamedIn(player, self)
    }
    /// Gets the 3D text label text.
    pub fn get_text(&self) -> String {
        let mut output = String::new();
        functions::TextLabel_GetText(self, &mut output, 64);
        output
    }
    /// Gets the 3D text label color.
    pub fn get_color(&self) -> Colour {
        Colour::from_rgba(functions::TextLabel_GetColor(self))
    }
    /// Gets the 3D text label position.
    pub fn get_pos(&self) -> Vector3 {
        let mut out = Vector3::default();
        functions::TextLabel_GetPos(self, &mut out.x, &mut out.y, &mut out.z);
        out
    }
    /// Sets the 3D text label draw distance.
    pub fn set_draw_distance(&self, distance: f32) -> bool {
        functions::TextLabel_SetDrawDistance(self, distance)
    }
    /// Gets the 3D text label draw distance.
    pub fn get_draw_distance(&self) -> f32 {
        functions::TextLabel_GetDrawDistance(self)
    }
    /// Gets the 3D text label line-of-sight.
    pub fn get_los(&self) -> bool {
        functions::TextLabel_GetLOS(self)
    }
    /// Sets the 3D text label line-of-sight.
    pub fn set_los(&self, status: bool) -> bool {
        functions::TextLabel_SetLOS(self, status)
    }
    /// Gets the 3D text label virtual world id.
    pub fn get_virtual_world(&self) -> i32 {
        functions::TextLabel_GetVirtualWorld(self)
    }
    /// Sets the 3D text label virtual world id.
    pub fn set_virtual_world(&self, world: i32) -> bool {
        functions::TextLabel_SetVirtualWorld(self, world)
    }
    /// Gets the 3D text label attached data.
    pub fn get_attached_data(&self) -> TextLabelAttachmentData {
        let mut data = TextLabelAttachmentData::default();
        functions::TextLabel_GetAttachedData(self, &mut data.playerID, &mut data.vehicleID);
        data
    }
    pub fn get_id(&self) -> i32 {
        functions::TextLabel_GetID(self)
    }
    pub fn from_id(id: i32) -> Option<TextLabel> {
        functions::TextLabel_FromID(id)
    }
}

#[derive(Clone, Copy)]
pub struct PlayerTextLabel {
    handle: *const c_void,
    pub player: Player,
}

impl PlayerTextLabel {
    pub fn get_handle(&self) -> *const c_void {
        self.handle
    }
    pub fn new(handle: *const c_void, player: Player) -> Self {
        Self { handle, player }
    }

    /// Updates a player 3D Text Label's text and color.
    pub fn update_text(&self, colour: Colour, text: &str) -> bool {
        functions::PlayerTextLabel_UpdateText(&self.player, self, colour.rgba(), text)
    }
    /// Gets the player's 3D text label text.
    pub fn get_text(&self) -> String {
        let mut output = String::new();
        functions::PlayerTextLabel_GetText(&self.player, self, &mut output, 64);
        output
    }
    /// Gets the player's 3D text label color.
    pub fn get_color(&self) -> Colour {
        let mut colourint = 0;
        functions::PlayerTextLabel_GetColor(&self.player, self, &mut colourint);
        Colour::from_rgba(colourint)
    }
    /// Gets the player's 3D text label position.
    pub fn get_pos(&self) -> Vector3 {
        let mut out = Vector3::default();
        functions::PlayerTextLabel_GetPos(&self.player, self, &mut out.x, &mut out.y, &mut out.z);
        out
    }
    pub fn set_draw_distance(&self, distance: f32) -> bool {
        functions::PlayerTextLabel_SetDrawDistance(&self.player, self, distance)
    }
    /// Gets the player's 3D text label draw distance.
    pub fn get_draw_distance(&self) -> f32 {
        functions::PlayerTextLabel_GetDrawDistance(&self.player, self)
    }
    /// Gets the player's 3D text label line-of-sight.
    pub fn get_los(&self) -> bool {
        functions::PlayerTextLabel_GetLOS(&self.player, self)
    }
    pub fn set_los(&self, status: bool) -> bool {
        functions::PlayerTextLabel_SetLOS(&self.player, self, status)
    }
    /// Gets the player's 3D text label attached data.
    pub fn get_attached_data(&self) -> TextLabelAttachmentData {
        let mut data = TextLabelAttachmentData::default();
        functions::PlayerTextLabel_GetAttachedData(
            &self.player,
            self,
            &mut data.playerID,
            &mut data.vehicleID,
        );
        data
    }
    pub fn get_id(&self) -> i32 {
        functions::PlayerTextLabel_GetID(&self.player, self)
    }
    pub fn get_from_id(id: i32, player: &Player) -> Option<PlayerTextLabel> {
        functions::PlayerTextLabel_FromID(player, id)
    }
}

#[repr(C)]
#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub struct TextLabelAttachmentData {
    pub playerID: i32,
    pub vehicleID: i32,
}
