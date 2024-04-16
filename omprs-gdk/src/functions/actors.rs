use omprs_codegen::native;

native!(CreateActor,skin: isize, x: f32, y: f32, z: f32, angle: f32,-> isize);
native!(DestroyActor,actorid: isize,->bool);
native!(IsActorStreamedIn,actorid: isize, playerid: isize,->bool);
native!(SetActorVirtualWorld,actorid: isize, virtualWorld: isize,->bool);
native!(GetActorVirtualWorld,actorid: isize,-> isize);
native!(ApplyActorAnimation,actorid: isize, animationLibrary: str, animationName: str, delta: f32, animloop: bool, lockX: bool, lockY: bool, freeze: bool, time: isize,->bool);
native!(ClearActorAnimations,actorid: isize,->bool);
native!(SetActorPos,actorid: isize, x: f32, y: f32, z: f32,->bool);
native!(GetActorPos,actorid: isize, x: mut f32, y: mut f32, z: mut f32,->bool);
native!(SetActorFacingAngle,actorid: isize, angle: f32,->bool);
native!(GetActorFacingAngle,actorid: isize, angle: mut f32,->bool);
native!(SetActorHealth,actorid: isize, health: f32,->bool);
native!(GetActorHealth,actorid: isize, health: mut f32,->bool);
native!(SetActorInvulnerable,actorid: isize, invulnerable: bool,->bool);
native!(IsActorInvulnerable,actorid: isize,->bool);
native!(IsValidActor,actorid: isize,->bool);
native!(SetActorSkin,actorid: isize, skin: isize,->bool);
native!(GetActorSkin,actorid: isize,-> isize);
native!(GetActorAnimation,actorid: isize, animationLibrary: mut str, animationName: mut str, delta: mut f32, animloop: mut bool, lockX: mut bool, lockY: mut bool, freeze: mut bool, time: mut usize,->bool);
native!(GetActorSpawnInfo,actorid: isize, skin: mut isize, x: mut f32, y: mut f32, z: mut f32, angle: mut f32,->bool);

pub fn load_actor_functions() {
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
    load_function!(IsValidActor);
    load_function!(SetActorSkin);
    load_function!(GetActorSkin);
    load_function!(GetActorAnimation);
    load_function!(GetActorSpawnInfo);
}
