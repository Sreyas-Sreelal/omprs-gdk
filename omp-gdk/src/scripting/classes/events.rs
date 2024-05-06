use omp_codegen::callback;

use crate::players::Player;

callback!(onPlayerRequestClass, player: Player,classId: usize, -> bool);
