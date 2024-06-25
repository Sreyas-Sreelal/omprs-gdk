use std::ffi::c_void;

pub mod events;
pub mod functions;

pub use functions::load_functions;

use crate::{
    players::Player,
    types::colour::Colour,
    types::vector::{Vector2, Vector3},
};

pub struct TextDraw {
    handle: *const c_void,
}

#[deny(non_snake_case)]
impl TextDraw {
    pub fn get_handle(&self) -> *const c_void {
        self.handle
    }
    pub fn new(handle: *const c_void) -> Self {
        Self { handle }
    }
    /// Creates a textdraw.
    pub fn create(position: Vector2, text: &str) -> Option<TextDraw> {
        let mut _id = -1;

        functions::TextDraw_Create(position.x, position.y, text, &mut _id)
    }
    /// Destroys a previously-created textdraw.
    pub fn destroy(&self) -> bool {
        functions::TextDraw_Destroy(self)
    }
    pub fn is_shown_for_player(&self, player: &Player) -> bool {
        functions::TextDraw_IsVisibleForPlayer(player, self)
    }
    pub fn set_letter_size(&self, size: Vector2) -> bool {
        functions::TextDraw_SetLetterSize(self, size.x, size.y)
    }
    pub fn set_text_size(&self, size: Vector2) -> bool {
        functions::TextDraw_SetTextSize(self, size.x, size.y)
    }
    pub fn set_alignment(&self, alignment: TextDrawAlignmentTypes) -> bool {
        functions::TextDraw_SetAlignment(self, alignment as i32)
    }
    pub fn set_color(&self, colour: Colour) -> bool {
        functions::TextDraw_SetColor(self, colour.rgba())
    }
    /// Toggle whether a textdraw uses a box or not.
    pub fn use_box(&self, use_box: bool) -> bool {
        functions::TextDraw_SetUseBox(self, use_box)
    }
    pub fn set_box_color(&self, colour: Colour) -> bool {
        functions::TextDraw_SetBoxColor(self, colour.rgba())
    }
    /// Sets the size of a textdraw's text's shadow.
    pub fn set_shadow(&self, size: i32) -> bool {
        functions::TextDraw_SetShadow(self, size)
    }
    /// Sets the thickness of a textdraw's text's outline.
    pub fn set_outline(&self, size: i32) -> bool {
        functions::TextDraw_SetOutline(self, size)
    }
    pub fn set_background_color(&self, colour: Colour) -> bool {
        functions::TextDraw_SetBackgroundColor(self, colour.rgba())
    }
    pub fn set_style(&self, font: TextDrawStyle) -> bool {
        functions::TextDraw_SetFont(self, font as i32)
    }
    /// Appears to scale text spacing to a proportional ratio.
    pub fn set_proportional(&self, set: bool) -> bool {
        functions::TextDraw_SetSetProportional(self, set)
    }
    /// Sets whether a textdraw can be selected (clicked on) or not.
    pub fn set_selectable(&self, set: bool) -> bool {
        functions::TextDraw_SetSelectable(self, set)
    }
    /// Shows a textdraw for a specific player.
    pub fn show_for_player(&self, player: &Player) -> bool {
        functions::TextDraw_ShowForPlayer(player, self)
    }
    /// Hides a textdraw for a specific player.
    pub fn hide_for_player(&self, player: &Player) -> bool {
        functions::TextDraw_HideForPlayer(player, self)
    }
    /// Shows a textdraw for all players.
    pub fn show_for_all(&self) -> bool {
        functions::TextDraw_ShowForAll(self)
    }
    /// Hides a text draw for all players.
    pub fn hide_for_all(&self) -> bool {
        functions::TextDraw_HideForAll(self)
    }
    /// Changes the text on a textdraw.
    pub fn set_string(&self, text: &str) -> bool {
        functions::TextDraw_SetString(self, text)
    }
    /// Set the model for a textdraw model preview.
    pub fn set_preview_model(&self, model: i32) -> bool {
        functions::TextDraw_SetPreviewModel(self, model)
    }
    pub fn set_preview_rotation(&self, rotation: Vector3, zoom: f32) -> bool {
        functions::TextDraw_SetPreviewRot(self, rotation.x, rotation.y, rotation.z, zoom)
    }
    pub fn set_preview_veh_colour(&self, colour1: i32, colour2: i32) -> bool {
        functions::TextDraw_SetPreviewVehCol(self, colour1, colour2)
    }
    /// Sets the position of a textdraw.
    pub fn set_pos(&self, pos: Vector2) -> bool {
        functions::TextDraw_SetPos(self, pos.x, pos.y)
    }
    /// Gets the text of a textdraw.
    pub fn get_string(&self) -> String {
        let mut text = String::new();
        functions::TextDraw_GetString(self, &mut text);
        text
    }
    /// Gets the width and height of the letters.
    pub fn get_letter_size(&self) -> Vector2 {
        let mut size = Vector2::default();
        functions::TextDraw_GetLetterSize(self, &mut size.x, &mut size.y);
        size
    }
    /// Gets the X axis and Y axis of the textdraw.
    pub fn get_text_size(&self) -> Vector2 {
        let mut size = Vector2::default();
        functions::TextDraw_GetTextSize(self, &mut size.x, &mut size.y);
        size
    }
    /// Gets the position of a textdraw.
    pub fn get_pos(&self) -> Vector2 {
        let mut pos = Vector2::default();
        functions::TextDraw_GetPos(self, &mut pos.x, &mut pos.y);
        pos
    }
    /// Gets the text color of a textdraw.
    pub fn get_color(&self) -> Colour {
        Colour::from_rgba(functions::TextDraw_GetColor(self) as u32)
    }
    /// Gets the box color of a textdraw.
    pub fn get_box_color(&self) -> Colour {
        Colour::from_rgba(functions::TextDraw_GetBoxColor(self) as u32)
    }
    /// Gets the background color of a textdraw.
    pub fn get_background_color(&self) -> Colour {
        Colour::from_rgba(functions::TextDraw_GetBackgroundColor(self) as u32)
    }
    /// Gets the size of a textdraw's text's shadow.
    pub fn get_shadow(&self) -> i32 {
        functions::TextDraw_GetShadow(self)
    }
    /// Gets the thickness of a textdraw's text's outline.
    pub fn get_outline(&self) -> i32 {
        functions::TextDraw_GetOutline(self)
    }
    pub fn get_style(&self) -> i32 {
        functions::TextDraw_GetFont(self)
    }
    /// Checks if a textdraw is box.
    pub fn is_box(&self) -> bool {
        functions::TextDraw_IsBox(self)
    }
    /// Checks if a textdraw is proportional.
    pub fn is_proportional(&self) -> bool {
        functions::TextDraw_IsProportional(self)
    }
    /// Checks if a textdraw is selectable.
    pub fn is_selectable(&self) -> bool {
        functions::TextDraw_IsSelectable(self)
    }
    /// Gets the text alignment of a textdraw.
    pub fn get_alignment(&self) -> TextDrawAlignmentTypes {
        unsafe { std::mem::transmute(functions::TextDraw_GetAlignment(self)) }
    }
    /// Gets the preview model of a 3D preview textdraw.
    pub fn get_preview_model(&self) -> i32 {
        functions::TextDraw_GetPreviewModel(self)
    }
    pub fn get_preview_rotation(&self) -> (Vector3, f32) {
        let mut rotation = Vector3::default();
        let mut zoom = 0.0;
        functions::TextDraw_GetPreviewRot(
            self,
            &mut rotation.x,
            &mut rotation.y,
            &mut rotation.z,
            &mut zoom,
        );
        (rotation, zoom)
    }
    pub fn get_preview_veh_colour(&self) -> (i32, i32) {
        let mut colour1 = 0;
        let mut colour2 = 0;
        functions::TextDraw_GetPreviewVehColor(self, &mut colour1, &mut colour2);
        (colour1, colour2)
    }
    /// Changes the text on a textdraw for a specific player.
    pub fn set_string_for_player(&self, player: &Player, text: &str) -> bool {
        functions::TextDraw_SetStringForPlayer(self, player, text)
    }
    pub fn get_id(&self) -> i32 {
        functions::TextDraw_GetID(self)
    }
    pub fn from_id(id: i32) -> Option<TextDraw> {
        functions::TextDraw_FromID(id)
    }
}

