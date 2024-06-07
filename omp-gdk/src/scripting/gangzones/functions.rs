use omp_codegen::native;
use std::ffi::c_void;

use crate::{players::Player, types::colour::Colour};

use super::{GangZone, GangZonePos};

native!(GangZoneCreate, pos:GangZonePos, -> struct GangZone);
native!(GangZoneDestroy, gangzone: struct GangZone);
native!(GangZoneShowForPlayer, gangzone: struct GangZone,player: struct Player, colour: Colour);
native!(GangZoneShowForAll, gangzone: struct GangZone, colour: Colour);
native!(GangZoneHideForPlayer, gangzone: struct GangZone,player: struct Player);
native!(GangZoneHideForAll, gangzone: struct GangZone);
native!(GangZoneFlashForPlayer, gangzone: struct GangZone,player: struct Player, colour: Colour);
native!(GangZoneFlashForAll, gangzone: struct GangZone, colour: Colour);
native!(GangZoneStopFlashForPlayer, gangzone: struct GangZone, player: struct Player);
native!(GangZoneStopFlashForAll, gangzone: struct GangZone);
native!(IsValidGangZoneID, gangzoneid:isize, -> bool);
native!(IsPlayerInGangZone, gangzone: struct GangZone, player: struct Player, -> bool);
native!(IsGangZoneVisibleForPlayer, gangzone: struct GangZone, player: struct Player, -> bool);
native!(GangZoneGetColorForPlayer, gangzone: struct GangZone, player: struct Player, -> isize);
native!(GangZoneGetFlashColorForPlayer, gangzone: struct GangZone, player: struct Player, -> isize);
native!(IsGangZoneFlashingForPlayer, gangzone: struct GangZone, player: struct Player, -> bool);
native!(GangZoneGetPos, gangzone: struct GangZone, pos: mut GangZonePos);
native!(UseGangZoneCheck, gangzone: struct GangZone, enable:bool);
native!(GetGangZoneID,gangzone: struct GangZone,-> isize);
native!(GetGangZoneFromID,gangzoneid: isize,-> struct GangZone);

#[doc(hidden)]
pub fn load_functions() {
    load_function!(GangZoneCreate);
    load_function!(GangZoneDestroy);
    load_function!(GangZoneShowForPlayer);
    load_function!(GangZoneShowForAll);
    load_function!(GangZoneHideForPlayer);
    load_function!(GangZoneHideForAll);
    load_function!(GangZoneFlashForPlayer);
    load_function!(GangZoneFlashForAll);
    load_function!(GangZoneStopFlashForPlayer);
    load_function!(GangZoneStopFlashForAll);
    load_function!(IsValidGangZoneID);
    load_function!(IsPlayerInGangZone);
    load_function!(IsGangZoneVisibleForPlayer);
    load_function!(GangZoneGetColorForPlayer);
    load_function!(GangZoneGetFlashColorForPlayer);
    load_function!(IsGangZoneFlashingForPlayer);
    load_function!(GangZoneGetPos);
    load_function!(UseGangZoneCheck);
    load_function!(GetGangZoneID);
    load_function!(GetGangZoneFromID);
}
