pub mod events;
pub mod functions;

use std::ffi::c_void;

use crate::{
    players::Player,
    types::vector::{Vector3, Vector4},
};

pub use functions::load_functions;

pub struct Vehicle {
    handle: *const c_void,
}

impl Vehicle {
    pub fn get_handle(&self) -> *const c_void {
        self.handle
    }
    pub fn new(handle: *const c_void) -> Self {
        Self { handle }
    }
    /// Creates a vehicle in the world.
    pub fn create(
        modelid: i32,
        pos: Vector3,
        rotation: f32,
        colour1: i32,
        colour2: i32,
        respawnDelay: i32,
        addSiren: bool,
    ) -> Option<Vehicle> {
        let mut _id = 0;
        functions::Vehicle_Create(
            modelid,
            pos.x,
            pos.y,
            pos.z,
            rotation,
            colour1,
            colour2,
            respawnDelay,
            addSiren,
            &mut _id,
        )
    }
    /// Gets the number of seats in the vehicle.
    pub fn get_seats(modelid: i32) -> i32 {
        functions::Vehicle_GetMaxPassengerSeats(modelid)
    }
    /// Destroy a vehicle.
    pub fn destroy(&self) -> bool {
        functions::Vehicle_Destroy(self)
    }
    /// Checks if a vehicle is streamed in for a player.
    pub fn is_streamed_in(&self, player: &Player) -> bool {
        functions::Vehicle_IsStreamedIn(self, player)
    }
    /// Gets the position of a vehicle.
    pub fn get_pos(&self) -> Vector3 {
        let mut pos = Vector3::default();
        functions::Vehicle_GetPos(self, &mut pos.x, &mut pos.y, &mut pos.z);
        pos
    }
    /// Set a vehicle's position.
    pub fn set_pos(&self, pos: Vector3) -> bool {
        functions::Vehicle_SetPos(self, pos.x, pos.y, pos.z)
    }
    /// Get the rotation of a vehicle on the Z axis (yaw).
    pub fn get_z_angle(&self) -> f32 {
        functions::Vehicle_GetZAngle(self)
    }
    /// Returns a vehicle's rotation on all axes as a quaternion.
    pub fn get_rotation_quat(&self) -> Vector4 {
        let mut rotation = Vector4::default();
        functions::Vehicle_GetRotationQuat(
            self,
            &mut rotation.w,
            &mut rotation.x,
            &mut rotation.y,
            &mut rotation.z,
        );
        rotation
    }
    /// This function can be used to calculate the distance (as a float) between a vehicle and another map coordinate.
    pub fn get_distance_from_point(&self, pos: Vector3) -> f32 {
        functions::Vehicle_GetDistanceFromPoint(self, pos.x, pos.y, pos.z)
    }
    /// Set the Z rotation (yaw) of a vehicle.
    pub fn set_z_angle(&self, angle: f32) -> bool {
        functions::Vehicle_SetZAngle(self, angle)
    }
    /// Set the parameters of a vehicle for a player.
    pub fn set_params_for_player(&self, player: &Player, params: VehicleParams) -> bool {
        functions::Vehicle_SetParamsForPlayer(
            self,
            player,
            params.objective as i32,
            params.doors as i32,
        )
    }
    pub fn use_manual_engine_and_lights() -> bool {
        functions::Vehicle_UseManualEngineAndLights()
    }
    pub fn set_params(&self, params: VehicleParams) -> bool {
        functions::Vehicle_SetParamsEx(
            self,
            params.engine as i32,
            params.lights as i32,
            params.alarm as i32,
            params.doors as i32,
            params.bonnet as i32,
            params.boot as i32,
            params.objective as i32,
        )
    }
    pub fn get_params(&self) -> VehicleParams {
        let mut params = VehicleParams::default();
        functions::Vehicle_GetParamsEx(
            self,
            &mut params.engine,
            &mut params.lights,
            &mut params.alarm,
            &mut params.doors,
            &mut params.bonnet,
            &mut params.boot,
            &mut params.objective,
        );
        params
    }
    /// Sets a vehicle back to the position at where it was created.
    pub fn set_to_respawn(&self) -> bool {
        functions::Vehicle_SetToRespawn(self)
    }
    /// Links a vehicle to an interior.
    pub fn link_to_interior(&self, interiorid: i32) -> bool {
        functions::Vehicle_LinkToInterior(self, interiorid)
    }
    /// Adds a 'component' (often referred to as a 'mod' (modification)) to a vehicle.
    pub fn add_component(&self, componentid: i32) -> bool {
        functions::Vehicle_AddComponent(self, componentid)
    }
    /// Remove a component from a vehicle.
    pub fn remove_component(&self, componentid: i32) -> bool {
        functions::Vehicle_RemoveComponent(self, componentid)
    }
    /// Change a vehicle's primary and secondary colors.
    pub fn change_color(&self, colour1: i32, colour2: i32) -> bool {
        functions::Vehicle_ChangeColor(self, colour1, colour2)
    }
    /// Change a vehicle's paintjob.
    pub fn change_paintjob(&self, paintjobid: i32) -> bool {
        functions::Vehicle_ChangePaintjob(self, paintjobid)
    }
    /// Set a vehicle's health.
    pub fn set_health(&self, health: f32) -> bool {
        functions::Vehicle_SetHealth(self, health)
    }
    /// Get the health of a vehicle.
    pub fn get_health(&self) -> f32 {
        functions::Vehicle_GetHealth(self)
    }
    /// Attach a vehicle to another vehicle as a trailer.
    pub fn attach_trailer(&self, trailer: &Vehicle) -> bool {
        functions::Vehicle_AttachTrailer(self, trailer)
    }
    /// Detach the connection between a vehicle and its trailer, if any.
    pub fn detach_trailer(&self) -> bool {
        functions::Vehicle_DetachTrailer(self)
    }
    /// Checks if a vehicle has a trailer attached to it.
    pub fn is_trailer_attached(&self) -> bool {
        functions::Vehicle_IsTrailerAttached(self)
    }
    /// Get the ID of the trailer attached to a vehicle.
    pub fn get_trailer(&self) -> Option<Vehicle> {
        functions::Vehicle_GetTrailer(self)
    }
    /// Set a vehicle numberplate.
    pub fn set_number_plate(&self, numberPlate: &str) -> bool {
        functions::Vehicle_SetNumberPlate(self, numberPlate)
    }
    /// Gets the model ID of a vehicle.
    pub fn get_model(&self) -> i32 {
        functions::Vehicle_GetModel(self)
    }
    /// Retrieves the installed component ID (modshop mod(ification)) on a vehicle in a specific slot.
    pub fn get_component_in_slot(&self, slot: i32) -> i32 {
        functions::Vehicle_GetComponentInSlot(self, slot)
    }
    /// Find out what type of component a certain ID is.
    pub fn get_component_type(componentid: i32) -> i32 {
        functions::Vehicle_GetComponentType(componentid)
    }
    /// Is the component legal on the vehicle?
    pub fn can_have_component(modelid: i32, componentid: i32) -> bool {
        functions::Vehicle_CanHaveComponent(modelid, componentid)
    }
    pub fn get_random_car_col_pair(modelid: i32) -> (i32, i32, i32, i32) {
        let (mut colour1, mut colour2, mut colour3, mut colour4) = Default::default();
        functions::Vehicle_GetRandomColorPair(
            modelid,
            &mut colour1,
            &mut colour2,
            &mut colour3,
            &mut colour4,
        );
        (colour1, colour2, colour3, colour4)
    }
    pub fn colour_index_to_colour(colourIndex: i32, alpha: i32) -> i32 {
        functions::Vehicle_ColorIndexToColor(colourIndex, alpha)
    }
    /// Fully repairs a vehicle, including visual damage (bumps, dents, scratches, popped tires etc.
    pub fn repair(&self) -> bool {
        functions::Vehicle_Repair(self)
    }
    /// Get the velocity of a vehicle on the X, Y and Z axes.
    pub fn get_velocity(&self) -> Vector3 {
        let mut velocity = Vector3::default();
        functions::Vehicle_GetVelocity(self, &mut velocity.x, &mut velocity.y, &mut velocity.z);
        velocity
    }
    /// Sets the X, Y and Z velocity of a vehicle.
    pub fn set_velocity(&self, velocity: Vector3) -> bool {
        functions::Vehicle_SetVelocity(self, velocity.x, velocity.y, velocity.z)
    }
    /// Sets the angular X, Y and Z velocity of a vehicle.
    pub fn set_angular_velocity(&self, velocity: Vector3) -> bool {
        functions::Vehicle_SetAngularVelocity(self, velocity.x, velocity.y, velocity.z)
    }
    /// Retrieve the damage statuses of a vehicle.
    pub fn get_damage_status(&self) -> VehicleDamageStatusData {
        let (mut panels, mut doors, mut lights, mut tires) = Default::default();

        functions::Vehicle_GetDamageStatus(self, &mut panels, &mut doors, &mut lights, &mut tires);

        VehicleDamageStatusData {
            panels,
            doors,
            lights,
            tires,
        }
    }
    /// Sets the various visual damage statuses of a vehicle, such as popped tires, broken lights and damaged panels.
    pub fn update_damage_status(&self, panels: i32, doors: i32, lights: i32, tires: i32) -> bool {
        functions::Vehicle_UpdateDamageStatus(self, panels, doors, lights, tires)
    }
    /// Retrieve information about a specific vehicle model such as the size or position of seats.
    pub fn get_model_info(model: i32, infotype: i32) -> Vector3 {
        let mut pos = Vector3::default();
        functions::Vehicle_GetModelInfo(model, infotype, &mut pos.x, &mut pos.y, &mut pos.z);
        pos
    }
    /// Sets the 'virtual world' of a vehicle.
    pub fn set_virtual_world(&self, virtualWorld: i32) -> bool {
        functions::Vehicle_SetVirtualWorld(self, virtualWorld)
    }
    /// Get the virtual world of a vehicle.
    pub fn get_virtual_world(&self) -> i32 {
        functions::Vehicle_GetVirtualWorld(self)
    }
    /// Gets the current vehicle landing gear state from the latest driver.
    pub fn get_landing_gear_state(&self) -> i32 {
        functions::Vehicle_GetLandingGearState(self)
    }
    /// Adds a 'static' vehicle (models are pre-loaded for players) to the gamemode.
    pub fn create_static(
        modelid: i32,
        spawn: Vector3,
        angle: f32,
        colour1: i32,
        colour2: i32,
        respawnDelay: i32,
        addSiren: bool,
    ) -> Option<Vehicle> {
        let mut _id = 0;
        functions::Vehicle_AddStaticEx(
            modelid,
            spawn.x,
            spawn.y,
            spawn.z,
            angle,
            colour1,
            colour2,
            respawnDelay,
            addSiren,
            &mut _id,
        )
    }