pub struct PlayerTextDraw {
    handle: *const c_void,
    pub player: Player,
}

impl PlayerTextDraw {
    pub fn get_handle(&self) -> *const c_void {
        self.handle
    }
    pub fn new(handle: *const c_void, player: Player) -> Self {
        Self { handle, player }
    }
    pub fn is_shown(&self) -> bool {
        functions::PlayerTextDraw_IsVisible(&self.player, self)
    }
    pub fn set_letter_size(&self, size: Vector2) -> bool {
        functions::PlayerTextDraw_SetLetterSize(&self.player, self, size.x, size.y)
    }
    pub fn set_text_size(&self, size: Vector2) -> bool {
        functions::PlayerTextDraw_SetTextSize(&self.player, self, size.x, size.y)
    }
    /// Set the text alignment of a player-textdraw.
    pub fn alignment(&self, alignment: TextDrawAlignmentTypes) -> bool {
        functions::PlayerTextDraw_SetAlignment(&self.player, self, alignment as i32)
    }
    /// Sets the text color of a player-textdraw.
    pub fn color(&self, colour: Colour) -> bool {
        functions::PlayerTextDraw_SetColor(&self.player, self, colour.rgba())
    }
    /// Toggle the box on a player-textdraw.
    pub fn use_box(&self, box_use: bool) -> bool {
        functions::PlayerTextDraw_UseBox(&self.player, self, box_use)
    }
    pub fn set_box_color(&self, colour: Colour) -> bool {
        functions::PlayerTextDraw_SetBoxColor(&self.player, self, colour.rgba())
    }
    /// Adds a shadow to the bottom-right side of the text in a player-textdraw.
    pub fn set_shadow(&self, size: i32) -> bool {
        functions::PlayerTextDraw_SetShadow(&self.player, self, size)
    }
    /// Set the outline of a player-textdraw.
    pub fn set_outline(&self, size: i32) -> bool {
        functions::PlayerTextDraw_SetOutline(&self.player, self, size)
    }
    /// Adjust the background color of a player-textdraw.
    pub fn background_color(&self, colour: Colour) -> bool {
        functions::PlayerTextDraw_SetBackgroundColor(&self.player, self, colour.rgba())
    }
    pub fn set_style(&self, font: TextDrawStyle) -> bool {
        functions::PlayerTextDraw_SetFont(&self.player, self, font as i32)
    }
    /// Appears to scale text spacing to a proportional ratio.
    pub fn set_proportional(&self, set: bool) -> bool {
        functions::PlayerTextDraw_SetProportional(&self.player, self, set)
    }
    /// Toggles whether a player-textdraw can be selected or not.
    pub fn set_selectable(&self, set: bool) -> bool {
        functions::PlayerTextDraw_SetSelectable(&self.player, self, set)
    }
    /// Show a player-textdraw to the player it was created for.
    pub fn show(&self) -> bool {
        functions::PlayerTextDraw_Show(&self.player, self)
    }
    /// Hide a player-textdraw from the player it was created for.
    pub fn hide(&self) -> bool {
        functions::PlayerTextDraw_Hide(&self.player, self)
    }
    /// Change the text of a player-textdraw.
    pub fn set_string(&self, text: &str) -> bool {
        functions::PlayerTextDraw_SetString(&self.player, self, text)
    }
    /// Sets a player textdraw 3D preview sprite of a specified model ID.
    pub fn set_preview_model(&self, model: i32) -> bool {
        functions::PlayerTextDraw_SetPreviewModel(&self.player, self, model)
    }
    pub fn set_preview_rotation(&self, rotation: Vector3, zoom: f32) -> bool {
        functions::PlayerTextDraw_SetPreviewRot(
            &self.player,
            self,
            rotation.x,
            rotation.y,
            rotation.z,
            zoom,
        )
    }
    pub fn set_preview_veh_colour(&self, colour1: i32, colour2: i32) -> bool {
        functions::PlayerTextDraw_SetPreviewVehCol(&self.player, self, colour1, colour2)
    }
    /// Sets the position of a player-textdraw.
    pub fn set_pos(&self, pos: Vector2) -> bool {
        functions::PlayerTextDraw_SetPos(&self.player, self, pos.x, pos.y)
    }
    /// Gets the text of a player-textdraw.
    pub fn get_string(&self) -> String {
        let mut text = String::new();
        functions::PlayerTextDraw_GetString(&self.player, self, &mut text);
        text
    }
    /// Gets the width and height of the letters.
    pub fn get_letter_size(&self) -> Vector2 {
        let mut size = Vector2::default();
        functions::PlayerTextDraw_GetLetterSize(&self.player, self, &mut size.x, &mut size.y);
        size
    }
    /// Gets the X axis and Y axis of the player-textdraw text size.
    pub fn get_text_size(&self) -> Vector2 {
        let mut size = Vector2::default();
        functions::PlayerTextDraw_GetTextSize(&self.player, self, &mut size.x, &mut size.y);
        size
    }
    /// Gets the position of a player-textdraw.
    pub fn get_pos(&self) -> Vector2 {
        let mut pos = Vector2::default();
        functions::PlayerTextDraw_GetPos(&self.player, self, &mut pos.x, &mut pos.y);
        pos
    }
    /// Gets the text color of a player-textdraw
    pub fn get_color(&self) -> Colour {
        Colour::from_rgba(functions::PlayerTextDraw_GetColor(&self.player, self) as u32)
    }
    /// Gets the box color of a player-textdraw
    pub fn get_box_color(&self) -> Colour {
        Colour::from_rgba(functions::PlayerTextDraw_GetBoxColor(&self.player, self) as u32)
    }
    /// Gets the background colour of a player-textdraw
    pub fn get_background_colour(&self) -> Colour {
        Colour::from_rgba(functions::PlayerTextDraw_GetBackgroundColor(&self.player, self) as u32)
    }
    /// Get the shadow size on a player-textdraw.
    pub fn get_shadow(&self) -> i32 {
        functions::PlayerTextDraw_GetShadow(&self.player, self)
    }
    /// Get the outline size on a player-textdraw.
    pub fn get_outline(&self) -> i32 {
        functions::PlayerTextDraw_GetOutline(&self.player, self)
    }
    pub fn get_style(&self) -> i32 {
        functions::PlayerTextDraw_GetFont(&self.player, self)
    }
    /// Checks if a player-textdraw is box.
    pub fn is_box(&self) -> bool {
        functions::PlayerTextDraw_IsBox(&self.player, self)
    }
    /// Checks if a player-textdraw is proportional.
    pub fn is_proportional(&self) -> bool {
        functions::PlayerTextDraw_IsProportional(&self.player, self)
    }
    /// Checks if a player-textdraw is selectable.
    pub fn is_selectable(&self) -> bool {
        functions::PlayerTextDraw_IsSelectable(&self.player, self)
    }
    /// Gets the text alignment of a player-textdraw.
    pub fn get_alignment(&self) -> TextDrawAlignmentTypes {
        unsafe { std::mem::transmute(functions::PlayerTextDraw_GetAlignment(&self.player, self)) }
    }
    /// Gets the preview model of a 3D preview player-textdraw.
    pub fn get_preview_model(&self) -> i32 {
        functions::PlayerTextDraw_GetPreviewModel(&self.player, self)
    }
    pub fn get_preview_rotation(&self) -> (Vector3, f32) {
        let mut rotation = Vector3::default();
        let mut zoom = 0.0;
        functions::PlayerTextDraw_GetPreviewRot(
            &self.player,
            self,
            &mut rotation.x,
            &mut rotation.y,
            &mut rotation.z,
            &mut zoom,
        );
        (rotation, zoom)
    }
    pub fn get_preview_veh_colour(&self) -> (i32, i32) {
        let mut colour1 = 0;
        let mut colour2 = 0;
        functions::PlayerTextDraw_GetPreviewVehColor(
            &self.player,
            self,
            &mut colour1,
            &mut colour2,
        );
        (colour1, colour2)
    }
    pub fn get_id(&self) -> i32 {
        functions::PlayerTextDraw_GetID(&self.player, self)
    }
    pub fn from_id(selfid: i32, player: &Player) -> Option<PlayerTextDraw> {
        functions::PlayerTextDraw_FromID(player, selfid)
    }
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug, Default)]
pub enum TextDrawAlignmentTypes {
    #[default]
    Default,
    Left,
    Center,
    Right,
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TextDrawStyle {
    FontBeckettRegular = 0,
    FontAharoniBold,
    FontBankGothic,
    FontPricedown,
    Sprite,
    Preview,
}
