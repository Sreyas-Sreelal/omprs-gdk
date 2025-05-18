#![allow(clippy::all)]
use std::mem::transmute;
use std::rc::Rc;

use super::Player;
use crate::events::EventArgs;
use crate::objects::{Object, PlayerObject};
use crate::types::stringview::StringView;
use crate::types::vector::Vector3;
use crate::vehicles::Vehicle;

#[repr(C)]
pub struct OnPlayerConnectArgs {
    player: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerConnect(args: *const EventArgs<OnPlayerConnectArgs>) {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        script.on_player_connect(Player::new(*(*(*args).list).player));
    }
}

#[repr(C)]
pub struct OnPlayerSpawnArgs {
    player: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerSpawn(args: *const EventArgs<OnPlayerSpawnArgs>) {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        script.on_player_spawn(Player::new(*(*(*args).list).player));
    }
}

#[repr(C)]
pub struct OnPlayerCommandTextArgs {
    player: *const *const std::ffi::c_void,
    command: *const StringView,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerCommandText(
    args: *const EventArgs<OnPlayerCommandTextArgs>,
) -> bool {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    let mut ret = false;
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        ret = script.on_player_command_text(
            Player::new(*(*(*args).list).player),
            (*(*(*args).list).command).get_data(),
        );
        if crate::runtime::__terminate_event_chain {
            crate::runtime::__terminate_event_chain = false;
            return ret;
        }
    }
    ret
}

#[repr(C)]
pub struct OnPlayerKeyStateChangeArgs {
    player: *const *const std::ffi::c_void,
    newKeys: *const i32,
    oldKeys: *const i32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerKeyStateChange(
    args: *const EventArgs<OnPlayerKeyStateChangeArgs>,
) {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        script.on_player_key_state_change(
            Player::new(*(*(*args).list).player),
            *(*(*args).list).newKeys,
            *(*(*args).list).oldKeys,
        );
    }
}

#[repr(C)]
pub struct OnIncomingConnectionArgs {
    player: *const *const std::ffi::c_void,
    ipAddress: *const StringView,
    port: *const i32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnIncomingConnection(
    args: *const EventArgs<OnIncomingConnectionArgs>,
) {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        script.on_incoming_connection(
            Player::new(*(*(*args).list).player),
            (*(*(*args).list).ipAddress).get_data(),
            *(*(*args).list).port,
        );
    }
}

#[repr(C)]
pub struct OnPlayerDisconnectArgs {
    player: *const *const std::ffi::c_void,
    reason: *const i32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerDisconnect(args: *const EventArgs<OnPlayerDisconnectArgs>) {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        script.on_player_disconnect(
            Player::new(*(*(*args).list).player),
            transmute(*(*(*args).list).reason),
        );
    }
}

#[repr(C)]
pub struct OnPlayerRequestSpawnArgs {
    player: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerRequestSpawn(
    args: *const EventArgs<OnPlayerRequestSpawnArgs>,
) -> bool {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    let mut ret = false;
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        ret = script.on_player_request_spawn(Player::new(*(*(*args).list).player));
        if crate::runtime::__terminate_event_chain {
            crate::runtime::__terminate_event_chain = false;
            return ret;
        }
    }
    ret
}

#[repr(C)]
pub struct OnPlayerStreamInArgs {
    player: *const *const std::ffi::c_void,
    forPlayer: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerStreamIn(args: *const EventArgs<OnPlayerStreamInArgs>) {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        script.on_player_stream_in(
            Player::new(*(*(*args).list).player),
            Player::new(*(*(*args).list).forPlayer),
        );
    }
}

#[repr(C)]
pub struct OnPlayerStreamOutArgs {
    player: *const *const std::ffi::c_void,
    forPlayer: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerStreamOut(args: *const EventArgs<OnPlayerStreamOutArgs>) {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        script.on_player_stream_out(
            Player::new(*(*(*args).list).player),
            Player::new(*(*(*args).list).forPlayer),
        );
    }
}

#[repr(C)]
pub struct OnPlayerTextArgs {
    player: *const *const std::ffi::c_void,
    text: *const StringView,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerText(args: *const EventArgs<OnPlayerTextArgs>) -> bool {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    let mut ret = false;
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        ret = script.on_player_text(
            Player::new(*(*(*args).list).player),
            (*(*(*args).list).text).get_data(),
        );
        if crate::runtime::__terminate_event_chain {
            crate::runtime::__terminate_event_chain = false;
            return ret;
        }
    }
    ret
}

