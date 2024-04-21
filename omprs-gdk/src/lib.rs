#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(clippy::too_many_arguments)]

#[macro_use]
mod helper;
mod types;

mod actors;
mod events;
mod functions;
mod models;
mod objects;
mod players;
mod utils;
mod vehicles;

#[macro_use]
mod runtime;

use functions::{
    actors::load_actor_functions, models::load_model_functions, players::load_player_functions,
};

pub use crate::utils::*;
pub use actors::*;
//pub use models::*;
pub use events::Events;
pub use objects::*;
pub use omprs_codegen::{callback, main, native};
pub use players::*;
pub use runtime::Runtime;
pub use types::*;
pub use vehicles::*;

pub fn init_functions() {
    load_function!(Print);
    // models
    load_model_functions();

    load_player_functions();
    load_actor_functions();
}