    /// Enable friendly fire for team vehicles.
    pub fn enable_friendly_fire() -> bool {
        functions::Vehicle_EnableFriendlyFire()
    }

    /// Gets the vehicle spawn location and colours.
    pub fn get_spawn_info(&self) -> VehicleSpawnData {
        let (
            mut position,
            mut rotation,
            mut colour1,
            mut colour2,
            respawnDelay,
            modelID,
            siren,
            interior,
        ): (Vector3, f32, i32, i32, i32, i32, bool, i32) = Default::default();

        functions::Vehicle_GetSpawnInfo(
            self,
            &mut position.x,
            &mut position.y,
            &mut position.z,
            &mut rotation,
            &mut colour1,
            &mut colour2,
        );

        VehicleSpawnData {
            position,
            rotation,
            colour1,
            colour2,
            respawnDelay,
            modelID,
            siren,
            interior,
        }
    }
    /// Adjusts vehicle model, spawn location, colours, respawn delay and interior.
    pub fn set_spawn_info(&self, data: VehicleSpawnData) -> bool {
        functions::Vehicle_SetSpawnInfo(
            self,
            data.modelID,
            data.position.x,
            data.position.y,
            data.position.z,
            data.rotation,
            data.colour1,
            data.colour2,
            data.respawnDelay,
            data.interior,
        )
    }
    /// Gets the model count of a vehicle model.
    pub fn get_model_count(modelid: i32) -> i32 {
        functions::Vehicle_GetModelCount(modelid)
    }
    /// Get the number of used vehicle models on the server.
    pub fn get_models_used() -> i32 {
        functions::Vehicle_GetModelsUsed()
    }
    /// Gets the vehicle's paintjob id.
    pub fn get_paintjob(&self) -> i32 {
        functions::Vehicle_GetPaintjob(self)
    }
    pub fn get_color(&self) -> (i32, i32) {
        let mut colour1 = -1;
        let mut colour2 = -1;
        functions::Vehicle_GetColor(self, &mut colour1, &mut colour2);
        (colour1, colour2)
    }
    /// Get the interior id of a vehicle.
    pub fn get_interior(&self) -> i32 {
        functions::Vehicle_GetInterior(self)
    }
    /// Get the number plate of a vehicle.
    pub fn get_number_plate(&self) -> String {
        let mut number_plate = String::new();
        functions::Vehicle_GetNumberPlate(self, &mut number_plate);
        number_plate
    }
    /// Set the respawn delay of a vehicle.
    pub fn set_respawn_delay(&self, respawn_delay: i32) -> bool {
        functions::Vehicle_SetRespawnDelay(self, respawn_delay)
    }
    /// Get the respawn delay of a vehicle.
    pub fn get_respawn_delay(&self) -> i32 {
        functions::Vehicle_GetRespawnDelay(self)
    }
    /// Get the ID of the cab attached to a vehicle.
    pub fn get_cab(&self) -> Option<Vehicle> {
        functions::Vehicle_GetCab(self)
    }
    /// Get the occupied tick of a vehicle.
    pub fn get_occupied_tick(&self) -> i32 {
        functions::Vehicle_GetOccupiedTick(self)
    }
    /// Get the respawn tick of a vehicle.
    pub fn get_respawn_tick(&self) -> i32 {
        functions::Vehicle_GetRespawnTick(self)
    }
    /// Check if a vehicle is occupied.
    pub fn has_been_occupied(&self) -> bool {
        functions::Vehicle_HasBeenOccupied(self)
    }
    /// Check if a vehicle is occupied.
    pub fn is_occupied(&self) -> bool {
        functions::Vehicle_IsOccupied(self)
    }
    /// Check if a vehicle is dead.
    pub fn is_dead(&self) -> bool {
        functions::Vehicle_IsDead(self)
    }
    /// Turn the siren for a vehicle on or off.
    pub fn toggle_siren_enabled(&self, status: bool) -> bool {
        functions::Vehicle_ToggleSirenEnabled(self, status)
    }
    /// Checks if a vehicle siren is on or off.
    pub fn is_siren_enabled(&self) -> bool {
        functions::Vehicle_IsSirenEnabled(self)
    }
    /// Get the last driver of a vehicle.
    pub fn get_last_driver(&self) -> Option<Player> {
        functions::Vehicle_GetLastDriver(self)
    }
    /// Get the playerid of the person driving the vehicle.
    pub fn get_driver(&self) -> Option<Player> {
        functions::Vehicle_GetDriver(self)
    }