#[repr(C)]
pub struct OnPlayerShotMissedArgs {
    player: *const *const std::ffi::c_void,
    weapon: *const i32,
    x: *const f32,
    y: *const f32,
    z: *const f32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerShotMissed(
    args: *const EventArgs<OnPlayerShotMissedArgs>,
) -> bool {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    let mut ret = false;
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        ret = script.on_player_shot_missed(
            Player::new(*(*(*args).list).player),
            transmute(*(*(*args).list).weapon as u8),
            Vector3::new(*(*(*args).list).x, *(*(*args).list).y, *(*(*args).list).z),
        );
        if crate::runtime::__terminate_event_chain {
            crate::runtime::__terminate_event_chain = false;
            return ret;
        }
    }
    ret
}

#[repr(C)]
pub struct OnPlayerShotPlayerArgs {
    player: *const *const std::ffi::c_void,
    target: *const *const std::ffi::c_void,
    weapon: *const i32,
    x: *const f32,
    y: *const f32,
    z: *const f32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerShotPlayer(
    args: *const EventArgs<OnPlayerShotPlayerArgs>,
) -> bool {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    let mut ret = false;
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        ret = script.on_player_shot_player(
            Player::new(*(*(*args).list).player),
            Player::new(*(*(*args).list).target),
            transmute(*(*(*args).list).weapon as u8),
            Vector3::new(*(*(*args).list).x, *(*(*args).list).y, *(*(*args).list).z),
        );
        if crate::runtime::__terminate_event_chain {
            crate::runtime::__terminate_event_chain = false;
            return ret;
        }
    }
    ret
}

#[repr(C)]
pub struct OnPlayerShotVehicleArgs {
    player: *const *const std::ffi::c_void,
    target: *const *const std::ffi::c_void,
    weapon: *const i32,
    x: *const f32,
    y: *const f32,
    z: *const f32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerShotVehicle(
    args: *const EventArgs<OnPlayerShotVehicleArgs>,
) -> bool {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    let mut ret = false;
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        ret = script.on_player_shot_vehicle(
            Player::new(*(*(*args).list).player),
            Vehicle::new(*(*(*args).list).target),
            transmute(*(*(*args).list).weapon as u8),
            Vector3::new(*(*(*args).list).x, *(*(*args).list).y, *(*(*args).list).z),
        );
        if crate::runtime::__terminate_event_chain {
            crate::runtime::__terminate_event_chain = false;
            return ret;
        }
    }
    ret
}

#[repr(C)]
pub struct OnPlayerShotObjectArgs {
    player: *const *const std::ffi::c_void,
    target: *const *const std::ffi::c_void,
    weapon: *const i32,
    x: *const f32,
    y: *const f32,
    z: *const f32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerShotObject(
    args: *const EventArgs<OnPlayerShotObjectArgs>,
) -> bool {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    let mut ret = false;
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        ret = script.on_player_shot_object(
            Player::new(*(*(*args).list).player),
            Object::new(*(*(*args).list).target),
            transmute(*(*(*args).list).weapon as u8),
            Vector3::new(*(*(*args).list).x, *(*(*args).list).y, *(*(*args).list).z),
        );
        if crate::runtime::__terminate_event_chain {
            crate::runtime::__terminate_event_chain = false;
            return ret;
        }
    }
    ret
}

#[repr(C)]
pub struct OnPlayerShotPlayerObjectArgs {
    player: *const *const std::ffi::c_void,
    target: *const *const std::ffi::c_void,
    weapon: *const i32,
    x: *const f32,
    y: *const f32,
    z: *const f32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerShotPlayerObject(
    args: *const EventArgs<OnPlayerShotPlayerObjectArgs>,
) -> bool {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    let mut ret = false;
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        ret = script.on_player_shot_player_object(
            Player::new(*(*(*args).list).player),
            PlayerObject::new(
                *(*(*args).list).target,
                Player::new(*(*(*args).list).player),
            ),
            transmute(*(*(*args).list).weapon as u8),
            Vector3::new(*(*(*args).list).x, *(*(*args).list).y, *(*(*args).list).z),
        );
        if crate::runtime::__terminate_event_chain {
            crate::runtime::__terminate_event_chain = false;
            return ret;
        }
    }
    ret
}

