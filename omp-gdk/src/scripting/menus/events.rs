use omp_codegen::callback;

use crate::players::Player;

callback!(onPlayerSelectedMenuRow, player:Player, row:isize);
callback!(onPlayerExitedMenu, player:Player);
