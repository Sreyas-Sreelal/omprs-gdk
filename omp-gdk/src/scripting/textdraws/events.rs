use omp_codegen::callback;

use crate::players::Player;

use super::{PlayerTextDraw, TextDraw};

callback!(onPlayerCancelTextDrawSelection, player: Player,->bool);
callback!(onPlayerCancelPlayerTextDrawSelection, player: Player,->bool);
callback!(onPlayerClickTextDraw, player:Player, textdraw: TextDraw);
callback!(onPlayerClickPlayerTextDraw, player:Player, textdraw: PlayerTextDraw);