#[repr(C)]
pub struct OnPlayerDeathArgs {
    player: *const *const std::ffi::c_void,
    killer: *const *const std::ffi::c_void,
    reason: *const i32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerDeath(args: *const EventArgs<OnPlayerDeathArgs>) {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    for script in scripts.iter() {
        let killer = if (*(*(*args).list).killer).is_null() {
            None
        } else {
            Some(Player::new(*(*(*args).list).killer))
        };
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        script.on_player_death(
            Player::new(*(*(*args).list).player),
            killer,
            *(*(*args).list).reason,
        );
    }
}

#[repr(C)]
pub struct OnPlayerTakeDamageArgs {
    player: *const *const std::ffi::c_void,
    from: *const *const std::ffi::c_void,
    amount: *const f32,
    weapon: *const i32,
    bodypart: *const i32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerTakeDamage(args: *const EventArgs<OnPlayerTakeDamageArgs>) {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    for script in scripts.iter() {
        let from = if (*(*(*args).list).from).is_null() {
            None
        } else {
            Some(Player::new(*(*(*args).list).from))
        };
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        script.on_player_take_damage(
            Player::new(*(*(*args).list).player),
            from,
            *(*(*args).list).amount,
            *(*(*args).list).weapon,
            transmute(*(*(*args).list).bodypart),
        );
    }
}

#[repr(C)]
pub struct OnPlayerGiveDamageArgs {
    player: *const *const std::ffi::c_void,
    to: *const *const std::ffi::c_void,
    amount: *const f32,
    weapon: *const i32,
    bodypart: *const i32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerGiveDamage(args: *const EventArgs<OnPlayerGiveDamageArgs>) {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        script.on_player_give_damage(
            Player::new(*(*(*args).list).player),
            Player::new(*(*(*args).list).to),
            *(*(*args).list).amount,
            *(*(*args).list).weapon,
            transmute(*(*(*args).list).bodypart),
        );
    }
}

#[repr(C)]
pub struct OnPlayerInteriorChangeArgs {
    player: *const *const std::ffi::c_void,
    newInterior: *const i32,
    oldInterior: *const i32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerInteriorChange(
    args: *const EventArgs<OnPlayerInteriorChangeArgs>,
) {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        script.on_player_interior_change(
            Player::new(*(*(*args).list).player),
            *(*(*args).list).newInterior,
            *(*(*args).list).oldInterior,
        );
    }
}

#[repr(C)]
pub struct OnPlayerStateChangeArgs {
    player: *const *const std::ffi::c_void,
    newState: *const i32,
    oldState: *const i32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerStateChange(
    args: *const EventArgs<OnPlayerStateChangeArgs>,
) {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        script.on_player_state_change(
            Player::new(*(*(*args).list).player),
            transmute(*(*(*args).list).newState),
            transmute(*(*(*args).list).oldState),
        );
    }
}

#[repr(C)]
pub struct OnPlayerClickMapArgs {
    player: *const *const std::ffi::c_void,
    x: *const f32,
    y: *const f32,
    z: *const f32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerClickMap(args: *const EventArgs<OnPlayerClickMapArgs>) {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        script.on_player_click_map(
            Player::new(*(*(*args).list).player),
            Vector3::new(*(*(*args).list).x, *(*(*args).list).y, *(*(*args).list).z),
        );
    }
}

#[repr(C)]
pub struct OnPlayerClickPlayerArgs {
    player: *const *const std::ffi::c_void,
    clicked: *const *const std::ffi::c_void,
    source: *const i32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerClickPlayer(
    args: *const EventArgs<OnPlayerClickPlayerArgs>,
) {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        script.on_player_click_player(
            Player::new(*(*(*args).list).player),
            Player::new(*(*(*args).list).clicked),
            transmute(*(*(*args).list).source),
        );
    }
}

#[repr(C)]
pub struct OnClientCheckResponseArgs {
    player: *const *const std::ffi::c_void,
    actionType: *const i32,
    address: *const i32,
    result: *const i32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnClientCheckResponse(
    args: *const EventArgs<OnClientCheckResponseArgs>,
) {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        script.on_client_check_response(
            Player::new(*(*(*args).list).player),
            *(*(*args).list).actionType,
            *(*(*args).list).address,
            *(*(*args).list).result,
        );
    }
}

#[repr(C)]
pub struct OnPlayerUpdateArgs {
    player: *const *const std::ffi::c_void,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerUpdate(args: *const EventArgs<OnPlayerUpdateArgs>) -> bool {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    let mut ret = false;
    for script in scripts.iter() {
        let script = &mut *(*Rc::as_ptr(script)).as_ptr();
        ret = script.on_player_update(Player::new(*(*(*args).list).player));
        if crate::runtime::__terminate_event_chain {
            crate::runtime::__terminate_event_chain = false;
            return ret;
        }
    }
    ret
}
