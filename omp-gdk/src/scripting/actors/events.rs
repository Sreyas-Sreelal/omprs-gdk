use omp_codegen::callback;

use crate::players::{BodyPart, Player};

use super::Actor;

callback!(onPlayerGiveDamageActor, player:Player,actor:Actor,amount:f32,weapon:usize,part:BodyPart);
callback!(onActorStreamIn, actor:Actor, player:Player);
callback!(onActorStreamOut, actor:Actor, player:Player);
