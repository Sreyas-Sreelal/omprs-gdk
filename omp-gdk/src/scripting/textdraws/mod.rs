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
        functions::TextDrawCreate(position, text)
    }
    /// Destroys a previously-created textdraw.
    pub fn destroy(&self) {
        functions::TextDrawDestroy(self)
    }
    pub fn is_shown_for_player(&self, player: &Player) -> bool {
        functions::IsTextDrawShownForPlayer(self, player)
    }
    pub fn set_letter_size(&self, size: Vector2) {
        functions::TextDrawSetLetterSize(self, size)
    }
    pub fn set_text_size(&self, size: Vector2) {
        functions::TextDrawSetTextSize(self, size)
    }
    pub fn set_alignment(&self, alignment: TextDrawAlignmentTypes) {
        functions::TextDrawSetAlignment(self, alignment)
    }
    pub fn set_color(&self, colour: Colour) {
        functions::TextDrawSetColor(self, colour)
    }
    /// Toggle whether a textdraw uses a box or not.
    pub fn use_box(&self, use_box: bool) {
        functions::TextDrawUseBox(self, use_box)
    }
    pub fn set_box_color(&self, colour: Colour) {
        functions::TextDrawSetBoxColor(self, colour)
    }
    /// Sets the size of a textdraw's text's shadow.
    pub fn set_shadow(&self, size: isize) {
        functions::TextDrawSetShadow(self, size)
    }
    /// Sets the thickness of a textdraw's text's outline.
    pub fn set_outline(&self, size: isize) {
        functions::TextDrawSetOutline(self, size)
    }
    pub fn set_background_color(&self, colour: Colour) {
        functions::TextDrawSetBackgroundColor(self, colour)
    }
    pub fn set_style(&self, font: TextDrawStyle) {
        functions::TextDrawSetStyle(self, font)
    }
    /// Appears to scale text spacing to a proportional ratio.
    pub fn set_proportional(&self, set: bool) {
        functions::TextDrawSetProportional(self, set)
    }
    /// Sets whether a textdraw can be selected (clicked on) or not.
    pub fn set_selectable(&self, set: bool) {
        functions::TextDrawSetSelectable(self, set)
    }
    /// Shows a textdraw for a specific player.
    pub fn show_for_player(&self, player: &Player) {
        functions::TextDrawShowForPlayer(self, player)
    }
    /// Hides a textdraw for a specific player.
    pub fn hide_for_player(&self, player: &Player) {
        functions::TextDrawHideForPlayer(self, player)
    }
    /// Shows a textdraw for all players.
    pub fn show_for_all(&self) {
        functions::TextDrawShowForAll(self)
    }
    /// Hides a text draw for all players.
    pub fn hide_for_all(&self) {
        functions::TextDrawHideForAll(self)
    }
    /// Changes the text on a textdraw.
    pub fn set_string(&self, text: &str) {
        functions::TextDrawSetString(self, text)
    }
    /// Set the model for a textdraw model preview.
    pub fn set_preview_model(&self, model: isize) {
        functions::TextDrawSetPreviewModel(self, model)
    }
    pub fn set_preview_rotation(&self, rotation: Vector3, zoom: f32) {
        functions::TextDrawSetPreviewRotation(self, rotation, zoom)
    }
    pub fn set_preview_veh_colour(&self, colour1: isize, colour2: isize) {
        functions::TextDrawSetPreviewVehColour(self, colour1, colour2)
    }
    /// Sets the position of a textdraw.
    pub fn set_pos(&self, pos: Vector2) {
        functions::TextDrawSetPos(self, pos)
    }
    /// Gets the text of a textdraw.
    pub fn get_string(&self) -> String {
        let mut text = String::new();
        functions::TextDrawGetString(self, &mut text);
        text
    }
    /// Gets the width and height of the letters.
    pub fn get_letter_size(&self) -> Vector2 {
        let mut size = Vector2::default();
        functions::TextDrawGetLetterSize(self, &mut size);
        size
    }
    /// Gets the X axis and Y axis of the textdraw.
    pub fn get_text_size(&self) -> Vector2 {
        let mut size = Vector2::default();
        functions::TextDrawGetTextSize(self, &mut size);
        size
    }
    /// Gets the position of a textdraw.
    pub fn get_pos(&self) -> Vector2 {
        let mut pos = Vector2::default();
        functions::TextDrawGetPos(self, &mut pos);
        pos
    }
    /// Gets the text color of a textdraw.
    pub fn get_color(&self) -> Colour {
        let mut colour = Colour::default();
        functions::TextDrawGetColor(self, &mut colour);
        colour
    }
    /// Gets the box color of a textdraw.
    pub fn get_box_color(&self) -> Colour {
        let mut colour = Colour::default();
        functions::TextDrawGetBoxColor(self, &mut colour);
        colour
    }
    /// Gets the background color of a textdraw.
    pub fn get_background_color(&self) -> Colour {
        let mut colour = Colour::default();
        functions::TextDrawGetBackgroundColor(self, &mut colour);
        colour
    }
    /// Gets the size of a textdraw's text's shadow.
    pub fn get_shadow(&self) -> isize {
        functions::TextDrawGetShadow(self)
    }
    /// Gets the thickness of a textdraw's text's outline.
    pub fn get_outline(&self) -> isize {
        functions::TextDrawGetOutline(self)
    }
    pub fn get_style(&self) -> isize {
        functions::TextDrawGetStyle(self)
    }
    /// Checks if a textdraw is box.
    pub fn is_box(&self) -> bool {
        functions::TextDrawIsBox(self)
    }
    /// Checks if a textdraw is proportional.
    pub fn is_proportional(&self) -> bool {
        functions::TextDrawIsProportional(self)
    }
    /// Checks if a textdraw is selectable.
    pub fn is_selectable(&self) -> bool {
        functions::TextDrawIsSelectable(self)
    }
    /// Gets the text alignment of a textdraw.
    pub fn get_alignment(&self) -> TextDrawAlignmentTypes {
        functions::TextDrawGetAlignment(self)
    }
    /// Gets the preview model of a 3D preview textdraw.
    pub fn get_preview_model(&self) -> isize {
        functions::TextDrawGetPreviewModel(self)
    }
    pub fn get_preview_rotation(&self) -> (Vector3, f32) {
        let mut rotation = Vector3::default();
        let mut zoom = 0.0;
        functions::TextDrawGetPreviewRotation(self, &mut rotation, &mut zoom);
        (rotation, zoom)
    }
    pub fn get_preview_veh_colour(&self) -> (isize, isize) {
        let mut colour1 = 0;
        let mut colour2 = 0;
        functions::TextDrawGetPreviewVehColour(self, &mut colour1, &mut colour2);
        (colour1, colour2)
    }
    /// Changes the text on a textdraw for a specific player.
    pub fn set_string_for_player(&self, player: &Player, text: &str) {
        functions::TextDrawSetStringForPlayer(self, player, text)
    }
    pub fn get_id(&self) -> isize {
        functions::TextDrawGetID(self)
    }
    pub fn from_id(id: isize) -> Option<TextDraw> {
        functions::TextDrawFromID(id)
    }
}

