#[macro_use]
pub mod runtime;

pub mod events {
    pub use omprs_gdk::Events;
}

pub mod actors {
    pub use omprs_gdk::actors::{Actor, ActorSpawnData};
}

pub mod checkpoint {
    pub use omprs_gdk::checkpoints::{
        PlayerCheckPointData, PlayerRaceCheckPointData, RaceCheckpointType,
    };
}

pub mod classes {
    pub use omprs_gdk::classes::{
        CreateClass, EditClassData, GetAvailableClasses, GetClassData, PlayerClass,
    };
}

pub mod core {
    pub use omprs_gdk::core::functions::*;
}

pub mod dialogs {
    pub use omprs_gdk::dialogs::{DialogResponse, DialogStyle};
}

pub mod gangzones {
    pub use omprs_gdk::gangzones::{GangZone, GangZonePos};
}

pub mod menus {
    pub use omprs_gdk::menus::Menu;
}

pub mod models {
    pub use omprs_gdk::models::{functions::*, ModelDownloadType};
}

pub mod objects {
    pub use omprs_gdk::objects::{
        Object, ObjectAttachmentData, ObjectAttachmentSlotData, ObjectAttachmentType,
        ObjectEditResponse, ObjectMaterialData, ObjectMaterialSize, ObjectMaterialTextAlign,
        ObjectMaterialType, ObjectMoveData, PlayerObject,
    };
}

pub mod pickups {
    pub use omprs_gdk::pickups::Pickup;
}

pub mod players {
    pub use omprs_gdk::players::{
        BodyPart, ClientVersion, EPlayerNameStatus, ESpectateType, MapIconStyle, Player,
        PlayerAimData, PlayerAnimationData, PlayerAnimationSyncType, PlayerBulletData,
        PlayerBulletHitType, PlayerCameraCutType, PlayerClickSource, PlayerFightingStyle,
        PlayerKeyData, PlayerSpecialAction, PlayerSpectateData, PlayerSpectateMode, PlayerState,
        PlayerSurfingData, PlayerWeaponSkill, PlayerWeaponState, WeaponSlotData, WeaponSlots,
    };
}

pub mod textdraws {
    pub use omprs_gdk::textdraws::{
        PlayerTextDraw, TextDraw, TextDrawAlignmentTypes, TextDrawStyle,
    };
}

pub mod textlabels {
    pub use omprs_gdk::textlabels::{PlayerTextLabel, TextLabel, TextLabelAttachmentData};
}

pub mod vehicles {
    pub use omprs_gdk::vehicles::{
        UnoccupiedVehicleUpdate, Vehicle, VehicleDamageStatusData, VehicleMatrix, VehicleParams,
        VehicleSpawnData,
    };
}

pub use omprs_gdk::{init_functions, main, types};
