#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(clippy::too_many_arguments)]

#[macro_use]
mod helper;
mod types;

mod actors;
mod functions;
mod models;
mod objects;
mod players;
mod utils;
mod vehicles;

use functions::{actors::load_actor_functions, players::load_player_functions};

pub use crate::utils::*;
pub use actors::*;
//pub use models::*;
pub use objects::*;
pub use omprs_codegen::{callback, main, native};
pub use players::*;
pub use types::*;
pub use vehicles::*;

pub fn init_functions() {
    load_function!(Print);
    // models
    /* load_function!(AddCharModel);
    load_function!(AddSimpleModel);
    load_function!(AddSimpleModelTimed);
    load_function!(GetPlayerCustomSkin);
    load_function!(RedirectDownload);
    load_function!(FindModelFileNameFromCRC);
    load_function!(FindTextureFileNameFromCRC);
    load_function!(IsValidCustomModel);
    load_function!(GetCustomModelPath); */

    load_player_functions();
    load_actor_functions();
}
