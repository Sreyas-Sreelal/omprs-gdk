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
        modelid: isize,
        pos: Vector3,
        rotation: f32,
        colour1: isize,
        colour2: isize,
        respawnDelay: isize,
        addSiren: bool,
    ) -> Option<Vehicle> {
        functions::CreateVehicle(
            modelid,
            pos,
            rotation,
            colour1,
            colour2,
            respawnDelay,
            addSiren,
        )
    }
    /// Gets the number of seats in the vehicle.
    pub fn get_seats(modelid: isize) -> isize {
        functions::GetVehicleSeats(modelid)
    }
    /// Destroy a vehicle.
    pub fn destroy(&self) {
        functions::DestroyVehicle(self)
    }
    /// Checks if a vehicle is streamed in for a player.
    pub fn is_streamed_in(&self, player: &Player) -> bool {
        functions::IsVehicleStreamedIn(self, player)
    }
    /// Gets the position of a vehicle.
    pub fn get_pos(&self) -> Vector3 {
        let mut pos = Vector3::default();
        functions::GetVehiclePos(self, &mut pos);
        pos
    }
    /// Set a vehicle's position.
    pub fn set_pos(&self, pos: Vector3) {
        functions::SetVehiclePos(self, pos)
    }
    /// Get the rotation of a vehicle on the Z axis (yaw).
    pub fn get_z_angle(&self) -> f32 {
        functions::GetVehicleZAngle(self)
    }
    /// Returns a vehicle's rotation on all axes as a quaternion.
    pub fn get_rotation_quat(&self) -> Vector4 {
        let mut rotation = Vector4::default();
        functions::GetVehicleRotationQuat(self, &mut rotation);
        rotation
    }
    /// This function can be used to calculate the distance (as a float) between a vehicle and another map coordinate.
    pub fn get_distance_from_point(&self, pos: Vector3) -> f32 {
        functions::GetVehicleDistanceFromPoint(self, pos)
    }
    /// Set the Z rotation (yaw) of a vehicle.
    pub fn set_z_angle(&self, angle: f32) {
        functions::SetVehicleZAngle(self, angle)
    }
    /// Set the parameters of a vehicle for a player.
    pub fn set_params_for_player(&self, player: &Player, params: VehicleParams) {
        functions::SetVehicleParamsForPlayer(self, player, params)
    }
    pub fn set_manual_engine_and_lights(set: bool) {
        functions::SetManualVehicleEngineAndLights(set)
    }
    pub fn set_params(&self, params: VehicleParams) {
        functions::SetVehicleParams(self, params)
    }
    pub fn get_params(&self) -> VehicleParams {
        let mut params = VehicleParams::default();
        functions::GetVehicleParams(self, &mut params);
        params
    }
    /// Sets a vehicle back to the position at where it was created.
    pub fn set_to_respawn(&self) {
        functions::SetVehicleToRespawn(self)
    }
    /// Links a vehicle to an interior.
    pub fn link_to_interior(&self, interiorid: isize) {
        functions::LinkVehicleToInterior(self, interiorid)
    }
    /// Adds a 'component' (often referred to as a 'mod' (modification)) to a vehicle.
    pub fn add_component(&self, componentid: isize) {
        functions::AddVehicleComponent(self, componentid)
    }
    /// Remove a component from a vehicle.
    pub fn remove_component(&self, componentid: isize) {
        functions::RemoveVehicleComponent(self, componentid)
    }
    /// Change a vehicle's primary and secondary colors.
    pub fn change_color(&self, colour1: isize, colour2: isize) {
        functions::ChangeVehicleColor(self, colour1, colour2)
    }
    /// Change a vehicle's paintjob.
    pub fn change_paintjob(&self, paintjobid: isize) {
        functions::ChangeVehiclePaintjob(self, paintjobid)
    }
    /// Set a vehicle's health.
    pub fn set_health(&self, health: f32) {
        functions::SetVehicleHealth(self, health)
    }
    /// Get the health of a vehicle.
    pub fn get_health(&self) -> f32 {
        functions::GetVehicleHealth(self)
    }
    /// Attach a vehicle to another vehicle as a trailer.
    pub fn attach_trailer(&self, trailer: &Vehicle) {
        functions::AttachTrailerToVehicle(self, trailer)
    }
    /// Detach the connection between a vehicle and its trailer, if any.
    pub fn detach_trailer(&self) {
        functions::DetachTrailerFromVehicle(self)
    }
    /// Checks if a vehicle has a trailer attached to it.
    pub fn is_trailer_attached(&self) -> bool {
        functions::IsTrailerAttachedToVehicle(self)
    }
    /// Get the ID of the trailer attached to a vehicle.
    pub fn get_trailer(&self) -> Option<Vehicle> {
        functions::GetVehicleTrailer(self)
    }
    /// Set a vehicle numberplate.
    pub fn set_number_plate(&self, numberPlate: &str) {
        functions::SetVehicleNumberPlate(self, numberPlate)
    }
    /// Gets the model ID of a vehicle.
    pub fn get_model(&self) -> isize {
        functions::GetVehicleModel(self)
    }
    /// Retrieves the installed component ID (modshop mod(ification)) on a vehicle in a specific slot.
    pub fn get_component_in_slot(&self, slot: isize) -> isize {
        functions::GetVehicleComponentInSlot(self, slot)
    }
    /// Find out what type of component a certain ID is.
    pub fn get_component_type(componentid: isize) -> isize {
        functions::GetVehicleComponentType(componentid)
    }
    /// Is the component legal on the vehicle?
    pub fn can_have_component(modelid: isize, componentid: isize) -> bool {
        functions::VehicleCanHaveComponent(modelid, componentid)
    }
    pub fn get_random_car_col_pair(modelid: isize) -> (isize, isize, isize, isize) {
        let (mut colour1, mut colour2, mut colour3, mut colour4) = Default::default();
        functions::GetRandomCarColPair(
            modelid,
            &mut colour1,
            &mut colour2,
            &mut colour3,
            &mut colour4,
        );
        (colour1, colour2, colour3, colour4)
    }
    pub fn car_col_index_to_colour(colourIndex: isize, alpha: isize) -> isize {
        functions::CarColIndexToColour(colourIndex, alpha)
    }
    /// Fully repairs a vehicle, including visual damage (bumps, dents, scratches, popped tires etc.
    pub fn repair(&self) {
        functions::RepairVehicle(self)
    }
    /// Get the velocity of a vehicle on the X, Y and Z axes.
    pub fn get_velocity(&self) -> Vector3 {
        let mut velocity = Vector3::default();
        functions::GetVehicleVelocity(self, &mut velocity);
        velocity
    }
    /// Sets the X, Y and Z velocity of a vehicle.
    pub fn set_velocity(&self, velocity: Vector3) {
        functions::SetVehicleVelocity(self, velocity)
    }
    /// Sets the angular X, Y and Z velocity of a vehicle.
    pub fn set_angular_velocity(&self, velocity: Vector3) {
        functions::SetVehicleAngularVelocity(self, velocity)
    }
    /// Retrieve the damage statuses of a vehicle.
    pub fn get_damage_status(&self) -> VehicleDamageStatusData {
        let (mut panels, mut doors, mut lights, mut tires) = Default::default();

        functions::GetVehicleDamageStatus(self, &mut panels, &mut doors, &mut lights, &mut tires);

        VehicleDamageStatusData {
            panels,
            doors,
            lights,
            tires,
        }
    }
    /// Sets the various visual damage statuses of a vehicle, such as popped tires, broken lights and damaged panels.
    pub fn update_damage_status(&self, panels: isize, doors: isize, lights: isize, tires: isize) {
        functions::UpdateVehicleDamageStatus(self, panels, doors, lights, tires)
    }
    /// Retrieve information about a specific vehicle model such as the size or position of seats.
    pub fn get_model_info(model: isize, infotype: isize) -> Vector3 {
        let mut pos = Vector3::default();
        functions::GetVehicleModelInfo(model, infotype, &mut pos);
        pos
    }
    /// Sets the 'virtual world' of a vehicle.
    pub fn set_virtual_world(&self, virtualWorld: isize) {
        functions::SetVehicleVirtualWorld(self, virtualWorld)
    }
    /// Get the virtual world of a vehicle.
    pub fn get_virtual_world(&self) -> isize {
        functions::GetVehicleVirtualWorld(self)
    }
    /// Gets the current vehicle landing gear state from the latest driver.
    pub fn get_landing_gear_state(&self) -> isize {
        functions::GetVehicleLandingGearState(self)
    }
    /// Adds a 'static' vehicle (models are pre-loaded for players) to the gamemode.
    pub fn create_static(
        modelid: isize,
        spawn: Vector3,
        angle: f32,
        colour1: isize,
        colour2: isize,
        respawnDelay: isize,
        addSiren: bool,
    ) -> Option<Vehicle> {
        functions::AddStaticVehicle(
            modelid,
            spawn,
            angle,
            colour1,
            colour2,
            respawnDelay,
            addSiren,
        )
    }
    /// Enable friendly fire for team vehicles.
    pub fn enable_friendly_fire(set: bool) {
        functions::EnableVehicleFriendlyFire(set)
    }
    /// Gets the vehicle spawn location and colours.
    pub fn get_spawn_info(&self) -> VehicleSpawnData {
        let (
            mut position,
            mut rotation,
            mut colour1,
            mut colour2,
            mut respawnDelay,
            mut modelID,
            mut siren,
            mut interior,
        ): (Vector3, f32, isize, isize, isize, isize, bool, isize) = Default::default();

        functions::GetVehicleSpawnInfo(
            self,
            &mut position,
            &mut rotation,
            &mut colour1,
            &mut colour2,
            &mut respawnDelay,
            &mut modelID,
            &mut siren,
            &mut interior,
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
    pub fn set_spawn_info(&self, data: VehicleSpawnData) {
        functions::SetVehicleSpawnInfo(
            self,
            data.modelID,
            data.position,
            data.rotation,
            data.colour1,
            data.colour2,
            data.respawnDelay,
            data.interior,
        )
    }
    /// Gets the model count of a vehicle model.
    pub fn get_model_count(modelid: isize) -> isize {
        functions::GetVehicleModelCount(modelid)
    }
    /// Get the number of used vehicle models on the server.
    pub fn get_models_used() -> isize {
        functions::GetVehicleModelsUsed()
    }
    /// Gets the vehicle's paintjob id.
    pub fn get_paintjob(&self) -> isize {
        functions::GetVehiclePaintjob(self)
    }
    pub fn get_color(&self) -> (isize, isize) {
        let mut colour1 = -1;
        let mut colour2 = -1;
        functions::GetVehicleColor(self, &mut colour1, &mut colour2);
        (colour1, colour2)
    }
    /// Get the interior id of a vehicle.
    pub fn get_interior(&self) -> isize {
        functions::GetVehicleInterior(self)
    }
    /// Get the number plate of a vehicle.
    pub fn get_number_plate(&self) -> String {
        let mut number_plate = String::new();
        functions::GetVehicleNumberPlate(self, &mut number_plate);
        number_plate
    }
    /// Set the respawn delay of a vehicle.
    pub fn set_respawn_delay(&self, respawn_delay: isize) {
        functions::SetVehicleRespawnDelay(self, respawn_delay)
    }
    /// Get the respawn delay of a vehicle.
    pub fn get_respawn_delay(&self) -> isize {
        functions::GetVehicleRespawnDelay(self)
    }
    /// Get the ID of the cab attached to a vehicle.
    pub fn get_cab(&self) -> Option<Vehicle> {
        functions::GetVehicleCab(self)
    }
    /// Get the occupied tick of a vehicle.
    pub fn get_occupied_tick(&self) -> isize {
        functions::GetVehicleOccupiedTick(self)
    }
    /// Get the respawn tick of a vehicle.
    pub fn get_respawn_tick(&self) -> isize {
        functions::GetVehicleRespawnTick(self)
    }
    /// Check if a vehicle is occupied.
    pub fn has_been_occupied(&self) -> bool {
        functions::HasVehicleBeenOccupied(self)
    }
    /// Check if a vehicle is occupied.
    pub fn is_occupied(&self) -> bool {
        functions::IsVehicleOccupied(self)
    }
    /// Check if a vehicle is dead.
    pub fn is_dead(&self) -> bool {
        functions::IsVehicleDead(self)
    }
    /// Turn the siren for a vehicle on or off.
    pub fn toggle_siren_enabled(&self, status: bool) {
        functions::ToggleVehicleSirenEnabled(self, status)
    }
    /// Checks if a vehicle siren is on or off.
    pub fn is_siren_enabled(&self) -> bool {
        functions::IsVehicleSirenEnabled(self)
    }
    /// Get the last driver of a vehicle.
    pub fn get_last_driver(&self) -> isize {
        functions::GetVehicleLastDriver(self)
    }
    /// Get the playerid of the person driving the vehicle.
    pub fn get_driver(&self) -> Option<Player> {
        functions::GetVehicleDriver(self)
    }

    /// Gets the siren state of the vehicle.
    pub fn get_siren_state(&self) -> isize {
        functions::GetVehicleSirenState(self)
    }
    /// Gets the hydra reactor angle of the vehicle.
    pub fn get_hydra_reactor_angle(&self) -> isize {
        functions::GetVehicleHydraReactorAngle(self)
    }
    /// Gets the speed of the train.
    pub fn get_train_speed(&self) -> f32 {
        functions::GetVehicleTrainSpeed(self)
    }
    /// Gets the actual rotation matrix of the vehicle.
    pub fn get_matrix(&self) -> VehicleMatrix {
        let (mut right, mut up, mut at) = Default::default();

        functions::GetVehicleMatrix(self, &mut right, &mut up, &mut at);
        VehicleMatrix { right, up, at }
    }
    pub fn get_occupant(&self, seat: isize) -> Option<Player> {
        functions::GetVehicleOccupant(self, seat)
    }
    pub fn get_max_passengers(model: isize) -> isize {
        functions::GetVehicleMaxPassengers(model)
    }
    pub fn count_occupants(&self) -> isize {
        functions::CountVehicleOccupants(self)
    }
    pub fn get_from_id(id: isize) -> Option<Vehicle> {
        functions::GetVehicleFromID(id)
    }
    pub fn get_id(&self) -> isize {
        functions::GetVehicleID(self)
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
    pub seat: u8,
    pub position: Vector3,
    pub velocity: Vector3,
}

#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub struct VehicleDamageStatusData {
    pub panels: isize,
    pub doors: isize,
    pub lights: isize,
    pub tires: isize,
}

#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub struct VehicleSpawnData {
    pub respawnDelay: isize,
    pub modelID: isize,
    pub position: Vector3,
    pub rotation: f32,
    pub colour1: isize,
    pub colour2: isize,
    pub siren: bool,
    pub interior: isize,
}

#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub struct VehicleMatrix {
    pub right: Vector3,
    pub up: Vector3,
    pub at: Vector3,
}
