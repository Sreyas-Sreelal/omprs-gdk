pub mod events;
pub mod functions;

pub use functions::load_functions;
//pub use events::ActorEvents;

use crate::animationdata::AnimationData;
use crate::players::Player;
use crate::types::vector::Vector3;

use std::ffi::c_void;

pub struct Actor {
    handle: *const c_void,
}

impl Actor {
    pub fn get_handle(&self) -> *const c_void {
        self.handle
    }

    pub fn new(handle: *const c_void) -> Self {
        Self { handle }
    }

    pub fn create_actor(skin: isize, pos: Vector3, angle: f32) -> Option<Actor> {
        functions::CreateActor(skin, pos, angle)
    }

    pub fn destroy(&self) {
        functions::DestroyActor(self)
    }

    pub fn is_streamed_in(&self, player: &Player) -> bool {
        functions::IsActorStreamedIn(self, player)
    }

    pub fn set_virtual_world(&self, virtual_world: isize) {
        functions::SetActorVirtualWorld(self, virtual_world)
    }

    pub fn get_virtual_world(&self) -> isize {
        functions::GetActorVirtualWorld(self)
    }

    pub fn apply_animation(&self, animation_data: AnimationData) {
        functions::ApplyActorAnimation(
            self,
            &animation_data.get_animation_library(),
            &animation_data.get_name(),
            animation_data.delta,
            animation_data.looping,
            animation_data.lockX,
            animation_data.lockY,
            animation_data.freeze,
            animation_data.time,
        )
    }

    pub fn clear_animations(&self) -> bool {
        functions::ClearActorAnimations(self)
    }

    pub fn set_pos(&self, pos: Vector3) {
        functions::SetActorPos(self, pos)
    }

    pub fn getpos(&self) -> Vector3 {
        let mut position = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        functions::GetActorPos(self, &mut position);
        position
    }

    pub fn setfacing_angle(&self, angle: f32) {
        functions::SetActorFacingAngle(self, angle)
    }

    pub fn getfacing_angle(&self) -> f32 {
        functions::GetActorFacingAngle(self)
    }

    pub fn sethealth(&self, health: f32) {
        functions::SetActorHealth(self, health)
    }

    pub fn gethealth(&self) -> f32 {
        functions::GetActorHealth(self)
    }

    pub fn setinvulnerable(&self, invulnerable: bool) {
        functions::SetActorInvulnerable(self, invulnerable)
    }

    pub fn isinvulnerable(&self) -> bool {
        functions::IsActorInvulnerable(self)
    }

    pub fn is_valid(&self) -> bool {
        functions::IsValidActor(self)
    }

    pub fn set_skin(&self, skin: isize) {
        functions::SetActorSkin(self, skin)
    }

    pub fn get_skin(&self) -> isize {
        functions::GetActorSkin(self)
    }

    pub fn get_animation(&self) -> AnimationData {
        let (
            mut animation_library,
            mut animation_name,
            mut delta,
            mut animloop,
            mut lock_x,
            mut lock_y,
            mut freeze,
            mut time,
        ) = Default::default();

        functions::GetActorAnimation(
            self,
            &mut animation_library,
            &mut animation_name,
            &mut delta,
            &mut animloop,
            &mut lock_x,
            &mut lock_y,
            &mut freeze,
            &mut time,
        );

        AnimationData::new(
            delta,
            animloop,
            lock_x,
            lock_y,
            freeze,
            time,
            &animation_library,
            &animation_name,
        )
    }

    pub fn get_spawn_info(&self) -> ActorSpawnData {
        let mut spawn_data = ActorSpawnData::default();
        functions::GetActorSpawnInfo(self, &mut spawn_data);
        spawn_data
    }
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct ActorSpawnData {
    position: Vector3,
    facingAngle: f32,
    skin: isize,
}
