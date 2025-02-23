#![allow(clippy::all)]
use crate::{events::EventArgs, players::Player};
use std::mem::transmute;

#[repr(C)]
pub struct OnPlayerFinishedDownloadingArgs {
    player: *const *const std::ffi::c_void,
    vw: *const i32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerFinishedDownloading(
    args: *const EventArgs<OnPlayerFinishedDownloadingArgs>,
) {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    for script in scripts.iter_mut() {
        script
            .borrow_mut()
            .on_player_finished_downloading(Player::new(*(*(*args).list).player));
    }
}

#[repr(C)]
pub struct OnPlayerRequestDownloadArgs {
    player: *const *const std::ffi::c_void,
    model_type: *const i32,
    checksum: *const i32,
}

#[no_mangle]
pub unsafe extern "C" fn OMPRS_OnPlayerRequestDownload(
    args: *const EventArgs<OnPlayerRequestDownloadArgs>,
) {
    let scripts = (&raw mut crate::runtime::Runtime)
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    for script in scripts.iter_mut() {
        script.borrow_mut().on_player_request_download(
            Player::new(*(*(*args).list).player),
            transmute(*(*(*args).list).model_type),
            *(*(*args).list).checksum,
        );
    }
}
