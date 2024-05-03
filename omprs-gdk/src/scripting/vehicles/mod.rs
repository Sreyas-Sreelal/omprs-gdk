pub mod events;
pub mod functions;

use std::ffi::c_void;

use crate::{
    players::Player,
    vector::{Vector3, Vector4},
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
    pub fn get_seats(modelid: isize) -> isize {
        functions::GetVehicleSeats(modelid)
    }
    pub fn destroy(&self) {
        functions::DestroyVehicle(self)
    }
    pub fn is_streamed_in(&self, player: &Player) -> bool {
        functions::IsVehicleStreamedIn(self, player)
    }
    pub fn get_pos(&self) -> Vector3 {
        let mut pos = Vector3::default();
        functions::GetVehiclePos(self, &mut pos);
        pos
    }
    pub fn set_pos(&self, pos: Vector3) {
        functions::SetVehiclePos(self, pos)
    }
    pub fn get_z_angle(&self) -> f32 {
        functions::GetVehicleZAngle(self)
    }
    pub fn get_rotation_quat(&self) -> Vector4 {
        let mut rotation = Vector4::default();
        functions::GetVehicleRotationQuat(self, &mut rotation);
        rotation
    }
    pub fn get_distance_from_point(&self, pos: Vector3) -> f32 {
        functions::GetVehicleDistanceFromPoint(self, pos)
    }
    pub fn set_z_angle(&self, angle: f32) {
        functions::SetVehicleZAngle(self, angle)
    }
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
    pub fn set_to_respawn(&self) {
        functions::SetVehicleToRespawn(self)
    }
    pub fn link_to_interior(&self, interiorid: isize) {
        functions::LinkVehicleToInterior(self, interiorid)
    }
    pub fn add_component(&self, componentid: isize) {
        functions::AddVehicleComponent(self, componentid)
    }
    pub fn remove_component(&self, componentid: isize) {
        functions::RemoveVehicleComponent(self, componentid)
    }
    pub fn change_color(&self, colour1: isize, colour2: isize) {
        functions::ChangeVehicleColor(self, colour1, colour2)
    }
    pub fn change_paintjob(&self, paintjobid: isize) {
        functions::ChangeVehiclePaintjob(self, paintjobid)
    }
    pub fn set_health(&self, health: f32) {
        functions::SetVehicleHealth(self, health)
    }
    pub fn get_health(&self) -> f32 {
        functions::GetVehicleHealth(self)
    }
    pub fn attach_trailer(&self, trailer: &Vehicle) {
        functions::AttachTrailerToVehicle(self, trailer)
    }
    pub fn detach_trailer(&self) {
        functions::DetachTrailerFromVehicle(self)
    }
    pub fn is_trailer_attached(&self) -> bool {
        functions::IsTrailerAttachedToVehicle(self)
    }
    pub fn get_trailer(&self) -> Option<Vehicle> {
        functions::GetVehicleTrailer(self)
    }
    pub fn set_number_plate(&self, numberPlate: &str) {
        functions::SetVehicleNumberPlate(self, numberPlate)
    }
    pub fn get_model(&self) -> isize {
        functions::GetVehicleModel(self)
    }
    pub fn get_component_in_slot(&self, slot: isize) -> isize {
        functions::GetVehicleComponentInSlot(self, slot)
    }
    pub fn get_component_type(componentid: isize) -> isize {
        functions::GetVehicleComponentType(componentid)
    }
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
    pub fn repair(&self) {
        functions::RepairVehicle(self)
    }
    pub fn get_velocity(&self) -> Vector3 {
        let mut velocity = Vector3::default();
        functions::GetVehicleVelocity(self, &mut velocity);
        velocity
    }
    pub fn set_velocity(&self, velocity: Vector3) {
        functions::SetVehicleVelocity(self, velocity)
    }
    pub fn set_angular_velocity(&self, velocity: Vector3) {
        functions::SetVehicleAngularVelocity(self, velocity)
    }
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
    pub fn update_damage_status(&self, panels: isize, doors: isize, lights: isize, tires: isize) {
        functions::UpdateVehicleDamageStatus(self, panels, doors, lights, tires)
    }
    pub fn get_model_info(model: isize, infotype: isize) -> Vector3 {
        let mut pos = Vector3::default();
        functions::GetVehicleModelInfo(model, infotype, &mut pos);
        pos
    }
    pub fn set_virtual_world(&self, virtualWorld: isize) {
        functions::SetVehicleVirtualWorld(self, virtualWorld)
    }
    pub fn get_virtual_world(&self) -> isize {
        functions::GetVehicleVirtualWorld(self)
    }
    pub fn get_landing_gear_state(&self) -> isize {
        functions::GetVehicleLandingGearState(self)
    }
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
    pub fn enable_friendly_fire(set: bool) {
        functions::EnableVehicleFriendlyFire(set)
    }
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
    pub fn get_model_count(modelid: isize) -> isize {
        functions::GetVehicleModelCount(modelid)
    }
    pub fn get_models_used() -> isize {
        functions::GetVehicleModelsUsed()
    }
    pub fn get_paintjob(&self) -> isize {
        functions::GetVehiclePaintjob(self)
    }
    pub fn get_color(&self) -> (isize, isize) {
        let mut colour1 = -1;
        let mut colour2 = -1;
        functions::GetVehicleColor(self, &mut colour1, &mut colour2);
        (colour1, colour2)
    }
    pub fn get_interior(&self) -> isize {
        functions::GetVehicleInterior(self)
    }
    pub fn get_number_plate(&self) -> String {
        let mut number_plate = String::new();
        functions::GetVehicleNumberPlate(self, &mut number_plate);
        number_plate
    }
    pub fn set_respawn_delay(&self, respawn_delay: isize) {
        functions::SetVehicleRespawnDelay(self, respawn_delay)
    }
    pub fn get_respawn_delay(&self) -> isize {
        functions::GetVehicleRespawnDelay(self)
    }
    pub fn get_cab(&self) -> Option<Vehicle> {
        functions::GetVehicleCab(self)
    }
    pub fn get_occupied_tick(&self) -> isize {
        functions::GetVehicleOccupiedTick(self)
    }
    pub fn get_respawn_tick(&self) -> isize {
        functions::GetVehicleRespawnTick(self)
    }
    pub fn has_been_occupied(&self) -> bool {
        functions::HasVehicleBeenOccupied(self)
    }
    pub fn is_occupied(&self) -> bool {
        functions::IsVehicleOccupied(self)
    }
    pub fn is_dead(&self) -> bool {
        functions::IsVehicleDead(self)
    }
    pub fn toggle_siren_enabled(&self, status: bool) {
        functions::ToggleVehicleSirenEnabled(self, status)
    }
    pub fn is_siren_enabled(&self) -> bool {
        functions::IsVehicleSirenEnabled(self)
    }
    pub fn get_last_driver(&self) -> isize {
        functions::GetVehicleLastDriver(self)
    }
    pub fn get_driver(&self) -> Option<Player> {
        functions::GetVehicleDriver(self)
    }

    pub fn get_siren_state(&self) -> isize {
        functions::GetVehicleSirenState(self)
    }
    pub fn get_hydra_reactor_angle(&self) -> isize {
        functions::GetVehicleHydraReactorAngle(self)
    }
    pub fn get_train_speed(&self) -> f32 {
        functions::GetVehicleTrainSpeed(self)
    }
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
pub struct UnoccupiedVehicleUpdate {
    seat: u8,
    position: Vector3,
    velocity: Vector3,
}

#[derive(Default)]
pub struct VehicleDamageStatusData {
    pub panels: isize,
    pub doors: isize,
    pub lights: isize,
    pub tires: isize,
}

#[derive(Default)]
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

#[derive(Default)]
pub struct VehicleMatrix {
    pub right: Vector3,
    pub up: Vector3,
    pub at: Vector3,
}
