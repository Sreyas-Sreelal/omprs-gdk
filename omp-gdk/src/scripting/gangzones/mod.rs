pub mod events;
pub mod functions;

use std::ffi::c_void;

pub use functions::load_functions;

use crate::{
    players::Player,
    runtime::queue_api_call,
    types::{colour::Colour, vector::Vector2},
};

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
        let mut _id = 0;
        functions::GangZone_Create(pos.min.x, pos.min.y, pos.max.x, pos.max.y, &mut _id)
    }

    /// Destroy a gangzone.
    pub fn destroy(&self) {
        self.defer_api_call(Box::new(move |gangzone| {
            functions::GangZone_Destroy(&gangzone);
        }));
    }

    /// Show a gangzone for a player.
    pub fn show_for_player(&self, player: &Player, colour: Colour) {
        let player_id = player.get_id();
        self.defer_api_call(Box::new(move |gangzone| {
            let player = Player::from_id(player_id).unwrap();
            functions::GangZone_ShowForPlayer(&player, &gangzone, colour.rgba());
        }));
    }

    /// Shows a gangzone with the desired color to all players.
    pub fn show_for_all(&self, colour: Colour) {
        self.defer_api_call(Box::new(move |gangzone| {
            functions::GangZone_FlashForAll(&gangzone, colour.rgba());
        }));
    }

    /// Hide a gangzone for a player.
    pub fn hide_for_player(&self, player: &Player) {
        let player_id = player.get_id();
        self.defer_api_call(Box::new(move |gangzone| {
            let player = Player::from_id(player_id).unwrap();
            functions::GangZone_HideForPlayer(&player, &gangzone);
        }));
    }

    /// Hide a gangzone for all players.
    pub fn hide_for_all(&self) {
        self.defer_api_call(Box::new(move |gangzone| {
            functions::GangZone_HideForAll(&gangzone);
        }));
    }

    /// Make a gangzone flash for a player.
    pub fn flash_for_player(&self, player: &Player, colour: Colour) -> bool {
        functions::GangZone_FlashForPlayer(player, self, colour.rgba())
    }

    /// Make a gangzone flash for all players.
    pub fn flash_for_all(&self, colour: Colour) -> bool {
        functions::GangZone_FlashForAll(self, colour.rgba())
    }

    /// Stop a gangzone flashing for a player.
    pub fn stop_flash_for_player(&self, player: &Player) -> bool {
        functions::GangZone_StopFlashForPlayer(player, self)
    }

    /// Make a gangzone flash for all players.
    pub fn stop_flash_for_all(&self) -> bool {
        functions::GangZone_StopFlashForAll(self)
    }

    /* /// Checks if a gangzone id is valid or not
    pub fn is_valid_id(gangzoneid: isize) -> bool {
        functions::IsValidGangZoneID(gangzoneid)
    }
    */

    /// Check if the player in gangzone.
    pub fn is_player_in_gang_zone(&self, player: &Player) -> bool {
        functions::GangZone_IsPlayerIn(player, self)
    }

    /// Check if the gangzone is visible for player.
    pub fn is_visible_for_player(&self, player: &Player) -> bool {
        functions::GangZone_IsVisibleForPlayer(player, self)
    }

    /// Get the colour of a gangzone for player.
    pub fn get_color_for_player(&self, player: &Player) -> i32 {
        functions::GangZone_GetColorForPlayer(player, self)
    }

    /// Get the flashing colour of a gangzone for player.
    pub fn get_flash_color_for_player(&self, player: &Player) -> i32 {
        functions::GangZone_GetFlashColorForPlayer(player, self)
    }

    // Checks if a gangzone is flashing for player
    pub fn is_flashing_for_player(&self, player: &Player) -> bool {
        functions::GangZone_IsFlashingForPlayer(player, self)
    }

    /// Get domensions of a Gangzone
    pub fn get_pos(&self) -> GangZonePos {
        let mut pos = GangZonePos::default();
        functions::GangZone_GetPos(
            self,
            &mut pos.min.x,
            &mut pos.min.y,
            &mut pos.max.x,
            &mut pos.max.y,
        );
        pos
    }

    /// Enables the callback when a player enters/leaves this zone.
    pub fn use_check(&self, enable: bool) -> bool {
        functions::GangZone_UseCheck(self, enable)
    }

    /// Gets id of a gangzone
    pub fn get_id(&self) -> i32 {
        functions::GangZone_GetID(self)
    }

    /// Get a gangzone object from an id
    pub fn from_id(gangzoneid: i32) -> Option<GangZone> {
        functions::GangZone_FromID(gangzoneid)
    }

    fn defer_api_call(&self, callback: Box<dyn FnOnce(Self)>) {
        let gangzone_id = self.get_id();
        queue_api_call(Box::new(move || {
            let gangzone = Self::from_id(gangzone_id).unwrap();
            callback(gangzone);
        }));
    }
}
