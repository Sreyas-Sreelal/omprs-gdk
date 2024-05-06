use omp_codegen::callback;

use crate::players::Player;

use super::GangZone;

callback!(onPlayerEnterGangZone, player: Player,zone: GangZone);
callback!(onPlayerLeaveGangZone, player: Player,zone: GangZone);
callback!(onPlayerClickGangZone, player: Player,zone: GangZone);