    /// Gets the siren state of the vehicle.
    pub fn get_siren_state(&self) -> i32 {
        functions::Vehicle_GetParamsSirenState(self)
    }
    /// Gets the hydra reactor angle of the vehicle.
    pub fn get_hydra_reactor_angle(&self) -> u32 {
        functions::Vehicle_GetHydraReactorAngle(self)
    }
    /// Gets the speed of the train.
    pub fn get_train_speed(&self) -> f32 {
        functions::Vehicle_GetTrainSpeed(self)
    }
    /// Gets the actual rotation matrix of the vehicle.
    pub fn get_matrix(&self) -> VehicleMatrix {
        let (mut right, mut up, mut at): (Vector3, Vector3, Vector3) = Default::default();

        functions::Vehicle_GetMatrix(
            self,
            &mut right.x,
            &mut right.y,
            &mut right.z,
            &mut up.x,
            &mut up.y,
            &mut up.z,
            &mut at.x,
            &mut at.y,
            &mut at.z,
        );
        VehicleMatrix { right, up, at }
    }
    pub fn get_occupant(&self, seat: i32) -> Option<Player> {
        functions::Vehicle_GetOccupant(self, seat)
    }
    pub fn get_max_passengers(model: i32) -> i32 {
        functions::Vehicle_GetMaxPassengerSeats(model)
    }
    pub fn count_occupants(&self) -> i32 {
        functions::Vehicle_CountOccupants(self)
    }
    pub fn get_from_id(id: i32) -> Option<Vehicle> {
        functions::Vehicle_FromID(id)
    }
    pub fn get_id(&self) -> i32 {
        functions::Vehicle_GetID(self)
    }
}

