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

#[doc(hidden)]
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
    textdraws::load_functions();
    textlabels::load_functions();
    vehicles::load_functions();
}
