#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(clippy::too_many_arguments)]

#[macro_use]
mod helper;
mod types;

mod actors;
mod models;
mod players;
mod utils;

pub use crate::utils::*;
pub use actors::*;
pub use models::*;
pub use omprs_codegen::{callback, main, native};
pub use players::*;

pub fn init_functions() {
    load_function!(SendClientMessage);
    load_function!(Print);
    load_function!(GetPlayerName);

    // models
    load_function!(AddCharModel);
    load_function!(AddSimpleModel);
    load_function!(AddSimpleModelTimed);
    load_function!(GetPlayerCustomSkin);
    load_function!(RedirectDownload);
    load_function!(FindModelFileNameFromCRC);
    load_function!(FindTextureFileNameFromCRC);
    load_function!(IsValidCustomModel);
    load_function!(GetCustomModelPath);

    load_player_functions();
    load_actor_functions();
}
