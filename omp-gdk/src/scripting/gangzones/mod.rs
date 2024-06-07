pub mod events;
pub mod functions;

use std::ffi::c_void;

pub use functions::load_functions;

use crate::{players::Player, types::colour::Colour, types::vector::Vector2};

/// GangZone dimensions
#[repr(C)]
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct GangZonePos {
    /// west (x) and south (y) side of a gangzone
    pub min: Vector2,
    /// east (x) and north (y) side of a gangzone
    pub max: Vector2,
}

impl GangZonePos {
    pub fn new(min: Vector2, max: Vector2) -> Self {
        GangZonePos { min, max }
    }
}

/// Main handler pointing to open.mp's GangZone object
pub struct GangZone {
    handle: *const c_void,
}

impl GangZone {
    pub fn get_handle(&self) -> *const c_void {
        self.handle
    }

    pub fn new(handle: *const c_void) -> Self {
        Self { handle }
    }

    /// Create a gangzone (colored radar area).
    pub fn create(pos: GangZonePos) -> Option<Self> {
        functions::GangZoneCreate(pos)
    }

    /// Destroy a gangzone.
    pub fn destroy(&self) {
        functions::GangZoneDestroy(self)
    }

    /// Show a gangzone for a player.
    pub fn show_for_player(&self, player: &Player, colour: Colour) {
        functions::GangZoneShowForPlayer(self, player, colour)
    }

    /// Shows a gangzone with the desired color to all players.
    pub fn show_for_all(&self, colour: Colour) {
        functions::GangZoneFlashForAll(self, colour)
    }

    /// Hide a gangzone for a player.
    pub fn hide_for_player(&self, player: &Player) {
        functions::GangZoneHideForPlayer(self, player)
    }

    /// Hide a gangzone for all players.
    pub fn hide_for_all(&self) {
        functions::GangZoneHideForAll(self)
    }

    /// Make a gangzone flash for a player.
    pub fn flash_for_player(&self, player: &Player, colour: Colour) {
        functions::GangZoneFlashForPlayer(self, player, colour)
    }

    /// Make a gangzone flash for all players.
    pub fn flash_for_all(&self, colour: Colour) {
        functions::GangZoneFlashForAll(self, colour)
    }

    /// Stop a gangzone flashing for a player.
    pub fn stop_flash_for_player(&self, player: &Player) {
        functions::GangZoneStopFlashForPlayer(self, player)
    }

    /// Make a gangzone flash for all players.
    pub fn stop_flash_for_all(&self) {
        functions::GangZoneStopFlashForAll(self)
    }

    /// Checks if a gangzone id is valid or not
    pub fn is_valid_id(gangzoneid: isize) -> bool {
        functions::IsValidGangZoneID(gangzoneid)
    }

    /// Check if the player in gangzone.
    pub fn is_player_in_gang_zone(&self, player: &Player) -> bool {
        functions::IsPlayerInGangZone(self, player)
    }

    /// Check if the gangzone is visible for player.
    pub fn is_visible_for_player(&self, player: &Player) -> bool {
        functions::IsGangZoneVisibleForPlayer(self, player)
    }

    /// Get the colour of a gangzone for player.
    pub fn get_color_for_player(&self, player: &Player) -> isize {
        functions::GangZoneGetColorForPlayer(self, player)
    }

    /// Get the flashing colour of a gangzone for player.
    pub fn get_flash_color_for_player(&self, player: &Player) -> isize {
        functions::GangZoneGetFlashColorForPlayer(self, player)
    }

    // Checks if a gangzone is flashing for player
    pub fn is_flashing_for_player(&self, player: &Player) -> bool {
        functions::IsGangZoneFlashingForPlayer(self, player)
    }

    /// Get domensions of a Gangzone
    pub fn get_pos(&self) -> GangZonePos {
        let mut pos = GangZonePos::default();
        functions::GangZoneGetPos(self, &mut pos);
        pos
    }

    /// Enables the callback when a player enters/leaves this zone.
    pub fn use_check(&self, enable: bool) {
        functions::UseGangZoneCheck(self, enable)
    }

    /// Gets id of a gangzone
    pub fn get_id(&self) -> isize {
        functions::GetGangZoneID(self)
    }

    /// Get a gangzone object from an id
    pub fn get_from_id(&self, gangzoneid: isize) -> Option<GangZone> {
        functions::GetGangZoneFromID(gangzoneid)
    }
}
