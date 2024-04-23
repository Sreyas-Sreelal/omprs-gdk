use crate::players::Player;
use omprs_codegen::callback;

callback!(onPlayerEnterCheckpoint, player:Player);
callback!(onPlayerLeaveCheckpoint, player:Player);
callback!(onPlayerEnterRaceCheckpoint, player:Player);
callback!(onPlayerLeaveRaceCheckpoint, player:Player);
