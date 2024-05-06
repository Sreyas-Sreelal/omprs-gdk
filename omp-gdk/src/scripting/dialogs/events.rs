use omp_codegen::callback;

use crate::players::Player;

use super::DialogResponse;

callback!(onDialogResponse, player: Player, dialog_id: i16, response: DialogResponse, list_item: isize, input_text: String);
