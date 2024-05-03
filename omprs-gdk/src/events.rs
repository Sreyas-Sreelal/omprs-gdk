use crate::{
    actors::Actor,
    gangzones::GangZone,
    models::ModelDownloadType,
    objects::{Object, ObjectAttachmentSlotData, ObjectEditResponse, PlayerObject},
    pickups::Pickup,
    players::{BodyPart, Player, PlayerBulletData, PlayerClickSource, PlayerState},
    scripting::dialogs::DialogResponse,
    textdraws::{PlayerTextDraw, TextDraw},
    types::{network::PeerDisconnectReason, vector::Vector3},
    vehicles::{UnoccupiedVehicleUpdate, Vehicle},
};

#[allow(unused_variables)]
pub trait Events {
    /// Player spawn event handlers
    fn on_player_request_spawn(&mut self, player: Player) -> bool {
        true
    }
    fn on_player_spawn(&mut self, player: Player) {}

    /// Player connection event handlers
    fn on_incoming_connection(&mut self, player: Player, ip_address: String, port: u16) {}
    fn on_player_connect(&mut self, player: Player) {}
    fn on_player_disconnect(&mut self, player: Player, reason: PeerDisconnectReason) {}
    fn on_player_client_init(&mut self, player: Player) {}

    /// Player streaming event handlers
    fn on_player_stream_in(&mut self, player: Player, for_player: Player) {}
    fn on_player_stream_out(&mut self, player: Player, for_player: Player) {}

    /// Player text and commands event handlers
    fn on_player_text(&mut self, player: Player, message: String) -> bool {
        true
    }
    fn on_player_command_text(&mut self, player: Player, message: String) -> bool {
        false
    }

    /// Player shooting event handlers
    fn on_player_shot_missed(&mut self, player: Player, bullet_data: PlayerBulletData) -> bool {
        true
    }
    fn on_player_shot_player(
        &mut self,
        player: Player,
        target: Player,
        bullet_data: PlayerBulletData,
    ) -> bool {
        true
    }
    fn on_player_shot_vehicle(
        &mut self,
        player: Player,
        target: Vehicle,
        bullet_data: PlayerBulletData,
    ) -> bool {
        true
    }
    fn on_player_shot_object(
        &mut self,
        player: Player,
        target: Object,
        bullet_data: PlayerBulletData,
    ) -> bool {
        true
    }
    fn on_player_shot_player_object(
        &mut self,
        player: Player,
        target: PlayerObject,
        bullet_data: PlayerBulletData,
    ) -> bool {
        true
    }

    /// Player data change event handlers
    fn on_player_score_change(&mut self, player: Player, score: isize) {}
    fn on_player_name_change(&mut self, player: Player, old_name: String) {}
    fn on_player_interior_change(
        &mut self,
        player: Player,
        new_interior: usize,
        old_interior: usize,
    ) {
    }
    fn on_player_state_change(
        &mut self,
        player: Player,
        new_state: PlayerState,
        old_state: PlayerState,
    ) {
    }
    fn on_player_key_state_change(&mut self, player: Player, new_keys: u32, old_keys: u32) {}

    /// APlayer death and damage event handlers
    fn on_player_death(&mut self, player: Player, killer: Option<Player>, reason: isize) {}
    fn on_player_take_damage(
        &mut self,
        player: Player,
        from: Option<Player>,
        amount: f32,
        weapon: usize,
        part: BodyPart,
    ) {
    }
    fn on_player_give_damage(
        &mut self,
        player: Player,
        to: Player,
        amount: f32,
        weapon: usize,
        part: BodyPart,
    ) {
    }

    /// Player clicking event handlers
    fn on_player_click_map(&mut self, player: Player, pos: Vector3) {}
    fn on_player_click_player(
        &mut self,
        player: Player,
        clicked: Player,
        source: PlayerClickSource,
    ) {
    }

    /// Player client check response event handler
    fn on_client_check_response(
        &mut self,
        player: Player,
        action_type: isize,
        address: isize,
        results: isize,
    ) {
    }

    /// Player update event handler
    fn on_player_update(&mut self, player: Player, now: isize) -> bool {
        true
    }

    // Player Model event handlers
    fn on_player_finished_downloading(&mut self, player: Player) {}

    fn on_player_request_download(
        &mut self,
        player: Player,
        model_type: ModelDownloadType,
        checksum: u32,
    ) -> bool {
        true
    }

    /// Actor Event handlers
    fn on_player_give_damage_actor(
        &mut self,
        player: Player,
        actor: Actor,
        amount: f32,
        weapon: usize,
        part: BodyPart,
    ) {
    }

    fn on_actor_stream_in(&mut self, actor: Actor, player: Player) {}

    fn on_actor_stream_out(&mut self, actor: Actor, player: Player) {}