pub struct PlayerTextDraw {
    handle: *const c_void,
}

impl PlayerTextDraw {
    pub fn get_handle(&self) -> *const c_void {
        self.handle
    }
    pub fn new(handle: *const c_void) -> Self {
        Self { handle }
    }
    pub fn is_shown(&self) -> bool {
        functions::IsPlayerTextDrawShown(self)
    }
    pub fn set_letter_size(&self, size: Vector2) {
        functions::PlayerTextDrawSetLetterSize(self, size)
    }
    pub fn set_text_size(&self, size: Vector2) {
        functions::PlayerTextDrawSetTextSize(self, size)
    }
    /// Set the text alignment of a player-textdraw.
    pub fn alignment(&self, alignment: TextDrawAlignmentTypes) {
        functions::PlayerTextDrawAlignment(self, alignment)
    }
    /// Sets the text color of a player-textdraw.
    pub fn color(&self, colour: Colour) {
        functions::PlayerTextDrawColor(self, colour)
    }
    /// Toggle the box on a player-textdraw.
    pub fn use_box(&self, box_use: bool) {
        functions::PlayerTextDrawUseBox(self, box_use)
    }
    pub fn set_box_color(&self, colour: Colour) {
        functions::PlayerTextDrawSetBoxColor(self, colour)
    }
    /// Adds a shadow to the bottom-right side of the text in a player-textdraw.
    pub fn set_shadow(&self, size: isize) {
        functions::PlayerTextDrawSetShadow(self, size)
    }
    /// Set the outline of a player-textdraw.
    pub fn set_outline(&self, size: isize) {
        functions::PlayerTextDrawSetOutline(self, size)
    }
    /// Adjust the background color of a player-textdraw.
    pub fn background_color(&self, colour: Colour) {
        functions::PlayerTextDrawBackgroundColor(self, colour)
    }
    pub fn set_style(&self, font: TextDrawStyle) {
        functions::PlayerTextDrawSetStyle(self, font)
    }
    /// Appears to scale text spacing to a proportional ratio.
    pub fn set_proportional(&self, set: bool) {
        functions::PlayerTextDrawSetProportional(self, set)
    }
    /// Toggles whether a player-textdraw can be selected or not.
    pub fn set_selectable(&self, set: bool) {
        functions::PlayerTextDrawSetSelectable(self, set)
    }
    /// Show a player-textdraw to the player it was created for.
    pub fn show(&self) {
        functions::PlayerTextDrawShow(self)
    }
    /// Hide a player-textdraw from the player it was created for.
    pub fn hide(&self) {
        functions::PlayerTextDrawHide(self)
    }
    /// Change the text of a player-textdraw.
    pub fn set_string(&self, text: &str) {
        functions::PlayerTextDrawSetString(self, text)
    }
    /// Sets a player textdraw 3D preview sprite of a specified model ID.
    pub fn set_preview_model(&self, model: isize) {
        functions::PlayerTextDrawSetPreviewModel(self, model)
    }
    pub fn set_preview_rotation(&self, rotation: Vector3, zoom: f32) {
        functions::PlayerTextDrawSetPreviewRotation(self, rotation, zoom)
    }
    pub fn set_preview_veh_colour(&self, colour1: isize, colour2: isize) {
        functions::PlayerTextDrawSetPreviewVehColour(self, colour1, colour2)
    }
    /// Sets the position of a player-textdraw.
    pub fn set_pos(&self, pos: Vector2) {
        functions::PlayerTextDrawSetPos(self, pos)
    }
    /// Gets the text of a player-textdraw.
    pub fn get_string(&self) -> String {
        let mut text = String::new();
        functions::PlayerTextDrawGetString(self, &mut text);
        text
    }
    /// Gets the width and height of the letters.
    pub fn get_letter_size(&self) -> Vector2 {
        let mut size = Vector2::default();
        functions::PlayerTextDrawGetLetterSize(self, &mut size);
        size
    }
    /// Gets the X axis and Y axis of the player-textdraw text size.
    pub fn get_text_size(&self) -> Vector2 {
        let mut size = Vector2::default();
        functions::PlayerTextDrawGetTextSize(self, &mut size);
        size
    }
    /// Gets the position of a player-textdraw.
    pub fn get_pos(&self) -> Vector2 {
        let mut pos = Vector2::default();
        functions::PlayerTextDrawGetPos(self, &mut pos);
        pos
    }
    /// Gets the text color of a player-textdraw
    pub fn get_color(&self) -> Colour {
        let mut colour = Colour::default();
        functions::PlayerTextDrawGetColor(self, &mut colour);
        colour
    }
    /// Gets the box color of a player-textdraw
    pub fn get_box_color(&self) -> Colour {
        let mut colour = Colour::default();
        functions::PlayerTextDrawGetBoxColor(self, &mut colour);
        colour
    }
    /// Gets the background colour of a player-textdraw
    pub fn get_background_colour(&self) -> Colour {
        let mut colour = Colour::default();
        functions::PlayerTextDrawGetBackgroundColour(self, &mut colour);
        colour
    }
    /// Get the shadow size on a player-textdraw.
    pub fn get_shadow(&self) -> isize {
        functions::PlayerTextDrawGetShadow(self)
    }
    /// Get the outline size on a player-textdraw.
    pub fn get_outline(&self) -> isize {
        functions::PlayerTextDrawGetOutline(self)
    }
    pub fn get_style(&self) -> isize {
        functions::PlayerTextDrawGetStyle(self)
    }
    /// Checks if a player-textdraw is box.
    pub fn is_box(&self) -> bool {
        functions::PlayerTextDrawIsBox(self)
    }
    /// Checks if a player-textdraw is proportional.
    pub fn is_proportional(&self) -> bool {
        functions::PlayerTextDrawIsProportional(self)
    }
    /// Checks if a player-textdraw is selectable.
    pub fn is_selectable(&self) -> bool {
        functions::PlayerTextDrawIsSelectable(self)
    }
    /// Gets the text alignment of a player-textdraw.
    pub fn get_alignment(&self) -> TextDrawAlignmentTypes {
        functions::PlayerTextDrawGetAlignment(self)
    }
    /// Gets the preview model of a 3D preview player-textdraw.
    pub fn get_preview_model(&self) -> isize {
        functions::PlayerTextDrawGetPreviewModel(self)
    }
    pub fn get_preview_rotation(&self) -> (Vector3, f32) {
        let mut rotation = Vector3::default();
        let mut zoom = 0.0;
        functions::PlayerTextDrawGetPreviewRotation(self, &mut rotation, &mut zoom);
        (rotation, zoom)
    }
    pub fn get_preview_veh_colour(&self) -> (isize, isize) {
        let mut colour1 = 0;
        let mut colour2 = 0;
        functions::PlayerTextDrawGetPreviewVehColour(self, &mut colour1, &mut colour2);
        (colour1, colour2)
    }
    pub fn get_id(&self) -> isize {
        functions::PlayerTextDrawGetID(self)
    }
    pub fn from_id(selfid: isize, player: &Player) -> Option<PlayerTextDraw> {
        functions::PlayerTextDrawFromID(selfid, player)
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
