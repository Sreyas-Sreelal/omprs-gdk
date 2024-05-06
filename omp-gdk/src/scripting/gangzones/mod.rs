pub mod events;
pub mod functions;

use std::ffi::c_void;

pub use functions::load_functions;

use crate::{players::Player, types::colour::Colour, types::vector::Vector2};

#[repr(C)]
#[derive(Default)]
pub struct GangZonePos {
    pub min: Vector2,
    pub max: Vector2,
}

impl GangZonePos {
    pub fn new(min: Vector2, max: Vector2) -> Self {
        GangZonePos { min, max }
    }
}
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

    pub fn create(pos: GangZonePos) -> Option<Self> {
        functions::GangZoneCreate(pos)
    }

    pub fn destroy(&self) {
        functions::GangZoneDestroy(self)
    }

    pub fn show_for_player(&self, player: &Player, colour: Colour) {
        functions::GangZoneShowForPlayer(self, player, colour)
    }

    pub fn show_for_all(&self, colour: Colour) {
        functions::GangZoneFlashForAll(self, colour)
    }

    pub fn hide_for_player(&self, player: &Player) {
        functions::GangZoneHideForPlayer(self, player)
    }

    pub fn hide_for_all(&self) {
        functions::GangZoneHideForAll(self)
    }

    pub fn flash_for_player(&self, player: &Player, colour: Colour) {
        functions::GangZoneFlashForPlayer(self, player, colour)
    }

    pub fn flash_for_all(&self, colour: Colour) {
        functions::GangZoneFlashForAll(self, colour)
    }

    pub fn stop_flash_for_player(&self, player: &Player) {
        functions::GangZoneStopFlashForPlayer(self, player)
    }

    pub fn stop_flash_for_all(&self) {
        functions::GangZoneStopFlashForAll(self)
    }

    pub fn is_valid_id(gangzoneid: isize) -> bool {
        functions::IsValidGangZoneID(gangzoneid)
    }

    pub fn is_player_in_gang_zone(&self, player: &Player) -> bool {
        functions::IsPlayerInGangZone(self, player)
    }

    pub fn is_visible_for_player(&self, player: &Player) -> bool {
        functions::IsGangZoneVisibleForPlayer(self, player)
    }

    pub fn get_color_for_player(&self, player: &Player) -> isize {
        functions::GangZoneGetColorForPlayer(self, player)
    }

    pub fn get_flash_color_for_player(&self, player: &Player) -> isize {
        functions::GangZoneGetFlashColorForPlayer(self, player)
    }

    pub fn is_flashing_for_player(&self, player: &Player) -> bool {
        functions::IsGangZoneFlashingForPlayer(self, player)
    }

    pub fn get_pos(&self) -> GangZonePos {
        let mut pos = GangZonePos::default();
        functions::GangZoneGetPos(self, &mut pos);
        pos
    }

    pub fn use_check(&self, enable: bool) {
        functions::UseGangZoneCheck(self, enable)
    }
}