    fn on_player_enter_checkpoint(&mut self, player: Player) {}

    fn on_player_leave_checkpoint(&mut self, player: Player) {}

    fn on_player_enter_race_checkpoint(&mut self, player: Player) {}

    fn on_player_leave_race_checkpoint(&mut self, player: Player) {}

    fn on_player_request_class(&mut self, player: Player, class_id: usize) -> bool {
        true
    }

    // Dialog callbacks
    fn on_dialog_response(
        &mut self,
        player: Player,
        dialog_id: i16,
        response: DialogResponse,
        list_item: isize,
        input_text: String,
    ) {
    }

    // GangZone callbacks
    fn on_player_enter_gang_zone(&mut self, player: Player, zone: GangZone) {}
    fn on_player_leave_gang_zone(&mut self, player: Player, zone: GangZone) {}
    fn on_player_click_gang_zone(&mut self, player: Player, zone: GangZone) {}

    // Menu callbacks
    fn on_player_selected_menu_row(&mut self, player: Player, row: isize) {}
    fn on_player_exited_menu(&mut self, player: Player) {}

    // Object callbacks
    fn on_object_moved(&mut self, object: Object) {}
    fn on_player_object_moved(&mut self, player: Player, object: PlayerObject) {}
    fn on_player_edit_object(
        &mut self,
        player: Player,
        object: Object,
        response: ObjectEditResponse,
        offset: Vector3,
        rotation: Vector3,
    ) {
    }
    fn on_player_object_edited(
        &mut self,
        player: Player,
        object: PlayerObject,
        response: ObjectEditResponse,
        offset: Vector3,
        rotation: Vector3,
    ) {
    }
    fn on_player_edit_attached_object(
        &mut self,
        player: Player,
        index: isize,
        saved: bool,
        data: ObjectAttachmentSlotData,
    ) {
    }
    fn on_player_select_object(
        &mut self,
        player: Player,
        object: Object,
        model: isize,
        position: Vector3,
    ) {
    }
    fn on_player_object_selected(
        &mut self,
        player: Player,
        object: PlayerObject,
        model: isize,
        position: Vector3,
    ) {
    }

    // Pickup callbacks
    fn on_player_pick_up_pickup(&mut self, player: Player, pickup: Pickup) {}

    // TextDraw callbacks
    fn on_player_cancel_text_draw_selection(&mut self, player: Player) -> bool {
        true
    }
    fn on_player_cancel_player_text_draw_selection(&mut self, player: Player) -> bool {
        true
    }
    fn on_player_click_text_draw(&mut self, player: Player, textdraw: TextDraw) {}
    fn on_player_click_player_text_draw(&mut self, player: Player, textdraw: PlayerTextDraw) {}

    // Vehicle callbacks
    fn on_vehicle_stream_in(&mut self, vehicle: Vehicle, player: Player) {}
    fn on_vehicle_stream_out(&mut self, vehicle: Vehicle, player: Player) {}
    fn on_vehicle_death(&mut self, vehicle: Vehicle, player: Player) {}
    fn on_player_enter_vehicle(&mut self, player: Player, vehicle: Vehicle, passenger: bool) {}
    fn on_player_exit_vehicle(&mut self, player: Player, vehicle: Vehicle) {}
    fn on_vehicle_damage_status_update(&mut self, vehicle: Vehicle, player: Player) {}
    fn on_vehicle_paint_job(&mut self, player: Player, vehicle: Vehicle, paintjob: isize) -> bool {
        true
    }
    fn on_vehicle_mod(&mut self, player: Player, vehicle: Vehicle, component: isize) -> bool {
        true
    }
    fn on_vehicle_respray(
        &mut self,
        player: Player,
        vehicle: Vehicle,
        colour1: isize,
        colour2: isize,
    ) -> bool {
        true
    }
    fn on_enter_exit_mod_shop(&mut self, player: Player, enterexit: bool, interior_id: isize) {}
    fn on_vehicle_spawn(&mut self, vehicle: Vehicle) {}
    fn on_unoccupied_vehicle_update(
        &mut self,
        vehicle: Vehicle,
        player: Player,
        updateData: UnoccupiedVehicleUpdate,
    ) -> bool {
        true
    }
    fn on_trailer_update(&mut self, player: Player, vehicle: Vehicle) -> bool {
        true
    }
    fn on_vehicle_siren_state_change(
        &mut self,
        player: Player,
        vehicle: Vehicle,
        sirenstate: u8,
    ) -> bool {
        true
    }

    // Core callbacks
    fn on_rcon_command(&mut self, cmd: String) -> bool {
        true
    }
    fn on_rcon_login_attempt(
        &mut self,
        player: Option<Player>,
        ip: String,
        password: String,
        success: bool,
    ) {
    }
}
