use crate::actors::*;
use crate::{players::Player, types::vector::Vector3};
use omp_codegen::native;
use std::ffi::c_void;

native!(CreateActor, skin: isize, pos:Vector3, angle: f32, -> struct Actor);
native!(DestroyActor, actor: struct Actor);
native!(IsActorStreamedIn, actor: struct Actor, player: struct Player, -> bool);
native!(SetActorVirtualWorld, actor: struct Actor, virtualWorld: isize);
native!(GetActorVirtualWorld, actor: struct Actor, -> isize);
native!(ApplyActorAnimation,actor: struct Actor, animationLibrary: str, animationName: str, delta: f32, animloop: bool, lockX: bool, lockY: bool, freeze: bool, time: usize);
native!(ClearActorAnimations, actor: struct Actor, -> bool);
native!(SetActorPos, actor: struct Actor, pos:Vector3);
native!(GetActorPos, actor: struct Actor, position:mut Vector3);
native!(SetActorFacingAngle, actor: struct Actor, angle: f32);
native!(GetActorFacingAngle, actor: struct Actor, -> f32);
native!(SetActorHealth, actor: struct Actor, health: f32);
native!(GetActorHealth, actor: struct Actor, -> f32);
native!(SetActorInvulnerable, actor: struct Actor, invulnerable:bool);
native!(IsActorInvulnerable, actor: struct Actor, -> bool);
native!(SetActorSkin, actor: struct Actor, skin: isize);
native!(GetActorSkin, actor: struct Actor, -> isize);
native!(GetActorAnimation,actor: struct Actor, animationLibrary: mut str, animationName: mut str, delta: mut f32, animloop: mut bool, lockX: mut bool, lockY: mut bool, freeze: mut bool, time: mut usize,->bool);
native!(GetActorSpawnInfo, actor: struct Actor, spawnData:mut ActorSpawnData);
native!(GetActorID,actor:struct Actor,-> usize);
native!(GetActorFromID,actorid:isize,-> struct Actor);

#[doc(hidden)]
pub fn load_functions() {
    load_function!(CreateActor);
    load_function!(DestroyActor);
    load_function!(IsActorStreamedIn);
    load_function!(SetActorVirtualWorld);
    load_function!(GetActorVirtualWorld);
    load_function!(ApplyActorAnimation);
    load_function!(ClearActorAnimations);
    load_function!(SetActorPos);
    load_function!(GetActorPos);
    load_function!(SetActorFacingAngle);
    load_function!(GetActorFacingAngle);
    load_function!(SetActorHealth);
    load_function!(GetActorHealth);
    load_function!(SetActorInvulnerable);
    load_function!(IsActorInvulnerable);
    load_function!(SetActorSkin);
    load_function!(GetActorSkin);
    load_function!(GetActorAnimation);
    load_function!(GetActorSpawnInfo);
    load_function!(GetActorID);
    load_function!(GetActorFromID);
}
