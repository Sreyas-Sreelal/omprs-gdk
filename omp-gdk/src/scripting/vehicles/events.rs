#![allow(clippy::all)]
use std::mem::transmute;

use crate::{events::EventArgs, players::Player, types::vector::Vector3};

use super::{UnoccupiedVehicleUpdate, Vehicle};

#[repr(C)]
pub struct OnVehicleStreamInArgs {
    vehicle: *const *const std::ffi::c_void,
    player: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnVehicleStreamIn(args: *const EventArgs<OnVehicleStreamInArgs>) {
    let scripts = crate::runtime::Runtime.as_mut().unwrap();
    for script in scripts.iter_mut() {
        script.lock().unwrap().on_vehicle_stream_in(
            Vehicle::new(*(*(*args).list).vehicle),
            Player::new(*(*(*args).list).player),
        );
    }
}

#[repr(C)]
pub struct OnVehicleStreamOutArgs {
    vehicle: *const *const std::ffi::c_void,
    player: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnVehicleStreamOut(args: *const EventArgs<OnVehicleStreamOutArgs>) {
    let scripts = crate::runtime::Runtime.as_mut().unwrap();
    for script in scripts.iter_mut() {
        script.lock().unwrap().on_vehicle_stream_out(
            Vehicle::new(*(*(*args).list).vehicle),
            Player::new(*(*(*args).list).player),
        );
    }
}

#[repr(C)]
pub struct OnVehicleDeathArgs {
    vehicle: *const *const std::ffi::c_void,
    player: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnVehicleDeath(args: *const EventArgs<OnVehicleDeathArgs>) {
    let scripts = crate::runtime::Runtime.as_mut().unwrap();
    for script in scripts.iter_mut() {
        script.lock().unwrap().on_vehicle_death(
            Vehicle::new(*(*(*args).list).vehicle),
            Player::new(*(*(*args).list).player),
        );
    }
}

#[repr(C)]
pub struct OnPlayerEnterVehicleArgs {
    player: *const *const std::ffi::c_void,
    vehicle: *const *const std::ffi::c_void,
    passenger: *const bool,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerEnterVehicle(
    args: *const EventArgs<OnPlayerEnterVehicleArgs>,
) {
    let scripts = crate::runtime::Runtime.as_mut().unwrap();
    for script in scripts.iter_mut() {
        script.lock().unwrap().on_player_enter_vehicle(
            Player::new(*(*(*args).list).player),
            Vehicle::new(*(*(*args).list).vehicle),
            *(*(*args).list).passenger,
        );
    }
}

#[repr(C)]
pub struct OnPlayerExitVehicleArgs {
    player: *const *const std::ffi::c_void,
    vehicle: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerExitVehicle(
    args: *const EventArgs<OnPlayerExitVehicleArgs>,
) {
    let scripts = crate::runtime::Runtime.as_mut().unwrap();
    for script in scripts.iter_mut() {
        script.lock().unwrap().on_player_exit_vehicle(
            Player::new(*(*(*args).list).player),
            Vehicle::new(*(*(*args).list).vehicle),
        );
    }
}

#[repr(C)]
pub struct OnVehicleDamageStatusUpdateArgs {
    vehicle: *const *const std::ffi::c_void,
    player: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnVehicleDamageStatusUpdate(
    args: *const EventArgs<OnVehicleDamageStatusUpdateArgs>,
) {
    let scripts = crate::runtime::Runtime.as_mut().unwrap();
    for script in scripts.iter_mut() {
        script.lock().unwrap().on_vehicle_damage_status_update(
            Vehicle::new(*(*(*args).list).vehicle),
            Player::new(*(*(*args).list).player),
        );
    }
}

#[repr(C)]
pub struct OnVehiclePaintJobArgs {
    player: *const *const std::ffi::c_void,
    vehicle: *const *const std::ffi::c_void,
    paintJob: *const i32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnVehiclePaintJob(
    args: *const EventArgs<OnVehiclePaintJobArgs>,
) -> bool {
    let scripts = crate::runtime::Runtime.as_mut().unwrap();
    let mut ret = false;
    for script in scripts.iter_mut() {
        ret = script.lock().unwrap().on_vehicle_paint_job(
            Player::new(*(*(*args).list).player),
            Vehicle::new(*(*(*args).list).vehicle),
            *(*(*args).list).paintJob,
        );
        if crate::runtime::__terminate_event_chain {
            crate::runtime::__terminate_event_chain = false;
            return ret;
        }
    }
    ret
}

#[repr(C)]
pub struct OnVehicleModArgs {
    player: *const *const std::ffi::c_void,
    vehicle: *const *const std::ffi::c_void,
    component: *const i32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnVehicleMod(args: *const EventArgs<OnVehicleModArgs>) -> bool {
    let scripts = crate::runtime::Runtime.as_mut().unwrap();
    let mut ret = false;
    for script in scripts.iter_mut() {
        ret = script.lock().unwrap().on_vehicle_mod(
            Player::new(*(*(*args).list).player),
            Vehicle::new(*(*(*args).list).vehicle),
            *(*(*args).list).component,
        );
        if crate::runtime::__terminate_event_chain {
            crate::runtime::__terminate_event_chain = false;
            return ret;
        }
    }
    ret
}

#[repr(C)]
pub struct OnVehicleResprayArgs {
    player: *const *const std::ffi::c_void,
    vehicle: *const *const std::ffi::c_void,
    color1: *const i32,
    color2: *const i32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnVehicleRespray(
    args: *const EventArgs<OnVehicleResprayArgs>,
) -> bool {
    let scripts = crate::runtime::Runtime.as_mut().unwrap();
    let mut ret = false;
    for script in scripts.iter_mut() {
        ret = script.lock().unwrap().on_vehicle_respray(
            Player::new(*(*(*args).list).player),
            Vehicle::new(*(*(*args).list).vehicle),
            *(*(*args).list).color1,
            *(*(*args).list).color2,
        );
        if crate::runtime::__terminate_event_chain {
            crate::runtime::__terminate_event_chain = false;
            return ret;
        }
    }
    ret
}

#[repr(C)]
pub struct OnEnterExitModShopArgs {
    player: *const *const std::ffi::c_void,
    enterexit: *const i32,
    interiorId: *const i32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnEnterExitModShop(args: *const EventArgs<OnEnterExitModShopArgs>) {
    let scripts = crate::runtime::Runtime.as_mut().unwrap();
    for script in scripts.iter_mut() {
        script.lock().unwrap().on_enter_exit_mod_shop(
            Player::new(*(*(*args).list).player),
            *(*(*args).list).enterexit != 0,
            *(*(*args).list).interiorId,
        );
    }
}

#[repr(C)]
pub struct OnVehicleSpawnArgs {
    vehicle: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnVehicleSpawn(args: *const EventArgs<OnVehicleSpawnArgs>) {
    let scripts = crate::runtime::Runtime.as_mut().unwrap();
    for script in scripts.iter_mut() {
        script
            .lock()
            .unwrap()
            .on_vehicle_spawn(Vehicle::new(*(*(*args).list).vehicle));
    }
}

#[repr(C)]
pub struct OnUnoccupiedVehicleUpdateArgs {
    vehicle: *const *const std::ffi::c_void,
    player: *const *const std::ffi::c_void,
    seat: *const i32,
    posX: *const f32,
    posY: *const f32,
    posZ: *const f32,
    velocityX: *const f32,
    velocityY: *const f32,
    velocityZ: *const f32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnUnoccupiedVehicleUpdate(
    args: *const EventArgs<OnUnoccupiedVehicleUpdateArgs>,
) -> bool {
    let scripts = crate::runtime::Runtime.as_mut().unwrap();
    let mut ret = false;
    for script in scripts.iter_mut() {
        ret = script.lock().unwrap().on_unoccupied_vehicle_update(
            Vehicle::new(*(*(*args).list).vehicle),
            Player::new(*(*(*args).list).player),
            UnoccupiedVehicleUpdate {
                seat: transmute(*(*(*args).list).seat),
                position: Vector3::new(
                    *(*(*args).list).posX,
                    *(*(*args).list).posY,
                    *(*(*args).list).posZ,
                ),
                velocity: Vector3::new(
                    *(*(*args).list).velocityX,
                    *(*(*args).list).velocityY,
                    *(*(*args).list).velocityZ,
                ),
            },
        );
        if crate::runtime::__terminate_event_chain {
            crate::runtime::__terminate_event_chain = false;
            return ret;
        }
    }
    ret
}

#[repr(C)]
pub struct OnTrailerUpdateArgs {
    player: *const *const std::ffi::c_void,
    trailer: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnTrailerUpdate(
    args: *const EventArgs<OnTrailerUpdateArgs>,
) -> bool {
    let scripts = crate::runtime::Runtime.as_mut().unwrap();
    let mut ret = false;
    for script in scripts.iter_mut() {
        ret = script.lock().unwrap().on_trailer_update(
            Player::new(*(*(*args).list).player),
            Vehicle::new(*(*(*args).list).trailer),
        );
        if crate::runtime::__terminate_event_chain {
            crate::runtime::__terminate_event_chain = false;
            return ret;
        }
    }
    ret
}

#[repr(C)]
pub struct OnVehicleSirenStateChangeArgs {
    player: *const *const std::ffi::c_void,
    vehicle: *const *const std::ffi::c_void,
    sirenState: *const i32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnVehicleSirenStateChange(
    args: *const EventArgs<OnVehicleSirenStateChangeArgs>,
) -> bool {
    let scripts = crate::runtime::Runtime.as_mut().unwrap();
    let mut ret = false;
    for script in scripts.iter_mut() {
        ret = script.lock().unwrap().on_vehicle_siren_state_change(
            Player::new(*(*(*args).list).player),
            Vehicle::new(*(*(*args).list).vehicle),
            *(*(*args).list).sirenState,
        );
        if crate::runtime::__terminate_event_chain {
            crate::runtime::__terminate_event_chain = false;
            return ret;
        }
    }
    ret
}
