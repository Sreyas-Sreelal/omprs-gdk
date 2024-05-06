use omp_codegen::callback;

use crate::players::Player;

callback!(OnRconCommand, cmd: String,->bool);
callback!(OnRconLoginAttempt,player: Option<Player>, ip: String, password: String, success:bool);