#[repr(C)]
pub struct VehicleParams {
    pub engine: i8,
    pub lights: i8,
    pub alarm: i8,
    pub doors: i8,
    pub bonnet: i8,
    pub boot: i8,
    pub objective: i8,
    pub siren: i8,
    pub doorDriver: i8,
    pub doorPassenger: i8,
    pub doorBackLeft: i8,
    pub doorBackRight: i8,
    pub windowDriver: i8,
    pub windowPassenger: i8,
    pub windowBackLeft: i8,
    pub windowBackRight: i8,
}

impl Default for VehicleParams {
    fn default() -> Self {
        Self {
            engine: -1,
            lights: -1,
            alarm: -1,
            doors: -1,
            bonnet: -1,
            boot: -1,
            objective: -1,
            siren: -1,
            doorDriver: -1,
            doorPassenger: -1,
            doorBackLeft: -1,
            doorBackRight: -1,
            windowDriver: -1,
            windowPassenger: -1,
            windowBackLeft: -1,
            windowBackRight: -1,
        }
    }
}

#[repr(C)]
#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub struct UnoccupiedVehicleUpdate {
    pub seat: i32,
    pub position: Vector3,
    pub velocity: Vector3,
}

#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub struct VehicleDamageStatusData {
    pub panels: i32,
    pub doors: i32,
    pub lights: i32,
    pub tires: i32,
}

#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub struct VehicleSpawnData {
    pub respawnDelay: i32,
    pub modelID: i32,
    pub position: Vector3,
    pub rotation: f32,
    pub colour1: i32,
    pub colour2: i32,
    pub siren: bool,
    pub interior: i32,
}

#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub struct VehicleMatrix {
    pub right: Vector3,
    pub up: Vector3,
    pub at: Vector3,
}
