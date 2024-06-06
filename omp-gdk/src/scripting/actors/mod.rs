pub mod events;
pub mod functions;

pub use functions::load_functions;

use crate::players::Player;
use crate::types::animationdata::AnimationData;
use crate::types::vector::Vector3;

use std::ffi::c_void;

/// Main handler pointing to open.mp's Actor object
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

    /// Create a static 'actor' in the world.
    /// These 'actors' are like NPCs, however they have limited functionality.
    /// They do not take up server player slots.
    pub fn create_actor(skin: isize, pos: Vector3, angle: f32) -> Option<Actor> {
        functions::CreateActor(skin, pos, angle)
    }

    /// Destroys an Actor
    pub fn destroy(&self) {
        functions::DestroyActor(self)
    }

    /// Checks if an actor is streamed in for a player.
    pub fn is_streamed_in(&self, player: &Player) -> bool {
        functions::IsActorStreamedIn(self, player)
    }

    /// Sets an actor's virtual world
    pub fn set_virtual_world(&self, virtual_world: isize) {
        functions::SetActorVirtualWorld(self, virtual_world)
    }

    /// Get the virtual world of an actor.
    pub fn get_virtual_world(&self) -> isize {
        functions::GetActorVirtualWorld(self)
    }

    /// Apply an animation to an actor.
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

    /// Clear any animations that are applied to an actor.
    pub fn clear_animations(&self) -> bool {
        functions::ClearActorAnimations(self)
    }

    /// Set the position of an actor.
    pub fn set_pos(&self, pos: Vector3) {
        functions::SetActorPos(self, pos)
    }

    /// Get the position of an actor.
    pub fn get_pos(&self) -> Vector3 {
        let mut position = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        functions::GetActorPos(self, &mut position);
        position
    }

    /// Set the facing angle of an actor.
    pub fn set_facing_angle(&self, angle: f32) {
        functions::SetActorFacingAngle(self, angle)
    }

    /// Get the facing angle of an actor.
    pub fn get_facing_angle(&self) -> f32 {
        functions::GetActorFacingAngle(self)
    }

    /// Set the health of an actor.
    pub fn set_health(&self, health: f32) {
        functions::SetActorHealth(self, health)
    }

    /// Get the health of an actor
    pub fn get_health(&self) -> f32 {
        functions::GetActorHealth(self)
    }

    /// Set actor invulnerability
    pub fn set_invulnerable(&self, invulnerable: bool) {
        functions::SetActorInvulnerable(self, invulnerable)
    }

    /// Check if actor is invulnerable.
    pub fn is_invulnerable(&self) -> bool {
        functions::IsActorInvulnerable(self)
    }

    /// Set the skin of the actor.
    pub fn set_skin(&self, skin: isize) {
        functions::SetActorSkin(self, skin)
    }

    /// Get the skin of the actor.
    pub fn get_skin(&self) -> isize {
        functions::GetActorSkin(self)
    }

    /// Get the animation the actor is currently performing.
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

    /// Get the initial spawn point of the actor.
    pub fn get_spawn_info(&self) -> ActorSpawnData {
        let mut spawn_data = ActorSpawnData::default();
        functions::GetActorSpawnInfo(self, &mut spawn_data);
        spawn_data
    }

    /// Get id of the Actor object
    pub fn get_id(&self) -> usize {
        functions::GetActorID(self)
    }

    /// Get the Actor object from an id
    pub fn from_id(actorid: isize) -> Option<Actor> {
        functions::GetActorFromID(actorid)
    }
}

/// Actor's spawn information
#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct ActorSpawnData {
    pub position: Vector3,
    pub facingAngle: f32,
    pub skin: isize,
}
