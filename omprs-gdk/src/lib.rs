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
pub use scripting::core;
pub use scripting::models;
pub use scripting::objects;
pub use scripting::players;
pub use scripting::vehicles;
pub use types::*;

pub fn init_functions() {
    core::load_functions();
    // models
    models::load_functions();
    players::load_functions();
    actors::load_functions();
    checkpoints::load_functions();
}
