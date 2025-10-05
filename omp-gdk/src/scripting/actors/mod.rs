pub mod events;
pub mod functions;

pub use functions::load_functions;

use crate::players::Player;
use crate::runtime::queue_api_call;
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
    pub fn create_actor(skin: i32, pos: Vector3, angle: f32) -> Option<Actor> {
        let mut _id: i32 = 0;
        functions::Actor_Create(skin, pos.x, pos.y, pos.z, angle, &mut _id)
    }

    /// Destroys an Actor
    pub fn destroy(&self) {
        self.defer_api_call(Box::new(move |actor| {
            functions::Actor_Destroy(&actor);
        }));
    }

    /// Checks if an actor is streamed in for a player.
    pub fn is_streamed_in(&self, player: &Player) -> bool {
        functions::Actor_IsStreamedInFor(self, player)
    }

    /// Sets an actor's virtual world
    pub fn set_virtual_world(&self, virtual_world: i32) -> bool {
        functions::Actor_SetVirtualWorld(self, virtual_world)
    }

    /// Get the virtual world of an actor.
    pub fn get_virtual_world(&self) -> i32 {
        functions::Actor_GetVirtualWorld(self)
    }

    /// Apply an animation to an actor.
    pub fn apply_animation(&self, animation_data: AnimationData) -> bool {
        functions::Actor_ApplyAnimation(
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
        functions::Actor_ClearAnimations(self)
    }

    /// Set the position of an actor.
    pub fn set_pos(&self, pos: Vector3) {
        self.defer_api_call(Box::new(move |actor| {
            functions::Actor_SetPos(&actor, pos.x, pos.y, pos.z);
        }));
    }

    /// Get the position of an actor.
    pub fn get_pos(&self) -> Vector3 {
        let mut position = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        functions::Actor_GetPos(self, &mut position.x, &mut position.y, &mut position.z);
        position
    }

    /// Set the facing angle of an actor.
    pub fn set_facing_angle(&self, angle: f32) -> bool {
        functions::Actor_SetFacingAngle(self, angle)
    }

    /// Get the facing angle of an actor.
    pub fn get_facing_angle(&self) -> f32 {
        functions::Actor_GetFacingAngle(self)
    }

    /// Set the health of an actor.
    pub fn set_health(&self, health: f32) {
        self.defer_api_call(Box::new(move |actor| {
            functions::Actor_SetHealth(&actor, health);
        }));
    }

    /// Get the health of an actor
    pub fn get_health(&self) -> f32 {
        functions::Actor_GetHealth(self)
    }

    /// Set actor invulnerability
    pub fn set_invulnerable(&self, invulnerable: bool) -> bool {
        functions::Actor_SetInvulnerable(self, invulnerable)
    }

    /// Check if actor is invulnerable.
    pub fn is_invulnerable(&self) -> bool {
        functions::Actor_IsInvulnerable(self)
    }

    /// Set the skin of the actor.
    pub fn set_skin(&self, skin: i32) -> bool {
        functions::Actor_SetSkin(self, skin)
    }

    /// Get the skin of the actor.
    pub fn get_skin(&self) -> i32 {
        functions::Actor_GetSkin(self)
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

        functions::Actor_GetAnimation(
            self,
            &mut animation_library,
            16,
            &mut animation_name,
            24,
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
        functions::Actor_GetSpawnInfo(
            self,
            &mut spawn_data.position.x,
            &mut spawn_data.position.y,
            &mut spawn_data.position.z,
            &mut spawn_data.facingAngle,
            &mut spawn_data.skin,
        );
        spawn_data
    }

    /// Get id of the Actor object
    pub fn get_id(&self) -> i32 {
        functions::Actor_GetID(self)
    }

    /// Get the Actor object from an id
    pub fn from_id(actorid: i32) -> Option<Actor> {
        functions::Actor_FromID(actorid)
    }

    fn defer_api_call(&self, callback: Box<dyn FnOnce(Self)>) {
        let actor_id = self.get_id();
        queue_api_call(Box::new(move || {
            let actor = match Self::from_id(actor_id) {
                Some(actor) => actor,
                None => {
                    log::error!("actor with id={actor_id} not found");
                    return;
                }
            };
            callback(actor);
        }));
    }
}

/// Actor's spawn information
#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct ActorSpawnData {
    pub position: Vector3,
    pub facingAngle: f32,
    pub skin: i32,
}
