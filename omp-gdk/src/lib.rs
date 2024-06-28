//! GDK crate, that does the core functionality like loading function address, executing, providing necessary types etc

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(clippy::too_many_arguments)]

#[macro_use]
mod helper;
pub mod types;

mod events;
mod scripting;

mod runtime;

use std::ffi::c_char;
use std::os::raw::c_void;

pub use events::Events;
pub use omp_codegen::main;
pub use runtime::{Runtime, __terminate_event_chain};
pub use scripting::actors;
pub use scripting::checkpoints;
pub use scripting::classes;
pub use scripting::core;
pub use scripting::dialogs;
pub use scripting::gangzones;
pub use scripting::menus;
pub use scripting::models;
pub use scripting::objects;
pub use scripting::pickups;
pub use scripting::players;
pub use scripting::textdraws;
pub use scripting::textlabels;
pub use scripting::vehicles;

pub use helper::gen_uid;

#[doc(hidden)]
pub fn init_functions() {
    load_function!(Component_Create);
    core::load_functions();
    models::load_functions();
    players::load_functions();
    actors::load_functions();
    checkpoints::load_functions();
    classes::load_functions();
    dialogs::load_functions();
    gangzones::load_functions();
    menus::load_functions();
    objects::load_functions();
    pickups::load_functions();
    textdraws::load_functions();
    textlabels::load_functions();
    vehicles::load_functions();
    events::load_event_functions();
}

#[repr(C)]
pub struct ComponentVersion {
    pub major: u8,   //< MAJOR version when you make incompatible API changes
    pub minor: u8,   //< MINOR version when you add functionality in a backwards compatible manner
    pub patch: u8,   //< PATCH version when you make backwards compatible bug fixes
    pub prerel: u16, //< PRE-RELEASE version
}

pub static mut OMPRS_Component_Create: Option<
    unsafe extern "C" fn(
        uid: u64,
        name: *const c_char,
        version: ComponentVersion,
        onLoadCB: *const c_void,
        onInitCB: *const c_void,
        onReadyCB: *const c_void,
        onResetCB: *const c_void,
        onFreeCB: *const c_void,
    ) -> *const c_void,
> = None;
