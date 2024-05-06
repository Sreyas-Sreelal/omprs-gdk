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
    /// This callback is called when a player attempts to spawn via class selection either by pressing SHIFT or clicking the 'Spawn' button.
    fn on_player_request_spawn(&mut self, player: Player) -> bool {
        true
    }

    /// This callback is called when a player spawns
    fn on_player_spawn(&mut self, player: Player) {}

    /// This callback is called when an IP address attempts a connection to the server.     
    fn on_incoming_connection(&mut self, player: Player, ip_address: String, port: u16) {}

    /// This callback is called when a player connects to the server.
    fn on_player_connect(&mut self, player: Player) {}

    /// This callback is called when a player disconnects from the server.
    fn on_player_disconnect(&mut self, player: Player, reason: PeerDisconnectReason) {}

    /// This callback is called when a player is streamed by some other player's client.
    fn on_player_stream_in(&mut self, player: Player, for_player: Player) {}

    /// This callback is called when a player is streamed out from some other player's client.
    fn on_player_stream_out(&mut self, player: Player, for_player: Player) {}

    /// This callback is called when a player sends a chat message.
    fn on_player_text(&mut self, player: Player, message: String) -> bool {
        true
    }

    /// This callback is called when a player enters a command into the client chat window. Commands are anything that start with a forward slash, e.g. /help.
    fn on_player_command_text(&mut self, player: Player, message: String) -> bool {
        false
    }

    fn on_player_shot_missed(&mut self, player: Player, bullet_data: PlayerBulletData) -> bool {
        true
    }

    /// This callback is called when a player shoots another player
    fn on_player_shot_player(
        &mut self,
        player: Player,
        target: Player,
        bullet_data: PlayerBulletData,
    ) -> bool {
        true
    }

    /// This callback is called when a player shoots a vehicle
    fn on_player_shot_vehicle(
        &mut self,
        player: Player,
        target: Vehicle,
        bullet_data: PlayerBulletData,
    ) -> bool {
        true
    }

    /// This callback is called when a player shoots an object
    fn on_player_shot_object(
        &mut self,
        player: Player,
        target: Object,
        bullet_data: PlayerBulletData,
    ) -> bool {
        true
    }

    /// This callback is called when a player shoots a player object
    fn on_player_shot_player_object(
        &mut self,
        player: Player,
        target: PlayerObject,
        bullet_data: PlayerBulletData,
    ) -> bool {
        true
    }

    /// This callback is called when a player's score changes
    fn on_player_score_change(&mut self, player: Player, score: isize) {}

    /// This callback is called when a player's name is changed
    fn on_player_name_change(&mut self, player: Player, old_name: String) {}

    /// This callback is called when a player changes interior
    fn on_player_interior_change(
        &mut self,
        player: Player,
        new_interior: usize,
        old_interior: usize,
    ) {
    }

    /// This callback is called when a player changes state. For example, when a player changes from being the driver of a vehicle to being on-foot.
    fn on_player_state_change(
        &mut self,
        player: Player,
        new_state: PlayerState,
        old_state: PlayerState,
    ) {
    }

    /// This callback is called when the state of any supported key is changed (pressed/released).
    /// Directional keys do not trigger on_player_key_state_change (up/down/left/right).
    fn on_player_key_state_change(&mut self, player: Player, new_keys: u32, old_keys: u32) {}

    /// This callback is called when a player dies, either by suicide or by being killed by another player.
    fn on_player_death(&mut self, player: Player, killer: Option<Player>, reason: isize) {}

    /// This callback is called when a player takes damage.
    fn on_player_take_damage(
        &mut self,
        player: Player,
        from: Option<Player>,
        amount: f32,
        weapon: usize,
        part: BodyPart,
    ) {
    }

    /// This callback is called when a player gives damage to another player.
    fn on_player_give_damage(
        &mut self,
        player: Player,
        to: Player,
        amount: f32,
        weapon: usize,
        part: BodyPart,
    ) {
    }

    /// This callback is called when a player places a target/waypoint on the pause menu map (by right-clicking).
    fn on_player_click_map(&mut self, player: Player, pos: Vector3) {}

    /// This callback is called when a player double-clicks on a player on the scoreboard.
    fn on_player_click_player(
        &mut self,
        player: Player,
        clicked: Player,
        source: PlayerClickSource,
    ) {
    }

    /// This callback is called when a send_client_check request completes.
    fn on_client_check_response(
        &mut self,
        player: Player,
        action_type: isize,
        address: isize,
        results: isize,
    ) {
    }

    /// This callback is called every time a client/player updates the server with their status.
    /// It can be used to monitor client updates that aren't actively tracked by the server, such as health or armor updates or players switching weapons.
    fn on_player_update(&mut self, player: Player, now: isize) -> bool {
        true
    }

    /// This callback is called when a player finishes downloading custom models
    fn on_player_finished_downloading(&mut self, player: Player) {}

    /// This callback is called when a player request for custom model downloads.
    fn on_player_request_download(
        &mut self,
        player: Player,
        model_type: ModelDownloadType,
        checksum: u32,
    ) -> bool {
        true
    }

    /// This callback is called when a player gives damage to an actor.
    fn on_player_give_damage_actor(
        &mut self,
        player: Player,
        actor: Actor,
        amount: f32,
        weapon: usize,
        part: BodyPart,
    ) {
    }

    /// This callback is called when an actor is streamed in by a player's client.
    fn on_actor_stream_in(&mut self, actor: Actor, player: Player) {}

    /// This callback is called when an actor is streamed out by a player's client.
    fn on_actor_stream_out(&mut self, actor: Actor, player: Player) {}

    /// This callback is called when a player enters the checkpoint set for that player.
    fn on_player_enter_checkpoint(&mut self, player: Player) {}

    /// This callback is called when a player leaves a checkpoint.
    fn on_player_leave_checkpoint(&mut self, player: Player) {}

    /// This callback is called when a player enters a race checkpoint.
    fn on_player_enter_race_checkpoint(&mut self, player: Player) {}

    /// This callback is called when a player leaves a race checkpoint.
    fn on_player_leave_race_checkpoint(&mut self, player: Player) {}

    /// This callback is called when a player changes class at class selection (and when class selection first appears).
    fn on_player_request_class(&mut self, player: Player, class_id: usize) -> bool {
        true
    }

    /// This callback is called when a player responds to a dialog shown using ShowPlayerDialog by either clicking a button, pressing ENTER/ESC or double-clicking a list item (if using a list style dialog).
    fn on_dialog_response(
        &mut self,
        player: Player,
        dialog_id: i16,
        response: DialogResponse,
        list_item: isize,
        input_text: String,
    ) {
    }

    /// This callback is called when a player enters a gangzone
    /// This callback requires the use of use_check method to be enable
    fn on_player_enter_gang_zone(&mut self, player: Player, zone: GangZone) {}

    /// This callback is called when a player exited a gangzone.
    /// This callback requires the use of use_check method to be enable
    fn on_player_leave_gang_zone(&mut self, player: Player, zone: GangZone) {}

    /// This callback is called when a player clicked a gangzone on the pause menu map (by right-clicking).
    fn on_player_click_gang_zone(&mut self, player: Player, zone: GangZone) {}

    /// This callback is called when a player selects an item from a menu
    fn on_player_selected_menu_row(&mut self, player: Player, row: isize) {}

    /// This callback is called when a player exits a menu.
    fn on_player_exited_menu(&mut self, player: Player) {}

    /// This callback is called when an object is completed moving
    fn on_object_moved(&mut self, object: Object) {}

    /// This callback is called when a player object is completed moving
    fn on_player_object_moved(&mut self, player: Player, object: PlayerObject) {}

    /// This callback is called when a player finishes editing an object
    fn on_player_edit_object(
        &mut self,
        player: Player,
        object: Object,
        response: ObjectEditResponse,
        offset: Vector3,
        rotation: Vector3,
    ) {
    }

    /// This callback is called when a player finishes editing a player object
    fn on_player_object_edited(
        &mut self,
        player: Player,
        object: PlayerObject,
        response: ObjectEditResponse,
        offset: Vector3,
        rotation: Vector3,
    ) {
    }

    /// This callback is called when a player ends attached object edition mode.
    fn on_player_edit_attached_object(
        &mut self,
        player: Player,
        index: isize,
        saved: bool,
        data: ObjectAttachmentSlotData,
    ) {
    }

    /// This callback is called when a player selects an object
    fn on_player_select_object(
        &mut self,
        player: Player,
        object: Object,
        model: isize,
        position: Vector3,
    ) {
    }

    /// This callback is called when a player selects a player object
    fn on_player_object_selected(
        &mut self,
        player: Player,
        object: PlayerObject,
        model: isize,
        position: Vector3,
    ) {
    }

    /// This callback is called when a player picks up a pickup created
    fn on_player_pick_up_pickup(&mut self, player: Player, pickup: Pickup) {}

    /// This callback is called when a player cancels the textdraw selection
    fn on_player_cancel_text_draw_selection(&mut self, player: Player) -> bool {
        true
    }

    /// This callback is called when a player cancels the player textdraw selection
    fn on_player_cancel_player_text_draw_selection(&mut self, player: Player) -> bool {
        true
    }

    /// This callback is called when a player clicks a textdraw
    fn on_player_click_text_draw(&mut self, player: Player, textdraw: TextDraw) {}

    /// This callback is called when a player clicks a player textdraw
    fn on_player_click_player_text_draw(&mut self, player: Player, textdraw: PlayerTextDraw) {}

    /// This callback is called when a vehicle is streamed to a player's client.
    fn on_vehicle_stream_in(&mut self, vehicle: Vehicle, player: Player) {}

    /// This callback is called when a vehicle streams out for a player.
    fn on_vehicle_stream_out(&mut self, vehicle: Vehicle, player: Player) {}

    /// This callback is called when a vehicle is destroyed - either by exploding or becoming submerged in water.
    fn on_vehicle_death(&mut self, vehicle: Vehicle, player: Player) {}

    /// This callback is called when a player starts to enter a vehicle, meaning the player is not in vehicle yet at the time this callback is called.
    fn on_player_enter_vehicle(&mut self, player: Player, vehicle: Vehicle, passenger: bool) {}

    /// This callback is called when a player leaves a vehicle.
    fn on_player_exit_vehicle(&mut self, player: Player, vehicle: Vehicle) {}

    /// This callback is called when a vehicle element such as doors, tyres, panels, or lights change their damage status.
    fn on_vehicle_damage_status_update(&mut self, vehicle: Vehicle, player: Player) {}

    /// This callback is called when a player previews a vehicle paintjob inside a mod shop. Watch out, this callback is not called when the player buys the paintjob.
    fn on_vehicle_paint_job(&mut self, player: Player, vehicle: Vehicle, paintjob: isize) -> bool {
        true
    }

    /// This callback is called when a vehicle is modded.
    fn on_vehicle_mod(&mut self, player: Player, vehicle: Vehicle, component: isize) -> bool {
        true
    }

    /// This callback is called when a player exits a mod shop, even if the colors weren't changed. Watch out, the name is ambiguous, Pay 'n' Spray shops don't call this callback.
    fn on_vehicle_respray(
        &mut self,
        player: Player,
        vehicle: Vehicle,
        colour1: isize,
        colour2: isize,
    ) -> bool {
        true
    }

    /// This callback is called when a vehicle enters or exits a mod shop.
    fn on_enter_exit_mod_shop(&mut self, player: Player, enterexit: bool, interior_id: isize) {}

    /// This callback is called when a vehicle respawns.
    fn on_vehicle_spawn(&mut self, vehicle: Vehicle) {}

    /// This callback is called when a player's client updates/syncs the position of a vehicle they're not driving. This can happen outside of the vehicle or when the player is a passenger of a vehicle that has no driver.
    fn on_unoccupied_vehicle_update(
        &mut self,
        vehicle: Vehicle,
        player: Player,
        updateData: UnoccupiedVehicleUpdate,
    ) -> bool {
        true
    }

    /// Called when a trailer's position is synced by a client.
    fn on_trailer_update(&mut self, player: Player, vehicle: Vehicle) -> bool {
        true
    }

    /// This callback is called when a vehicle's siren is toggled.
    fn on_vehicle_siren_state_change(
        &mut self,
        player: Player,
        vehicle: Vehicle,
        sirenstate: u8,
    ) -> bool {
        true
    }

    /// This callback is called when a command is sent through the server console, remote RCON, or via the in-game "/rcon command".
    fn on_rcon_command(&mut self, cmd: String) -> bool {
        true
    }

    /// This callback is called when an attempt to login to RCON is made.
    fn on_rcon_login_attempt(
        &mut self,
        player: Option<Player>,
        ip: String,
        password: String,
        success: bool,
    ) {
    }
}
