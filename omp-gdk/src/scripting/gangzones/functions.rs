use omp_codegen::native;
use std::ffi::c_void;

use crate::players::Player;

use super::GangZone;

native!(GangZone_Create, minx: f32, miny: f32, maxx: f32, maxy: f32, id: mut i32, -> struct GangZone);
native!(GangZone_Destroy, gangzone: struct GangZone, -> bool);
native!(GangZone_FromID, gangzoneid: i32, -> struct GangZone);
native!(GangZone_GetID, gangzone: struct GangZone, -> i32);
native!(GangZone_ShowForPlayer, player: struct Player, gangzone: struct GangZone, color: u32, -> bool);
native!(GangZone_ShowForAll, gangzone: struct GangZone, color: u32, -> bool);
native!(GangZone_HideForPlayer, player: struct Player, gangzone: struct GangZone, -> bool);
native!(GangZone_HideForAll, gangzone: struct GangZone, -> bool);
native!(GangZone_FlashForPlayer, player: struct Player, gangzone: struct GangZone, color: u32, -> bool);
native!(GangZone_FlashForAll, gangzone: struct GangZone, color: u32, -> bool);
native!(GangZone_StopFlashForPlayer, player: struct Player, gangzone: struct GangZone, -> bool);
native!(GangZone_StopFlashForAll, gangzone: struct GangZone, -> bool);
native!(GangZone_IsValid, gangzone: struct GangZone, -> bool);
native!(GangZone_IsPlayerIn, player: struct Player, gangzone: struct GangZone, -> bool);
native!(GangZone_IsVisibleForPlayer, player: struct Player, gangzone: struct GangZone, -> bool);
native!(GangZone_GetColorForPlayer, player: struct Player, gangzone: struct GangZone, -> i32);
native!(GangZone_GetFlashColorForPlayer, player: struct Player, gangzone: struct GangZone, -> i32);
native!(GangZone_IsFlashingForPlayer, player: struct Player, gangzone: struct GangZone, -> bool);
native!(GangZone_GetPos, gangzone: struct GangZone, minx: mut f32, miny: mut f32, maxx: mut f32, maxy: mut f32, -> bool);
native!(GangZone_UseCheck, gangzone: struct GangZone, enable: bool, -> bool);

#[doc(hidden)]
pub fn load_functions() {
    load_function!(GangZone_Create);
    load_function!(GangZone_Destroy);
    load_function!(GangZone_FromID);
    load_function!(GangZone_GetID);
    load_function!(GangZone_ShowForPlayer);
    load_function!(GangZone_ShowForAll);
    load_function!(GangZone_HideForPlayer);
    load_function!(GangZone_HideForAll);
    load_function!(GangZone_FlashForPlayer);
    load_function!(GangZone_FlashForAll);
    load_function!(GangZone_StopFlashForPlayer);
    load_function!(GangZone_StopFlashForAll);
    load_function!(GangZone_IsValid);
    load_function!(GangZone_IsPlayerIn);
    load_function!(GangZone_IsVisibleForPlayer);
    load_function!(GangZone_GetColorForPlayer);
    load_function!(GangZone_GetFlashColorForPlayer);
    load_function!(GangZone_IsFlashingForPlayer);
    load_function!(GangZone_GetPos);
    load_function!(GangZone_UseCheck);
}
