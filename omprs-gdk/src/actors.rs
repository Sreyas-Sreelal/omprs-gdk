use omprs_codegen::native;

use crate::{hybridstring::HybridString, types::vector3::Vector3};

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
native!(GetActorsComponent, ->  &'static ActorsComponent);

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
    load_function!(GetActorsComponent);
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct AnimationData {
    delta: f32,
    looping: bool,
    lockX: bool,
    lockY: bool,
    freeze: bool,
    time: usize,
    lib: HybridString<16>,
    name: HybridString<24>,
}

impl AnimationData {
    pub fn new(
        delta: f32,
        looping: bool,
        lockX: bool,
        lockY: bool,
        freeze: bool,
        time: usize,
        lib: &str,
        name: &str,
    ) -> Self {
        AnimationData {
            delta,
            looping,
            lockX,
            lockY,
            freeze,
            time,
            lib: lib.into(),
            name: name.into(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_animation_library(&self) -> String {
        self.lib.to_string()
    }
}
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ActorSpawnData {
    position: Vector3,
    facingAngle: f32,
    skin: isize,
}

#[repr(C)]
pub struct ActorsComponentVtbl {
    _pvt: [u8; std::mem::size_of::<isize>() * 17],
    pub create: extern "thiscall" fn(
        this: *const ActorsComponent,
        skin: isize,
        pos: Vector3,
        angle: f32,
    ) -> &'static Actor,
}

#[repr(C)]
pub struct ActorsComponent {
    pub vtbl: &'static ActorsComponentVtbl,
}
impl ActorsComponent {
    #[inline]
    pub fn create(&self, skin: isize, pos: Vector3, angle: f32) -> &'static Actor {
        (self.vtbl.create)(self, skin, pos, angle)
    }
}

#[repr(C)]
pub struct ActorVtbl {
    _pvt: [u8; std::mem::size_of::<isize>() * 5],
    pub setSkin: extern "thiscall" fn(this: *const Actor, id: isize),
    pub getSkin: extern "thiscall" fn(this: *const Actor) -> isize,
    // Apply an animation for the actor
    pub applyAnimation: extern "thiscall" fn(this: *const Actor, animation: &AnimationData),

    pub getAnimation: extern "thiscall" fn(this: *const Actor) -> &'static AnimationData,

    // Clear the actor's animations
    pub clearAnimations: extern "thiscall" fn(this: *const Actor),

    // Set the actor's health
    pub setHealth: extern "thiscall" fn(this: *const Actor, health: f32),

    // Get the actor's health
    pub getHealth: extern "thiscall" fn(this: *const Actor) -> f32,

    // Set whether the actor is invulnerable
    pub setInvulnerable: extern "thiscall" fn(this: *const Actor, invuln: bool),

    // Get whether the actor is invulnerable
    pub isInvulnerable: extern "thiscall" fn(this: *const Actor) -> bool,

    /* // Checks if actor is streamed for a player
    pub isStreamedInForPlayer: fn(const IPlayer& player)->bool;

    // Streams actor for a player
    virtual void streamInForPlayer(IPlayer& player) = 0;

    // Streams out actor for a player
    virtual void streamOutForPlayer(IPlayer& player) = 0; */
    _p: [u8; std::mem::size_of::<isize>() * 3],

    // Get actor spawn data
    pub getSpawnData: extern "thiscall" fn(this: *const Actor) -> &'static ActorSpawnData,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Actor {
    pub vtbl: &'static ActorVtbl,
}

impl Actor {
    #[inline]
    pub fn setSkin(&self, id: isize) {
        (self.vtbl.setSkin)(self, id);
    }

    #[inline]
    pub fn getSkin(&self) -> isize {
        (self.vtbl.getSkin)(self)
    }

    #[inline]
    pub fn applyAnimation(&self, animation_data: &AnimationData) {
        (self.vtbl.applyAnimation)(self, animation_data);
    }

    #[inline]
    pub fn clearAnimations(&self) {
        (self.vtbl.clearAnimations)(self);
    }

    #[inline]
    pub fn getAnimation(&self) -> &'static AnimationData {
        (self.vtbl.getAnimation)(self)
    }

    #[inline]
    pub fn setHealth(&self, health: f32) {
        (self.vtbl.setHealth)(self, health);
    }

    #[inline]
    pub fn getHealth(&self) -> f32 {
        (self.vtbl.getHealth)(self)
    }

    #[inline]
    pub fn setInvulnerable(&self, invulnerable: bool) {
        (self.vtbl.setInvulnerable)(self, invulnerable);
    }

    #[inline]
    pub fn isInvulnerable(&self) -> bool {
        (self.vtbl.isInvulnerable)(self)
    }

    #[inline]
    pub fn getSpawnData(&self) -> &ActorSpawnData {
        (self.vtbl.getSpawnData)(self)
    }
}
