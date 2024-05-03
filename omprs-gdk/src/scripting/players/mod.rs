pub mod events;
pub mod functions;

pub use functions::load_functions;

use crate::{
    actors::Actor,
    animationdata::AnimationData,
    classes::{self, PlayerClass},
    colour::Colour,
    dialogs::{self, DialogStyle},
    objects::{self, Object, ObjectAttachmentSlotData, PlayerObject},
    staticarray::StaticArray,
    textdraws::{self, PlayerTextDraw},
    vector::{Vector2, Vector3, Vector4},
    vehicles::{self, Vehicle},
};
use std::os::raw::c_void;

use super::{
    checkpoints::{self, PlayerCheckPointData, PlayerRaceCheckPointData, RaceCheckpointType},
    menus::{self, Menu},
    textlabels::{self, PlayerTextLabel},
};

pub struct Player {
    handle: *const c_void,
}

impl Player {
    pub fn new(handle: *const c_void) -> Self {
        Self { handle }
    }

    pub fn get_handle(&self) -> *const c_void {
        self.handle
    }

    pub fn send_client_message(&self, colour: Colour, message: &str) {
        functions::SendClientMessage(self, colour, message);
    }

    pub fn get_name(&self) -> String {
        let mut name = String::new();
        functions::GetPlayerName(self, &mut name);
        name
    }

    /* pub fn send_client_message_to_all(&self, colour: Colour, message: String) {
        todo!()
    } */

    pub fn set_camera_pos(&self, pos: Vector3) {
        functions::SetPlayerCameraPos(self, pos);
    }

    pub fn set_drunk_level(&self, level: isize) {
        functions::SetPlayerDrunkLevel(self, level);
    }

    pub fn set_interior(&self, interiorid: isize) {
        functions::SetPlayerInterior(self, interiorid);
    }

    pub fn set_wanted_level(&self, level: isize) {
        functions::SetPlayerWantedLevel(self, level);
    }

    pub fn set_weather(&self, weatherid: isize) {
        functions::SetPlayerWeather(self, weatherid);
    }

    pub fn get_weather(&self) -> isize {
        functions::GetPlayerWeather(self)
    }

    pub fn set_skin(&self, skinid: isize) {
        functions::SetPlayerSkin(self, skinid);
    }

    pub fn set_shop_name(&self, shopname: &str) {
        functions::SetPlayerShopName(self, shopname)
    }

    pub fn give_money(&self, amount: isize) {
        functions::GivePlayerMoney(self, amount)
    }

    pub fn set_camera_look_at(&self, pos: Vector3, cut: isize) {
        functions::SetPlayerCameraLookAt(self, pos, cut)
    }

    pub fn set_camera_behind_player(&self) {
        functions::SetCameraBehindPlayer(self)
    }

    pub fn create_explosion(&self, pos: Vector3, explosion_type: isize, radius: f32) {
        functions::CreateExplosionForPlayer(self, pos, explosion_type, radius);
    }
    /* pub fn create_explosion_for_all(&self, pos: Vector3, explosion_type: isize, radius: f32) {
        todo!()
    } */
    pub fn play_audio_stream(&self, url: &str, pos: Vector3, distance: f32, use_pos: bool) {
        functions::PlayAudioStreamForPlayer(self, url, pos, distance, use_pos);
    }

    pub fn stop_audio_stream(&self) {
        functions::StopAudioStreamForPlayer(self)
    }

    pub fn send_death_message(&self, killee: Player, weapon: isize) {
        functions::SendDeathMessage(self, &killee, weapon);
    }

    pub fn toggle_widescreen(&self, enable: bool) {
        functions::TogglePlayerWidescreen(self, enable)
    }

    pub fn is_widescreen_toggled(&self) -> bool {
        functions::IsPlayerWidescreenToggled(self)
    }

    pub fn set_health(&self, health: f32) {
        functions::SetPlayerHealth(self, health);
    }

    pub fn get_health(&self) -> f32 {
        functions::GetPlayerHealth(self)
    }

    pub fn set_armour(&self, armour: f32) {
        functions::SetPlayerArmour(self, armour)
    }

    pub fn get_armour(&self) -> f32 {
        functions::GetPlayerArmour(self)
    }

    pub fn set_team(&self, teamid: isize) {
        functions::SetPlayerTeam(self, teamid)
    }

    pub fn get_team(&self) -> isize {
        functions::GetPlayerTeam(self)
    }

    pub fn set_score(&self, score: isize) {
        functions::SetPlayerScore(self, score)
    }

    pub fn get_score(&self) -> isize {
        functions::GetPlayerScore(self)
    }

