#![allow(clippy::all)]
use crate::{events::EventArgs, players::Player, runtime::each_module};
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
    each_module(move |mut script| {
        script.on_player_finished_downloading(Player::new(*(*(*args).list).player));
        None
    });
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
    each_module(move |mut script| {
        script.on_player_request_download(
            Player::new(*(*(*args).list).player),
            transmute(*(*(*args).list).model_type),
            *(*(*args).list).checksum,
        );
        None
    });
}
