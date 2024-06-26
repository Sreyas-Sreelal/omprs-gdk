//! Rust SDK for developing open.mp gamemodes
#[macro_use]
pub mod runtime;

pub mod events {
    pub use omp_gdk::Events;
}

pub mod actors {
    pub use omp_gdk::actors::{Actor, ActorSpawnData};
}

pub mod checkpoint {
    pub use omp_gdk::checkpoints::{
        PlayerCheckPointData, PlayerRaceCheckPointData, RaceCheckpointType,
    };
}

pub mod classes {
    pub use omp_gdk::classes::Class;
}

pub mod core {
    pub use omp_gdk::core::functions::{
        All_CreateExplosion as CreateExplosionForAll,
        All_EnableStuntBonus as EnableStuntBonusForAll,
        All_SendClientMessage as SendClientMessageToAll, All_SendDeathMessage as SendDeathMessage,
        Core_AddRule as AddRule, Core_AllowAdminTeleport as AllowAdminTeleport,
        Core_AllowInteriorWeapons as AllowInteriorWeapons,
        Core_AllowNickNameCharacter as AllowNickNameCharacter,
        Core_AreAllAnimationsEnabled as AreAllAnimationsEnabled,
        Core_AreInteriorWeaponsAllowed as AreInteriorWeaponsAllowed,
        Core_BlockIpAddress as BlockIpAddress, Core_ClearBanList as ClearBanList,
        Core_DisableEntryExitMarkers as DisableInteriorEnterExits,
        Core_DisableNameTagsLOS as DisableNameTagsLOS,
        Core_EnableAllAnimations as EnableAllAnimations, Core_EnableZoneNames as EnableZoneNames,
        Core_GameMode_SetText as SetGameModeText, Core_GetGravity as GetGravity,
        Core_GetWeaponName as GetWeaponName, Core_GetWeaponSlot as GetWeaponSlot,
        Core_GetWeather as GetWeather, Core_GetWorldTime as GetWorldTime,
        Core_HideGameTextForAll as HideGameTextForAll,
        Core_IsAdminTeleportAllowed as IsAdminTeleportAllowed,
        Core_IsAnimationLibraryValid as IsAnimationLibraryValid,
        Core_IsChatTextReplacementToggled as IsChatTextReplacementToggled,
        Core_IsIpAddressBanned as IsIpAddressBanned,
        Core_IsNickNameCharacterAllowed as IsNickNameCharacterAllowed,
        Core_IsNickNameValid as IsNickNameValid, Core_IsValidRule as IsValidRule, Core_Log as Log,
        Core_MaxPlayers as MaxPlayers, Core_NetworkStats as NetworkStats,
        Core_RemoveRule as RemoveRule, Core_SendRconCommand as SendRconCommand,
        Core_ServerTickRate as ServerTickRate, Core_SetChatRadius as SetChatRadius,
        Core_SetDeathDropAmount as SetDeathDropAmount, Core_SetGravity as SetGravity,
        Core_SetMarkerRadius as SetMarkerRadius,
        Core_SetNameTagsDrawDistance as SetNameTagsDrawDistance, Core_SetWeather as SetWeather,
        Core_SetWorldTime as SetWorldTime, Core_ShowGameTextForAll as GameTextForAll,
        Core_ShowNameTags as ShowNameTags, Core_ShowPlayerMarkers as ShowPlayerMarkers,
        Core_TickCount as TickCount, Core_ToggleChatTextReplacement as ToggleChatTextReplacement,
        Core_UnBlockIpAddress as UnBlockIpAddress, Core_UsePedAnims as UsePedAnims,
    };
}

pub mod dialogs {
    pub use omp_gdk::dialogs::{DialogResponse, DialogStyle};
}

pub mod gangzones {
    pub use omp_gdk::gangzones::{GangZone, GangZonePos};
}

pub mod menus {
    pub use omp_gdk::menus::Menu;
}

pub mod models {
    pub use omp_gdk::models::{functions::*, ModelDownloadType};
}

pub mod objects {
    pub use omp_gdk::objects::{
        Object, ObjectAttachmentData, ObjectAttachmentSlotData, ObjectAttachmentType,
        ObjectEditResponse, ObjectMaterialData, ObjectMaterialSize, ObjectMaterialTextAlign,
        ObjectMaterialType, ObjectMoveData, PlayerObject,
    };
}

pub mod pickups {
    pub use omp_gdk::pickups::Pickup;
}

pub mod players {
    pub use omp_gdk::players::{
        BodyPart, ClientVersion, MapIconStyle, Player, PlayerAimData, PlayerAnimationData,
        PlayerAnimationSyncType, PlayerBulletData, PlayerBulletHitType, PlayerCameraCutType,
        PlayerClickSource, PlayerFightingStyle, PlayerKeyData, PlayerKeys, PlayerNameStatus,
        PlayerSpecialAction, PlayerSpectateData, PlayerSpectateMode, PlayerState,
        PlayerSurfingData, PlayerWeapon, PlayerWeaponSkill, PlayerWeaponState, SpectateType,
        WeaponSlotData, WeaponSlots,
    };
}

pub mod textdraws {
    pub use omp_gdk::textdraws::{PlayerTextDraw, TextDraw, TextDrawAlignmentTypes, TextDrawStyle};
}

pub mod textlabels {
    pub use omp_gdk::textlabels::{PlayerTextLabel, TextLabel, TextLabelAttachmentData};
}

pub mod vehicles {
    pub use omp_gdk::vehicles::{
        UnoccupiedVehicleUpdate, Vehicle, VehicleDamageStatusData, VehicleMatrix, VehicleParams,
        VehicleSpawnData,
    };
}

pub use omp_gdk::ComponentVersion;
pub use omp_gdk::OMPRS_Component_Create as Component_Create;
pub use omp_gdk::{init_functions, main, types};