    pub fn get_skin(&self) -> isize {
        functions::GetPlayerSkin(self)
    }

    pub fn set_color(&self, colour: Colour) {
        functions::SetPlayerColor(self, colour)
    }

    pub fn get_color(&self) -> isize {
        functions::GetPlayerColor(self)
    }

    pub fn get_default_colour(&self) -> isize {
        functions::GetDefaultPlayerColour(self)
    }

    pub fn get_drunk_level(&self) -> isize {
        functions::GetPlayerDrunkLevel(self)
    }

    pub fn give_weapon(&self, data: WeaponSlotData) {
        functions::GivePlayerWeapon(self, data)
    }

    pub fn remove_weapon(&self, weaponid: u8) {
        functions::RemovePlayerWeapon(self, weaponid)
    }

    pub fn get_money(&self) -> isize {
        functions::GetPlayerMoney(self)
    }

    pub fn reset_money(&self) {
        functions::ResetPlayerMoney(self)
    }

    pub fn set_name(&self, name: &str) -> EPlayerNameStatus {
        functions::SetPlayerName(self, name)
    }

    pub fn get_state(&self) -> PlayerState {
        functions::GetPlayerState(self)
    }

    pub fn get_ping(&self) -> isize {
        functions::GetPlayerPing(self)
    }

    pub fn get_weapon(&self) -> isize {
        functions::GetPlayerWeapon(self)
    }

    pub fn set_time(&self, hour: isize, minute: isize) {
        functions::SetPlayerTime(self, hour, minute)
    }

    pub fn get_time(&self, hour: &mut isize, minute: &mut isize) {
        functions::GetPlayerTime(self, hour, minute)
    }

    pub fn toggle_clock(&self, enable: bool) {
        functions::TogglePlayerClock(self, enable)
    }

    pub fn has_clock_enabled(&self) -> bool {
        functions::PlayerHasClockEnabled(self)
    }

    pub fn force_class_selection(&self) {
        functions::ForceClassSelection(self)
    }

    pub fn get_wanted_level(&self) -> isize {
        functions::GetPlayerWantedLevel(self)
    }

    pub fn set_fighting_style(&self, style: PlayerFightingStyle) {
        functions::SetPlayerFightingStyle(self, style)
    }

    pub fn get_fighting_style(&self) -> PlayerFightingStyle {
        functions::GetPlayerFightingStyle(self)
    }

    pub fn set_velocity(&self, pos: Vector3) {
        functions::SetPlayerVelocity(self, pos)
    }

    pub fn get_velocity(&self) -> Vector3 {
        let mut pos = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        functions::GetPlayerVelocity(self, &mut pos);
        pos
    }

    pub fn get_camera_pos(&self) -> Vector3 {
        let mut pos = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        functions::GetPlayerCameraPos(self, &mut pos);
        pos
    }

    pub fn get_distance_from_point(&self, pos: Vector3) -> f32 {
        functions::GetPlayerDistanceFromPoint(self, pos)
    }

    pub fn get_interior(&self) -> isize {
        functions::GetPlayerInterior(self)
    }

    pub fn set_pos(&self, pos: Vector3) {
        functions::SetPlayerPos(self, pos)
    }

    pub fn get_pos(&self) -> Vector3 {
        let mut pos = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        functions::GetPlayerPos(self, &mut pos);
        pos
    }

    pub fn get_virtual_world(&self) -> isize {
        functions::GetPlayerVirtualWorld(self)
    }

    pub fn is_npc(&self) -> bool {
        functions::IsPlayerNPC(self)
    }

    pub fn is_streamed_in(&self, other: Player) -> bool {
        functions::IsPlayerStreamedIn(self, &other)
    }

    pub fn play_sound(&self, sound: usize, pos: Vector3) {
        functions::PlayerPlaySound(self, sound, pos)
    }

    pub fn spectate_player(&self, target: Player, mode: PlayerSpectateMode) {
        functions::PlayerSpectatePlayer(self, &target, mode)
    }

    pub fn spectate_vehicle(&self, vehicle: Vehicle, mode: PlayerSpectateMode) {
        functions::PlayerSpectateVehicle(self, &vehicle, mode)
    }

    pub fn set_virtual_world(&self, vw: isize) {
        functions::SetPlayerVirtualWorld(self, vw)
    }

    pub fn set_world_bounds(&self, coords: Vector4) {
        functions::SetPlayerWorldBounds(self, coords)
    }

    pub fn clear_world_bounds(&self) {
        functions::ClearPlayerWorldBounds(self)
    }

