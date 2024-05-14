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
    pub use omp_gdk::classes::{
        CreateClass, EditClassData, GetAvailableClasses, GetClassData, PlayerClass,
    };
}

pub mod core {
    pub use omp_gdk::core::functions::*;
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

pub use omp_gdk::{init_functions, main, types};
