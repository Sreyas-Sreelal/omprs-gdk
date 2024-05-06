use omp_codegen::callback;

use crate::players::Player;

use super::Pickup;

callback!(onPlayerPickUpPickup, player: Player, pickup: Pickup);