    pub fn get_world_bounds(&self) -> Vector4 {
        let mut pos = Vector4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };
        functions::GetPlayerWorldBounds(self, &mut pos);
        pos
    }

    pub fn clear_animations(&self, sync_type: PlayerAnimationSyncType) {
        functions::ClearAnimations(self, sync_type)
    }

    pub fn get_last_shot_vectors(&self) -> PlayerBulletData {
        functions::GetPlayerLastShotVectors(self)
    }

    pub fn get_camera_target_player(&self) -> Option<Player> {
        functions::GetPlayerCameraTargetPlayer(self)
    }

    pub fn get_camera_target_actor(&self) -> Option<Actor> {
        functions::GetPlayerCameraTargetActor(self)
    }

    pub fn get_camera_target_object(&self) -> Option<Object> {
        functions::GetPlayerCameraTargetObject(self)
    }

    pub fn get_camera_target_vehicle(&self) -> Option<Vehicle> {
        functions::GetPlayerCameraTargetVehicle(self)
    }

    pub fn put_in_vehicle(&self, vehicle: Vehicle, seat_id: isize) {
        functions::PutPlayerInVehicle(self, &vehicle, seat_id)
    }

    pub fn remove_building(&self, model: isize, pos: Vector3, radius: f32) {
        functions::RemoveBuildingForPlayer(self, model, pos, radius)
    }

    pub fn get_buildings_removed(&self) -> isize {
        functions::GetPlayerBuildingsRemoved(self)
    }

    pub fn remove_from_vehicle(&self, force: bool) {
        functions::RemovePlayerFromVehicle(self, force)
    }

    pub fn remove_map_icon(&self, icon_id: isize) {
        functions::RemovePlayerMapIcon(self, icon_id)
    }

    pub fn set_map_icon(
        &self,
        icon_id: isize,
        pos: Vector3,
        icon_type: isize,
        colour: Colour,
        style: MapIconStyle,
    ) {
        functions::SetPlayerMapIcon(self, icon_id, pos, icon_type, colour, style)
    }

    pub fn reset_weapons(&self) {
        functions::ResetPlayerWeapons(self)
    }

    pub fn set_ammo(&self, data: WeaponSlotData) {
        functions::SetPlayerAmmo(self, data)
    }

    pub fn set_armed_weapon(&self, weapon: isize) {
        functions::SetPlayerArmedWeapon(self, weapon)
    }

    pub fn set_chat_bubble(
        &self,
        text: &str,
        colour: Colour,
        drawdistance: f32,
        expiretime: isize,
    ) {
        functions::SetPlayerChatBubble(self, text, colour, drawdistance, expiretime)
    }

    pub fn set_pos_find_z(&self, pos: Vector3) {
        functions::SetPlayerPosFindZ(self, pos)
    }

    pub fn set_skill_level(&self, weapon: PlayerWeaponSkill, level: isize) {
        functions::SetPlayerSkillLevel(self, weapon, level)
    }

    pub fn set_special_action(&self, action: PlayerSpecialAction) {
        functions::SetPlayerSpecialAction(self, action)
    }

    pub fn show_name_tag(&self, other: Player, enable: bool) {
        functions::ShowPlayerNameTagForPlayer(self, &other, enable)
    }

    pub fn toggle_controllable(&self, enable: bool) {
        functions::TogglePlayerControllable(self, enable)
    }

    pub fn toggle_spectating(&self, enable: bool) {
        functions::TogglePlayerSpectating(self, enable)
    }

    pub fn apply_animation(&self, animation_data: AnimationData, sync: PlayerAnimationSyncType) {
        functions::ApplyAnimation(
            self,
            &animation_data.get_animation_library(),
            &animation_data.get_name(),
            animation_data.delta,
            animation_data.looping,
            animation_data.lockX,
            animation_data.lockY,
            animation_data.freeze,
            animation_data.time,
            sync,
        )
    }

    /* pub fn get_animation_name(&self, index: isize, lib: &mut String, name: &mut String) {
        functions::GetAnimationName(lib, name)
    } */

    pub fn edit_attached_object(&self, index: isize) {
        functions::EditAttachedObject(self, index)
    }

    pub fn enable_camera_target(&self, enable: bool) {
        functions::EnablePlayerCameraTarget(self, enable)
    }

    pub fn enable_stunt_bonus(&self, enable: bool) {
        functions::EnableStuntBonusForPlayer(self, enable)
    }

    /*  pub fn enable_stunt_bonus_for_all(&self, enable: bool) {
        todo!()
    } */

    pub fn get_ammo(&self) -> isize {
        functions::GetPlayerAmmo(self)
    }

    pub fn get_animation_index(&self) -> isize {
        functions::GetPlayerAnimationIndex(self)
    }

    pub fn get_facing_angle(&self) -> f32 {
        functions::GetPlayerFacingAngle(self)
    }

    pub fn get_ip(&self) -> String {
        let mut ip = String::new();
        functions::GetPlayerIp(self, &mut ip);
        ip
    }

    pub fn get_special_action(&self) -> PlayerSpecialAction {
        functions::GetPlayerSpecialAction(self)
    }

    pub fn get_vehicle_id(&self) -> isize {
        functions::GetPlayerVehicleID(self)
    }

    pub fn get_vehicle_seat(&self) -> isize {
        functions::GetPlayerVehicleSeat(self)
    }

    pub fn get_weapon_data(&self, slot: isize) -> WeaponSlotData {
        let mut weapon: WeaponSlotData = WeaponSlotData { ammo: 0, id: 0 };
        functions::GetPlayerWeaponData(self, slot, &mut weapon);
        weapon
    }

    pub fn get_weapon_state(&self) -> isize {
        functions::GetPlayerWeaponState(self)
    }

    pub fn interpolate_camera_pos(
        &self,
        from: Vector3,
        to: Vector3,
        time: isize,
        cut: PlayerCameraCutType,
    ) {
        functions::InterpolateCameraPos(self, from, to, time, cut)
    }

    pub fn interpolate_camera_look_at(
        &self,
        from: Vector3,
        to: Vector3,
        time: isize,
        cut: PlayerCameraCutType,
    ) -> bool {
        functions::InterpolateCameraLookAt(self, from, to, time, cut)
    }

    pub fn is_attached_object_slot_used(&self, index: isize) -> bool {
        functions::IsPlayerAttachedObjectSlotUsed(self, index)
    }

    pub fn attach_camera_to_object(&self, object: Object) {
        functions::AttachCameraToObject(self, &object)
    }

    pub fn attach_camera_to_player_object(&self, object: PlayerObject) {
        functions::AttachCameraToPlayerObject(self, &object)
    }

    pub fn get_aim_data(&self) -> PlayerAimData {
        functions::GetPlayerAimData(self)
    }

    pub fn get_keys(
        &self,
        keys: &mut isize,
        updown: &mut isize,
        leftright: &mut isize,
    ) -> PlayerKeyData {
        functions::GetPlayerKeys(self, keys, updown, leftright)
    }

    pub fn get_surfing_data(&self) -> PlayerSurfingData {
        functions::GetPlayerSurfingData(self)
    }

    pub fn get_target_player(&self) -> Option<Player> {
        functions::GetPlayerTargetPlayer(self)
    }

    pub fn get_target_actor(&self) -> Option<Actor> {
        functions::GetPlayerTargetActor(self)
    }

    pub fn is_in_vehicle(&self, target_vehicle: Vehicle) -> bool {
        functions::IsPlayerInVehicle(self, &target_vehicle)
    }

    pub fn is_in_any_vehicle(&self) -> bool {
        functions::IsPlayerInAnyVehicle(self)
    }

    pub fn is_in_range_of_point(&self, range: f32, coord: Vector3) -> bool {
        functions::IsPlayerInRangeOfPoint(self, range, coord)
    }

    pub fn play_crime_report(&self, suspect: Player, crime: isize) -> bool {
        functions::PlayCrimeReportForPlayer(self, &suspect, crime)
    }

    pub fn remove_attached_object(&self, index: isize) {
        functions::RemovePlayerAttachedObject(self, index)
    }
    //pub fn SetPlayerAttachedObject(&self, index: isize, attachment: ObjectAttachmentSlotData) { todo!() }
    //pub fn GetPlayerAttachedObject(&self, index: isize) -> ObjectAttachmentSlotData { todo!() }
    pub fn set_facing_angle(&self, angle: f32) {
        functions::SetPlayerFacingAngle(self, angle)
    }

    pub fn set_marker_for_player(&self, other: Player, colour: Colour) {
        functions::SetPlayerMarkerForPlayer(self, &other, colour)
    }

    pub fn get_marker_for_player(&self, other: Player) -> isize {
        functions::GetPlayerMarkerForPlayer(self, &other)
    }

    pub fn allow_teleport(&self, allow: bool) {
        functions::AllowPlayerTeleport(self, allow)
    }

    pub fn is_teleport_allowed(&self) -> bool {
        functions::IsPlayerTeleportAllowed(self)
    }

    pub fn set_remote_vehicle_collisions(&self, collide: bool) {
        functions::SetRemoteVehicleCollisions(self, collide)
    }

    pub fn select_text_draw(&self, hover_colour: Colour) {
        functions::SelectTextDraw(self, hover_colour)
    }

    pub fn cancel_select_text_draw(&self) {
        functions::CancelSelectTextDraw(self)
    }

    pub fn send_client_check(
        &self,
        action_type: isize,
        address: isize,
        offset: isize,
        count: isize,
    ) {
        functions::SendClientCheck(self, action_type, address, offset, count)
    }

    pub fn spawn(&self) {
        functions::SpawnPlayer(self)
    }

    pub fn gpci(&self) -> String {
        let mut output = String::new();
        functions::gpci(self, &mut output);
        output
    }

    pub fn is_admin(&self) -> bool {
        functions::IsPlayerAdmin(self)
    }
    pub fn kick(&self) {
        functions::Kick(self)
    }

    pub fn game_text(&self, msg: &str, time: isize, style: isize) {
        functions::GameTextForPlayer(self, msg, time, style)
    }

    pub fn hide_game_text(&self, style: isize) {
        functions::HideGameTextForPlayer(self, style)
    }

    pub fn has_game_text(&self, style: isize) -> bool {
        functions::HasGameText(self, style)
    }

    pub fn get_game_text(
        &self,
        style: isize,
        message: &mut String,
        time: &mut isize,
        remaining: &mut isize,
    ) -> bool {
        functions::GetGameText(self, style, message, time, remaining)
    }

    pub fn ban(&self) {
        functions::Ban(self)
    }

    pub fn ban_ex(&self, msg: &str) {
        functions::BanEx(self, msg)
    }

    pub fn send_death_message_to_player(&self, killer: Player, killee: Player, weapon: isize) {
        functions::SendDeathMessageToPlayer(self, &killer, &killee, weapon)
    }

    pub fn send_message_to_player(&self, sender: Player, message: &str) {
        functions::SendPlayerMessageToPlayer(self, &sender, message)
    }

    pub fn get_version(&self, output: &mut String) {
        functions::GetPlayerVersion(self, output)
    }

    pub fn get_skill_level(&self, skill: isize) -> isize {
        functions::GetPlayerSkillLevel(self, skill)
    }

    pub fn get_spectate_id(&self) -> isize {
        functions::GetPlayerSpectateID(self)
    }

    pub fn get_spectate_data(&self) -> PlayerSpectateData {
        functions::GetPlayerSpectateData(self)
    }

    pub fn get_raw_ip(&self) -> isize {
        functions::GetPlayerRawIp(self)
    }

    pub fn set_gravity(&self, gravity: f32) {
        functions::SetPlayerGravity(self, gravity)
    }

    pub fn get_gravity(&self) -> f32 {
        functions::GetPlayerGravity(self)
    }

    pub fn set_admin(&self, set: bool) {
        functions::SetPlayerAdmin(self, set)
    }

    pub fn is_spawned(&self) -> bool {
        functions::IsPlayerSpawned(self)
    }

    pub fn is_controllable(&self) -> bool {
        functions::IsPlayerControllable(self)
    }

    pub fn is_camera_target_enabled(&self) -> bool {
        functions::IsPlayerCameraTargetEnabled(self)
    }

    pub fn toggle_ghost_mode(&self, toggle: bool) {
        functions::TogglePlayerGhostMode(self, toggle)
    }

    pub fn get_ghost_mode(&self) -> bool {
        functions::GetPlayerGhostMode(self)
    }

    pub fn allow_weapons(&self, allow: bool) {
        functions::AllowPlayerWeapons(self, allow)
    }

    pub fn are_weapons_allowed(&self) -> bool {
        functions::ArePlayerWeaponsAllowed(self)
    }

    pub fn is_using_official_client(&self) -> bool {
        functions::IsPlayerUsingOfficialClient(self)
    }

    pub fn get_animation_data(&self) -> PlayerAnimationData {
        functions::GetPlayerAnimationData(self)
    }

    pub fn is_in_drive_by_mode(&self) -> bool {
        functions::IsPlayerInDriveByMode(self)
    }

    pub fn is_cuffed(&self) -> bool {
        functions::IsPlayerCuffed(self)
    }

    pub fn get_custom_skin(&self) -> isize {
        functions::GetPlayerCustomSkin(self)
    }

    pub fn redirect_download(&self, url: &str) -> bool {
        functions::RedirectDownload(self, url)
    }

    // Player Checkpoints methods
    pub fn set_player_checkpoint(&self, centre_position: Vector3, radius: f32) {
        checkpoints::functions::SetPlayerCheckpoint(self, centre_position, radius);
    }

    pub fn disable_player_checkpoint(&self) {
        checkpoints::functions::DisablePlayerCheckpoint(self);
    }

    pub fn is_player_in_checkpoint(&self) -> bool {
        checkpoints::functions::IsPlayerInCheckpoint(self)
    }

    pub fn set_player_race_checkpoint(
        &self,
        race_check_point_type: RaceCheckpointType,
        centre_position: Vector3,
        next_position: Vector3,
        radius: f32,
    ) {
        checkpoints::functions::SetPlayerRaceCheckpoint(
            self,
            race_check_point_type,
            centre_position,
            next_position,
            radius,
        )
    }

    pub fn disable_player_race_checkpoint(&self) {
        checkpoints::functions::DisablePlayerRaceCheckpoint(self)
    }

    pub fn is_player_in_race_checkpoint(&self) -> bool {
        checkpoints::functions::IsPlayerInRaceCheckpoint(self)
    }

    pub fn is_player_checkpoint_active(&self) -> bool {
        checkpoints::functions::IsPlayerCheckpointActive(self)
    }

    pub fn get_player_checkpoint(&self) -> PlayerCheckPointData {
        let mut center_pos = Vector3::default();
        let mut radius = 0.0;
        checkpoints::functions::GetPlayerCheckpoint(self, &mut center_pos, &mut radius);
        PlayerCheckPointData::new(center_pos, radius)
    }

    pub fn is_player_race_checkpoint_active(&self) -> bool {
        checkpoints::functions::IsPlayerRaceCheckpointActive(self)
    }

    pub fn get_player_race_checkpoint(&self) -> PlayerRaceCheckPointData {
        let mut center_pos = Vector3::default();
        let mut next_pos = Vector3::default();
        let mut radius = 0.0;
        checkpoints::functions::GetPlayerRaceCheckpoint(
            self,
            &mut center_pos,
            &mut next_pos,
            &mut radius,
        );
        PlayerRaceCheckPointData::new(center_pos, next_pos, radius)
    }

    pub fn set_spawn_info(&self, player_class: PlayerClass) {
        classes::functions::SetSpawnInfo(self, player_class)
    }

    pub fn get_spawn_info(&self) -> PlayerClass {
        let mut data = PlayerClass::default();
        classes::functions::GetSpawnInfo(self, &mut data);
        data
    }

    pub fn show_dialog(
        &self,
        dialog: i16,
        style: DialogStyle,
        title: &str,
        body: &str,
        button1: &str,
        button2: &str,
    ) {
        dialogs::functions::ShowPlayerDialog(self, dialog, style, title, body, button1, button2)
    }

    pub fn get_dialog_id(&self) -> i16 {
        dialogs::functions::GetPlayerDialogID(self)
    }

    pub fn hide_dialog(&self) -> bool {
        dialogs::functions::HidePlayerDialog(self)
    }

    pub fn get_id(&self) -> usize {
        functions::GetPlayerID(self)
    }

    pub fn get_menu(&self) -> Option<Menu> {
        menus::functions::GetPlayerMenu(self)
    }

    pub fn edit_object(&self, object: &Object) {
        objects::functions::EditObject(self, object)
    }
    pub fn select_object(&self) {
        objects::functions::SelectObject(self)
    }
    pub fn end_object_editing(&self) {
        objects::functions::EndObjectEditing(self)
    }
    pub fn create_player_object(
        &self,
        modelid: isize,
        position: Vector3,
        rotation: Vector3,
        drawDistance: f32,
    ) -> Option<PlayerObject> {
        objects::functions::CreatePlayerObject(self, modelid, position, rotation, drawDistance)
    }
    pub fn destroy_player_object(&self, object: PlayerObject) {
        objects::functions::DestroyPlayerObject(self, &object);
    }
    pub fn edit_player_object(&self, object: &PlayerObject) {
        objects::functions::EditPlayerObject(self, object)
    }
    pub fn create_player_text_draw(&self, position: Vector2, text: &str) -> Option<PlayerTextDraw> {
        textdraws::functions::CreatePlayerTextDraw(self, position, text)
    }
    pub fn player_text_draw_destroy(&self, textdraw: &PlayerTextDraw) {
        textdraws::functions::PlayerTextDrawDestroy(self, textdraw)
    }

    pub fn create_player_text_label_on_player(
        &self,
        attachedPlayer: &Player,
        text: &str,
        colour: Colour,
        position: Vector3,
        drawDistance: f32,
        los: bool,
    ) -> Option<PlayerTextLabel> {
        textlabels::functions::CreatePlayer3DTextLabelOnPlayer(
            self,
            attachedPlayer,
            text,
            colour,
            position,
            drawDistance,
            los,
        )
    }
    pub fn create_player_text_label_on_vehicle(
        &self,
        attachedVehicle: &Vehicle,
        text: &str,
        colour: Colour,
        position: Vector3,
        drawDistance: f32,
        los: bool,
    ) -> Option<PlayerTextLabel> {
        textlabels::functions::CreatePlayer3DTextLabelOnVehicle(
            self,
            attachedVehicle,
            text,
            colour,
            position,
            drawDistance,
            los,
        )
    }
    pub fn create_player_text_label(
        &self,
        text: &str,
        colour: Colour,
        position: Vector3,
        drawDistance: f32,
        los: bool,
    ) -> Option<PlayerTextLabel> {
        textlabels::functions::CreatePlayer3DTextLabel(
            self,
            text,
            colour,
            position,
            drawDistance,
            los,
        )
    }
    pub fn delete_player_text_label(&self, textlabel: PlayerTextLabel) {
        textlabels::functions::DeletePlayer3DTextLabel(self, &textlabel)
    }

    pub fn is_player_in_mod_shop(&self) -> bool {
        vehicles::functions::IsPlayerInModShop(self)
    }
    pub fn get_player_siren_state(&self) -> isize {
        vehicles::functions::GetPlayerSirenState(self)
    }
    pub fn get_player_landing_gear_state(&self) -> isize {
        vehicles::functions::GetPlayerLandingGearState(self)
    }
    pub fn get_player_hydra_reactor_angle(&self) -> isize {
        vehicles::functions::GetPlayerHydraReactorAngle(self)
    }
    pub fn get_player_train_speed(&self) -> f32 {
        vehicles::functions::GetPlayerTrainSpeed(self)
    }

    pub fn net_stats__bytes_received(&self) -> isize {
        functions::NetStats_BytesReceived(self)
    }
    pub fn net_stats__bytes_sent(&self) -> isize {
        functions::NetStats_BytesSent(self)
    }
    pub fn net_stats__connection_status(&self) -> isize {
        functions::NetStats_ConnectionStatus(self)
    }
    pub fn net_stats__get_connected_time(&self) -> isize {
        functions::NetStats_GetConnectedTime(self)
    }
    pub fn net_stats__get_ip_port(&self) -> String {
        let mut output = String::new();
        functions::NetStats_GetIpPort(self, &mut output);
        output
    }
    pub fn net_stats__messages_received(&self) -> isize {
        functions::NetStats_MessagesReceived(self)
    }
    pub fn net_stats__messages_recv_per_second(&self) -> isize {
        functions::NetStats_MessagesRecvPerSecond(self)
    }
    pub fn net_stats__messages_sent(&self) -> isize {
        functions::NetStats_MessagesSent(self)
    }
    pub fn net_stats__packet_loss_percent(&self) -> f32 {
        functions::NetStats_PacketLossPercent(self)
    }
    pub fn send_message_to_all(&self, message: &str) {
        functions::SendPlayerMessageToAll(self, message)
    }

    pub fn set_attached_object(&self, index: isize, attachment: ObjectAttachmentSlotData) {
        functions::SetPlayerAttachedObject(self, index, attachment)
    }
    pub fn get_attached_object(&self, index: isize) -> ObjectAttachmentSlotData {
        functions::GetPlayerAttachedObject(self, index)
    }
}

