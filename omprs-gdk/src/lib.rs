#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(clippy::too_many_arguments)]

#[macro_use]
mod helper;
mod types;

mod events;
mod scripting;

#[macro_use]
mod runtime;

pub use events::Events;
pub use omprs_codegen::main;
pub use runtime::Runtime;
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
pub use scripting::vehicles;
pub use types::*;

pub fn init_functions() {
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
}