#[repr(C)]
pub enum MapIconStyle {
    MapIconStyleLocal,
    MapIconStyleGlobal,
    MapIconStyleLocalCheckpoint,
    MapIconStyleGlobalCheckpoint,
}

#[repr(u8)]
pub enum ClientVersion {
    ClientVersionSamp037,
    ClientVersionSamp03dl,
    ClientVersionOpenmp,
}

#[repr(C)]
pub enum PlayerCameraCutType {
    PlayerCameraCutTypeCut,
    PlayerCameraCutTypeMove,
}

/// The player's name status returned when updating their name
#[repr(C)]
pub enum EPlayerNameStatus {
    Updated, // The name has successfully been updated
    Taken,   // The name is already taken by another player
    Invalid, // The name is invalid
}

pub enum PlayerAnimationSyncType {
    PlayerAnimationSyncTypeNoSync,
    PlayerAnimationSyncTypeSync,
    PlayerAnimationSyncTypeSyncOthers,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct WeaponSlotData {
    id: u8,
    ammo: u32,
}

impl WeaponSlotData {
    pub fn new(id: u8, ammo: u32) -> Self {
        Self { id, ammo }
    }
}

#[repr(C)]
pub struct PlayerAnimationData {
    ID: u16,
    flags: u16,
}
#[repr(C)]
pub enum PlayerFightingStyle {
    PlayerFightingStyleNormal = 4,
    PlayerFightingStyleBoxing = 5,
    PlayerFightingStyleKungFu = 6,
    PlayerFightingStyleKneeHead = 7,
    PlayerFightingStyleGrabKick = 15,
    PlayerFightingStyleElbow = 16,
}

#[repr(C)]
pub enum PlayerState {
    PlayerStateNone = 0,
    PlayerStateOnFoot = 1,
    PlayerStateDriver = 2,
    PlayerStatePassenger = 3,
    PlayerStateExitVehicle = 4,
    PlayerStateEnterVehicleDriver = 5,
    PlayerStateEnterVehiclePassenger = 6,
    PlayerStateWasted = 7,
    PlayerStateSpawned = 8,
    PlayerStateSpectating = 9,
}

#[repr(C)]
pub enum PlayerWeaponSkill {
    PlayerWeaponSkillPistol,
    PlayerWeaponSkillSilencedPistol,
    PlayerWeaponSkillDesertEagle,
    PlayerWeaponSkillShotgun,
    PlayerWeaponSkillSawnOff,
    PlayerWeaponSkillSpas12,
    PlayerWeaponSkillUzi,
    PlayerWeaponSkillMp5,
    PlayerWeaponSkillAk47,
    PlayerWeaponSkillM4,
    PlayerWeaponSkillSniper,
}

#[repr(C)]
pub enum PlayerSpecialAction {
    SpecialActionNone,
    SpecialActionDuck,
    SpecialActionJetpack,
    SpecialActionEnterVehicle,
    SpecialActionExitVehicle,
    SpecialActionDance1,
    SpecialActionDance2,
    SpecialActionDance3,
    SpecialActionDance4,
    SpecialActionHandsUp = 10,
    SpecialActionCellphone,
    SpecialActionSitting,
    SpecialActionStopCellphone,
    SpecialActionBeer = 20,
    SpecialactionSmoke,
    SpecialActionWine,
    SpecialActionSprunk,
    SpecialActionCuffed,
    SpecialActionCarry,
    SpecialActionPissing = 68,
}

#[repr(C)]
pub struct PlayerSurfingData {
    surftype: isize,
    ID: isize,
    offset: Vector3,
}

#[repr(C)]
pub struct PlayerKeyData {
    // todo fill with union
    keys: u32,
    upDown: i16,
    leftRight: i16,
}

#[repr(C)]
pub struct PlayerBulletData {
    origin: Vector3,
    hitPos: Vector3,
    offset: Vector3,
    weapon: u8,
    hitType: PlayerBulletHitType,
    hitID: u16,
}
#[repr(C)]
pub enum PlayerBulletHitType {
    PlayerBulletHitTypeNone,
    PlayerBulletHitTypePlayer = 1,
    PlayerBulletHitTypeVehicle = 2,
    PlayerBulletHitTypeObject = 3,
    PlayerBulletHitTypePlayerObject = 4,
}

#[repr(C)]
pub enum ESpectateType {
    None,
    Vehicle,
    Player,
}

#[repr(C)]
pub struct PlayerSpectateData {
    spectating: bool,
    spectateID: isize,
    spectate_type: ESpectateType,
}

#[repr(C)]
pub enum PlayerSpectateMode {
    PlayerSpectateModeNormal = 1,
    PlayerSpectateModeFixed,
    PlayerSpectateModeSide,
}

#[repr(C)]
pub struct PlayerAimData {
    camFrontVector: Vector3,
    camPos: Vector3,
    aimZ: f32,
    camZoom: f32,
    aspectRatio: f32,
    weaponState: PlayerWeaponState,
    camMode: u8,
}

#[repr(C)]
pub enum PlayerWeaponState {
    PlayerWeaponStateUnknown = -1,
    PlayerWeaponStateNoBullets,
    PlayerWeaponStateLastBullet,
    PlayerWeaponStateMoreBullets,
    PlayerWeaponStateReloading,
}

#[repr(C)]
pub enum BodyPart {
    BodyPartTorso = 3,
    BodyPartGroin,
    BodyPartLeftArm,
    BodyPartRightArm,
    BodyPartLeftLeg,
    BodyPartRightLeg,
    BodyPartHead,
}

#[repr(C)]
pub enum PlayerClickSource {
    PlayerClickSourceScoreboard,
}

pub type WeaponSlots = StaticArray<WeaponSlotData, 13>;
