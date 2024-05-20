pub mod events;
pub mod functions;

pub use functions::load_functions;

use crate::{
    actors::Actor,
    classes::{self, PlayerClass},
    dialogs::{self, DialogStyle},
    objects::{self, Object, ObjectAttachmentSlotData, PlayerObject},
    textdraws::{self, PlayerTextDraw},
    types::animationdata::AnimationData,
    types::colour::Colour,
    types::staticarray::StaticArray,
    types::vector::{Vector2, Vector3, Vector4},
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

    /// This function sends a message to a specific player with a chosen color in the chat.
    pub fn send_client_message(&self, colour: Colour, message: &str) {
        functions::SendClientMessage(self, colour, message);
    }

    /// Get a player's name.
    pub fn get_name(&self) -> String {
        let mut name = String::new();
        functions::GetPlayerName(self, &mut name);
        name
    }

    /// Sets the camera to a specific position for a player.
    pub fn set_camera_pos(&self, pos: Vector3) {
        functions::SetPlayerCameraPos(self, pos);
    }

    /// Sets the drunk level of a player which makes the player's camera sway and vehicles hard to control.
    pub fn set_drunk_level(&self, level: isize) {
        functions::SetPlayerDrunkLevel(self, level);
    }

    /// Set a player's interior.
    pub fn set_interior(&self, interiorid: isize) {
        functions::SetPlayerInterior(self, interiorid);
    }

    /// Set a player's wanted level (6 brown stars under HUD).
    pub fn set_wanted_level(&self, level: isize) {
        functions::SetPlayerWantedLevel(self, level);
    }

    /// Set a player's weather.
    pub fn set_weather(&self, weatherid: isize) {
        functions::SetPlayerWeather(self, weatherid);
    }

    /// Get a player's weather.
    pub fn get_weather(&self) -> isize {
        functions::GetPlayerWeather(self)
    }

    /// Set the skin of a player.
    pub fn set_skin(&self, skinid: isize) {
        functions::SetPlayerSkin(self, skinid);
    }

    /// Loads or unloads an interior script for a player (for example the ammunation menu).
    pub fn set_shop_name(&self, shopname: &str) {
        functions::SetPlayerShopName(self, shopname)
    }

    /// Give money to or take money from a player.
    pub fn give_money(&self, amount: isize) {
        functions::GivePlayerMoney(self, amount)
    }

    /// Set the direction a player's camera looks at.
    pub fn set_camera_look_at(&self, pos: Vector3, cut: PlayerCameraCutType) {
        functions::SetPlayerCameraLookAt(self, pos, cut)
    }

    /// Restore the camera to a place behind the player, after using a function like SetPlayerCameraPos.
    pub fn set_camera_behind_player(&self) {
        functions::SetCameraBehindPlayer(self)
    }

    /// Creates an explosion that is only visible to a single player.
    pub fn create_explosion(&self, pos: Vector3, explosion_type: isize, radius: f32) {
        functions::CreateExplosionForPlayer(self, pos, explosion_type, radius);
    }

    /// Play an 'audio stream' for a player.
    pub fn play_audio_stream(&self, url: &str, pos: Vector3, distance: f32, use_pos: bool) {
        functions::PlayAudioStreamForPlayer(self, url, pos, distance, use_pos);
    }

    /// Stops the current audio stream for a player.
    pub fn stop_audio_stream(&self) {
        functions::StopAudioStreamForPlayer(self)
    }

    /// Adds a death to the 'killfeed' on the right-hand side of the screen for all players.
    pub fn send_death_message(&self, killee: &Player, weapon: PlayerWeapon) {
        functions::SendDeathMessage(self, killee, weapon);
    }

    /// Toggle player's widescreen.
    pub fn toggle_widescreen(&self, enable: bool) {
        functions::TogglePlayerWidescreen(self, enable)
    }

    /// Checks if a player widescreen is on or off.
    pub fn is_widescreen_toggled(&self) -> bool {
        functions::IsPlayerWidescreenToggled(self)
    }

    /// Set the health of a player.
    pub fn set_health(&self, health: f32) {
        functions::SetPlayerHealth(self, health);
    }

    /// The function GetPlayerHealth allows you to retrieve the health of a player.
    pub fn get_health(&self) -> f32 {
        functions::GetPlayerHealth(self)
    }

    /// Set a player's armor level.
    pub fn set_armour(&self, armour: f32) {
        functions::SetPlayerArmour(self, armour)
    }

    /// This function stores the armour of a player into a variable.
    pub fn get_armour(&self) -> f32 {
        functions::GetPlayerArmour(self)
    }

    /// Set the team of a player.
    pub fn set_team(&self, teamid: isize) {
        functions::SetPlayerTeam(self, teamid)
    }

    /// Get the ID of the team the player is on.
    pub fn get_team(&self) -> isize {
        functions::GetPlayerTeam(self)
    }

    /// Set a player's score.
    pub fn set_score(&self, score: isize) {
        functions::SetPlayerScore(self, score)
    }

    /// This function returns a player's score as it was set using SetPlayerScore.
    pub fn get_score(&self) -> isize {
        functions::GetPlayerScore(self)
    }

    /// Returns the class of the players skin.
    pub fn get_skin(&self) -> isize {
        functions::GetPlayerSkin(self)
    }

    /// Set the colour of a player's nametag and marker (radar blip).
    pub fn set_color(&self, colour: Colour) {
        functions::SetPlayerColor(self, colour)
    }

    /// Gets the color of the player's name and radar marker.
    pub fn get_color(&self) -> isize {
        functions::GetPlayerColor(self)
    }

    /// Gets the default colour for the player ID.
    pub fn get_default_colour(&self) -> isize {
        functions::GetDefaultPlayerColour(self)
    }

    /// Checks the player's level of drunkenness.
    pub fn get_drunk_level(&self) -> isize {
        functions::GetPlayerDrunkLevel(self)
    }

    /// Give a player a weapon with a specified amount of ammo.
    pub fn give_weapon(&self, data: WeaponSlotData) {
        functions::GivePlayerWeapon(self, data)
    }

    /// Remove a specified weapon from a player.
    pub fn remove_weapon(&self, weaponid: u8) {
        functions::RemovePlayerWeapon(self, weaponid)
    }

    /// Retrieves the amount of money a player has.
    pub fn get_money(&self) -> isize {
        functions::GetPlayerMoney(self)
    }

    /// Reset a player's money to $0.
    pub fn reset_money(&self) {
        functions::ResetPlayerMoney(self)
    }

    /// Sets the name of a player.
    pub fn set_name(&self, name: &str) -> PlayerNameStatus {
        functions::SetPlayerName(self, name)
    }

    /// Get a player's current state.
    pub fn get_state(&self) -> PlayerState {
        functions::GetPlayerState(self)
    }

    /// Get the ping of a player.
    pub fn get_ping(&self) -> isize {
        functions::GetPlayerPing(self)
    }

    /// Returns the ID of the weapon a player is currently holding.
    pub fn get_weapon(&self) -> PlayerWeapon {
        functions::GetPlayerWeapon(self)
    }

    /// Sets the game time for a player.
    pub fn set_time(&self, hour: isize, minute: isize) {
        functions::SetPlayerTime(self, hour, minute)
    }

    /// Get the player's current game time.
    pub fn get_time(&self) -> (isize, isize) {
        let mut hour = 0;
        let mut minute = 0;
        functions::GetPlayerTime(self, &mut hour, &mut minute);
        (hour, minute)
    }

    /// Toggle the in-game clock (top-right corner) for a specific player.
    pub fn toggle_clock(&self, enable: bool) {
        functions::TogglePlayerClock(self, enable)
    }

    /// Checks whether the player has their in-game clock enabled.
    pub fn has_clock_enabled(&self) -> bool {
        functions::PlayerHasClockEnabled(self)
    }

    /// Forces a player to go back to class selection.
    pub fn force_class_selection(&self) {
        functions::ForceClassSelection(self)
    }

    /// Gets the wanted level of a player.
    pub fn get_wanted_level(&self) -> isize {
        functions::GetPlayerWantedLevel(self)
    }

    /// Set a player's special fighting style.
    pub fn set_fighting_style(&self, style: PlayerFightingStyle) {
        functions::SetPlayerFightingStyle(self, style)
    }

    /// Get the fighting style the player currently using.
    pub fn get_fighting_style(&self) -> PlayerFightingStyle {
        functions::GetPlayerFightingStyle(self)
    }

    /// Set a player's velocity on the X, Y and Z axes.
    pub fn set_velocity(&self, pos: Vector3) {
        functions::SetPlayerVelocity(self, pos)
    }

    /// Get the velocity (speed) of a player on the X, Y and Z axes.
    pub fn get_velocity(&self) -> Vector3 {
        let mut pos = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        functions::GetPlayerVelocity(self, &mut pos);
        pos
    }

    /// Get the position of the player's camera.
    pub fn get_camera_pos(&self) -> Vector3 {
        let mut pos = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        functions::GetPlayerCameraPos(self, &mut pos);
        pos
    }

    /// Calculate the distance between a player and a map coordinate.
    pub fn get_distance_from_point(&self, pos: Vector3) -> f32 {
        functions::GetPlayerDistanceFromPoint(self, pos)
    }

    /// Retrieves the player's current interior.
    pub fn get_interior(&self) -> isize {
        functions::GetPlayerInterior(self)
    }

    /// Set a player's position.
    pub fn set_pos(&self, pos: Vector3) {
        functions::SetPlayerPos(self, pos)
    }

    /// Get the position of a player, represented by X, Y and Z coordinates.
    pub fn get_pos(&self) -> Vector3 {
        let mut pos = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        functions::GetPlayerPos(self, &mut pos);
        pos
    }

    /// Retrieves the current virtual world the player is in.
    pub fn get_virtual_world(&self) -> isize {
        functions::GetPlayerVirtualWorld(self)
    }

    /// Check if a player is an actual player or an NPC.
    pub fn is_npc(&self) -> bool {
        functions::IsPlayerNPC(self)
    }

    /// Checks if a player is streamed in another player's client.
    pub fn is_streamed_in(&self, other: &Player) -> bool {
        functions::IsPlayerStreamedIn(self, other)
    }

    /// Plays the specified sound for a player.
    pub fn play_sound(&self, sound: usize, pos: Vector3) {
        functions::PlayerPlaySound(self, sound, pos)
    }

    /// Makes a player spectate (watch) another player.
    pub fn spectate_player(&self, target: &Player, mode: PlayerSpectateMode) {
        functions::PlayerSpectatePlayer(self, target, mode)
    }

    /// Sets a player to spectate another vehicle.
    pub fn spectate_vehicle(&self, vehicle: &Vehicle, mode: PlayerSpectateMode) {
        functions::PlayerSpectateVehicle(self, vehicle, mode)
    }

    /// Set the virtual world of a player.
    pub fn set_virtual_world(&self, vw: isize) {
        functions::SetPlayerVirtualWorld(self, vw)
    }

    /// Set the world boundaries for a player.
    pub fn set_world_bounds(&self, coords: Vector4) {
        functions::SetPlayerWorldBounds(self, coords)
    }

    /// Reset the player's world boundaries to default world boundaries.
    pub fn clear_world_bounds(&self) {
        functions::ClearPlayerWorldBounds(self)
    }

    /// Get a player's world boundaries.
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

    /// Clears all animations for the given player (it also cancels all current tasks such as jetpacking, parachuting, entering vehicles, driving (removes player out of vehicle), swimming, etc).
    pub fn clear_animations(&self, sync_type: PlayerAnimationSyncType) {
        functions::ClearAnimations(self, sync_type)
    }

    /// Retrieves the start and end (hit) position of the last bullet a player fired.
    pub fn get_last_shot_vectors(&self) -> PlayerBulletData {
        functions::GetPlayerLastShotVectors(self)
    }

    /// Allows you to retrieve the ID of the player the playerid is looking at.
    pub fn get_camera_target_player(&self) -> Option<Player> {
        functions::GetPlayerCameraTargetPlayer(self)
    }

    /// Allows you to retrieve the ID of the actor the player is looking at (in any).
    pub fn get_camera_target_actor(&self) -> Option<Actor> {
        functions::GetPlayerCameraTargetActor(self)
    }

    /// Allows you to retrieve the ID of the object the player is looking at.
    pub fn get_camera_target_object(&self) -> Option<Object> {
        functions::GetPlayerCameraTargetObject(self)
    }

    /// Get the ID of the vehicle the player is looking at.
    pub fn get_camera_target_vehicle(&self) -> Option<Vehicle> {
        functions::GetPlayerCameraTargetVehicle(self)
    }

    /// Puts a player in a vehicle.
    pub fn put_in_vehicle(&self, vehicle: &Vehicle, seat_id: isize) {
        functions::PutPlayerInVehicle(self, vehicle, seat_id)
    }

    /// Removes a standard San Andreas model for a single player within a specified range.
    pub fn remove_building(&self, model: isize, pos: Vector3, radius: f32) {
        functions::RemoveBuildingForPlayer(self, model, pos, radius)
    }

    /// Gets the number of removed buildings for a player.
    pub fn get_buildings_removed(&self) -> isize {
        functions::GetPlayerBuildingsRemoved(self)
    }

    /// Removes/ejects a player from their vehicle.
    pub fn remove_from_vehicle(&self, force: bool) {
        functions::RemovePlayerFromVehicle(self, force)
    }

    /// Removes a map icon that was set earlier for a player using SetPlayerMapIcon.
    pub fn remove_map_icon(&self, icon_id: isize) {
        functions::RemovePlayerMapIcon(self, icon_id)
    }

    /// Place an icon/marker on a player's map.
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

    /// Removes all weapons from a player.
    pub fn reset_weapons(&self) {
        functions::ResetPlayerWeapons(self)
    }

    /// Set the ammo of a player's weapon.
    pub fn set_ammo(&self, data: WeaponSlotData) {
        functions::SetPlayerAmmo(self, data)
    }

    /// Sets which weapon (that a player already has) the player is holding.
    pub fn set_armed_weapon(&self, weapon: PlayerWeapon) {
        functions::SetPlayerArmedWeapon(self, weapon)
    }

    /// Creates a chat bubble above a player's name tag.
    pub fn set_chat_bubble(
        &self,
        text: &str,
        colour: Colour,
        drawdistance: f32,
        expiretime: isize,
    ) {
        functions::SetPlayerChatBubble(self, text, colour, drawdistance, expiretime)
    }

    /// This sets the players position then adjusts the players z-coordinate to the nearest solid ground under the position.
    pub fn set_pos_find_z(&self, pos: Vector3) {
        functions::SetPlayerPosFindZ(self, pos)
    }

    /// Set the skill level of a certain weapon type for a player.
    pub fn set_skill_level(&self, weapon: PlayerWeaponSkill, level: isize) {
        functions::SetPlayerSkillLevel(self, weapon, level)
    }

    /// This function allows to set players special action.
    pub fn set_special_action(&self, action: PlayerSpecialAction) {
        functions::SetPlayerSpecialAction(self, action)
    }

    /// This functions allows you to toggle the drawing of player nametags, healthbars and armor bars which display above their head.
    pub fn show_name_tag(&self, other: &Player, enable: bool) {
        functions::ShowPlayerNameTagForPlayer(self, other, enable)
    }

    /// Toggles whether a player can control their character or not.
    pub fn toggle_controllable(&self, enable: bool) {
        functions::TogglePlayerControllable(self, enable)
    }

    /// Toggle whether a player is in spectator mode or not.
    pub fn toggle_spectating(&self, enable: bool) {
        functions::TogglePlayerSpectating(self, enable)
    }

    /// Apply an animation to a player.
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

    /// Enter edition mode for an attached object.
    pub fn edit_attached_object(&self, index: isize) {
        functions::EditAttachedObject(self, index)
    }

    /// Toggle camera targeting functions for a player.
    pub fn enable_camera_target(&self, enable: bool) {
        functions::EnablePlayerCameraTarget(self, enable)
    }

    /// Toggle stunt bonuses for a player.
    pub fn enable_stunt_bonus(&self, enable: bool) {
        functions::EnableStuntBonusForPlayer(self, enable)
    }

    /// Gets the amount of ammo in a player's current weapon.
    pub fn get_ammo(&self) -> isize {
        functions::GetPlayerAmmo(self)
    }

    /// Returns the index of any running applied animations.
    pub fn get_animation_index(&self) -> isize {
        functions::GetPlayerAnimationIndex(self)
    }

    /// Gets the angle a player is facing.
    pub fn get_facing_angle(&self) -> f32 {
        functions::GetPlayerFacingAngle(self)
    }

    /// Get the specified player's IP address and store it in a string.
    pub fn get_ip(&self) -> String {
        let mut ip = String::new();
        functions::GetPlayerIp(self, &mut ip);
        ip
    }

    /// Retrieves a player's current special action.
    pub fn get_special_action(&self) -> PlayerSpecialAction {
        functions::GetPlayerSpecialAction(self)
    }

    /// This function gets the ID of the vehicle the player is currently in.
    pub fn get_vehicle_id(&self) -> isize {
        functions::GetPlayerVehicleID(self)
    }

    /// Find out which seat a player is in.
    pub fn get_vehicle_seat(&self) -> isize {
        functions::GetPlayerVehicleSeat(self)
    }

    /// Get the weapon and ammo in a specific player's weapon slot (e.
    pub fn get_weapon_data(&self, slot: isize) -> WeaponSlotData {
        let mut weapon: WeaponSlotData = WeaponSlotData {
            ammo: 0,
            id: PlayerWeapon::Fist,
        };
        functions::GetPlayerWeaponData(self, slot, &mut weapon);
        weapon
    }

    /// Check the state of a player's weapon.
    pub fn get_weapon_state(&self) -> isize {
        functions::GetPlayerWeaponState(self)
    }

    /// Move a player's camera from one position to another, within the set time.
    pub fn interpolate_camera_pos(
        &self,
        from: Vector3,
        to: Vector3,
        time: isize,
        cut: PlayerCameraCutType,
    ) {
        functions::InterpolateCameraPos(self, from, to, time, cut)
    }

    /// Interpolate a player's camera's 'look at' point between two coordinates with a set speed.
    pub fn interpolate_camera_look_at(
        &self,
        from: Vector3,
        to: Vector3,
        time: isize,
        cut: PlayerCameraCutType,
    ) -> bool {
        functions::InterpolateCameraLookAt(self, from, to, time, cut)
    }

    /// Check if a player has an object attached in the specified index (slot).
    pub fn is_attached_object_slot_used(&self, index: isize) -> bool {
        functions::IsPlayerAttachedObjectSlotUsed(self, index)
    }

    /// You can use this function to attach the player camera to objects.
    pub fn attach_camera_to_object(&self, object: &Object) {
        functions::AttachCameraToObject(self, object)
    }

    /// Attaches a player's camera to a player-object.
    pub fn attach_camera_to_player_object(&self, object: &PlayerObject) {
        functions::AttachCameraToPlayerObject(self, object)
    }

    pub fn get_aim_data(&self) -> PlayerAimData {
        functions::GetPlayerAimData(self)
    }

    /// Check which keys a player is pressing.
    pub fn get_keys(&self) -> PlayerKeyData {
        let mut keys = 0;
        let mut updown = 0;
        let mut leftright = 0;
        functions::GetPlayerKeys(self, &mut keys, &mut updown, &mut leftright)
    }

    pub fn get_surfing_data(&self) -> PlayerSurfingData {
        functions::GetPlayerSurfingData(self)
    }

    /// Check who a player is aiming at.
    pub fn get_target_player(&self) -> Option<Player> {
        functions::GetPlayerTargetPlayer(self)
    }

    /// Gets id of an actor which is aimed by certain player.
    pub fn get_target_actor(&self) -> Option<Actor> {
        functions::GetPlayerTargetActor(self)
    }

    /// Checks if a player is in a specific vehicle.
    pub fn is_in_vehicle(&self, target_vehicle: &Vehicle) -> bool {
        functions::IsPlayerInVehicle(self, target_vehicle)
    }

    /// Check if a player is inside any vehicle (as a driver or passenger).
    pub fn is_in_any_vehicle(&self) -> bool {
        functions::IsPlayerInAnyVehicle(self)
    }

    /// Checks if a player is in range of a point.
    pub fn is_in_range_of_point(&self, range: f32, coord: Vector3) -> bool {
        functions::IsPlayerInRangeOfPoint(self, range, coord)
    }

    /// This function plays a crime report for a player - just like in single-player when CJ commits a crime.
    pub fn play_crime_report(&self, suspect: &Player, crime: isize) -> bool {
        functions::PlayCrimeReportForPlayer(self, suspect, crime)
    }

    /// Remove an attached object from a player.
    pub fn remove_attached_object(&self, index: isize) {
        functions::RemovePlayerAttachedObject(self, index)
    }

    /// Set a player's facing angle (Z rotation).
    pub fn set_facing_angle(&self, angle: f32) {
        functions::SetPlayerFacingAngle(self, angle)
    }

    /// Change the colour of a player's nametag and radar blip for another player.
    pub fn set_marker_for_player(&self, other: &Player, colour: Colour) {
        functions::SetPlayerMarkerForPlayer(self, other, colour)
    }

    /// Get the colour of a player's nametag and radar blip for another player.
    pub fn get_marker_for_player(&self, other: &Player) -> isize {
        functions::GetPlayerMarkerForPlayer(self, other)
    }

    /// Enable/Disable the teleporting ability for a player by right-clicking on the map.
    pub fn allow_teleport(&self, allow: bool) {
        functions::AllowPlayerTeleport(self, allow)
    }

    /// Can this player teleport by right-clicking on the map?
    pub fn is_teleport_allowed(&self) -> bool {
        functions::IsPlayerTeleportAllowed(self)
    }

    pub fn set_remote_vehicle_collisions(&self, collide: bool) {
        functions::SetRemoteVehicleCollisions(self, collide)
    }

    /// Display the cursor and allow the player to select a textdraw.
    pub fn select_text_draw(&self, hover_colour: Colour) {
        functions::SelectTextDraw(self, hover_colour)
    }

    /// Cancel textdraw selection with the mouse.
    pub fn cancel_select_text_draw(&self) {
        functions::CancelSelectTextDraw(self)
    }

    /// Perform a memory check on the client.
    pub fn send_client_check(
        &self,
        action_type: isize,
        address: isize,
        offset: isize,
        count: isize,
    ) {
        functions::SendClientCheck(self, action_type, address, offset, count)
    }

    /// (Re)Spawns a player.
    pub fn spawn(&self) {
        functions::SpawnPlayer(self)
    }

    /// Fetch the CI (computer/client identification) of a user, this is linked to their SAMP/GTA on their computer.
    pub fn gpci(&self) -> String {
        let mut output = String::new();
        functions::gpci(self, &mut output);
        output
    }

    /// Check if a player is logged in as an RCON admin.
    pub fn is_admin(&self) -> bool {
        functions::IsPlayerAdmin(self)
    }
    /// Kicks a player from the server. They will have to quit the game and re-connect if they wish to continue playing.
    pub fn kick(&self) {
        functions::Kick(self)
    }

    /// Shows 'game text' (on-screen text) for a certain length of time for a specific player.
    pub fn game_text(&self, msg: &str, time: isize, style: isize) {
        functions::GameTextForPlayer(self, msg, time, style)
    }

    /// Stop showing a gametext style to a player.
    pub fn hide_game_text(&self, style: isize) {
        functions::HideGameTextForPlayer(self, style)
    }

    /// Does the player currently have text in the given gametext style displayed?
    pub fn has_game_text(&self, style: isize) -> bool {
        functions::HasGameText(self, style)
    }

    /// Returns all the information on the given game text style.
    pub fn get_game_text(
        &self,
        style: isize,
        message: &mut String,
        time: &mut isize,
        remaining: &mut isize,
    ) -> bool {
        functions::GetGameText(self, style, message, time, remaining)
    }

    /// Ban a player who is currently in the server.
    pub fn ban(&self) {
        functions::Ban(self)
    }

    /// Ban a player with a reason.
    pub fn ban_ex(&self, msg: &str) {
        functions::BanEx(self, msg)
    }

    /// Adds a death to the 'killfeed' on the right-hand side of the screen for a single player.
    pub fn send_death_message_to_player(
        &self,
        killer: &Player,
        killee: &Player,
        weapon: PlayerWeapon,
    ) {
        functions::SendDeathMessageToPlayer(self, killer, killee, weapon)
    }

    /// Sends a message in the name of a player to another player on the server.
    pub fn send_message_to_player(&self, sender: &Player, message: &str) {
        functions::SendPlayerMessageToPlayer(self, sender, message)
    }

    /// Returns the SA-MP client version, as reported by the player.
    pub fn get_version(&self, output: &mut String) {
        functions::GetPlayerVersion(self, output)
    }

    /// Get the player skill level of a certain weapon type.
    pub fn get_skill_level(&self, skill: isize) -> isize {
        functions::GetPlayerSkillLevel(self, skill)
    }

    /// Gets the ID of the player or vehicle the player is spectating (watching).
    pub fn get_spectate_id(&self) -> isize {
        functions::GetPlayerSpectateID(self)
    }

    pub fn get_spectate_data(&self) -> PlayerSpectateData {
        functions::GetPlayerSpectateData(self)
    }

    /// Get the specified player's Raw IP address (v4).
    pub fn get_raw_ip(&self) -> isize {
        functions::GetPlayerRawIp(self)
    }

    /// Set a player's gravity.
    pub fn set_gravity(&self, gravity: f32) {
        functions::SetPlayerGravity(self, gravity)
    }

    /// Get a player's gravity.
    pub fn get_gravity(&self) -> f32 {
        functions::GetPlayerGravity(self)
    }

    /// Sets the player as an RCON admin.
    pub fn set_admin(&self, set: bool) {
        functions::SetPlayerAdmin(self, set)
    }

    /// Checks if a player is spawned.
    pub fn is_spawned(&self) -> bool {
        functions::IsPlayerSpawned(self)
    }

    /// Check if the player is controllable.
    pub fn is_controllable(&self) -> bool {
        functions::IsPlayerControllable(self)
    }

    /// Check if the player camera target is enabled.
    pub fn is_camera_target_enabled(&self) -> bool {
        functions::IsPlayerCameraTargetEnabled(self)
    }

    /// Toggle player's ghost mode.
    pub fn toggle_ghost_mode(&self, toggle: bool) {
        functions::TogglePlayerGhostMode(self, toggle)
    }

    /// Get player's ghost mode.
    pub fn get_ghost_mode(&self) -> bool {
        functions::GetPlayerGhostMode(self)
    }

    /// Enable/Disable weapons for a player.
    pub fn allow_weapons(&self, allow: bool) {
        functions::AllowPlayerWeapons(self, allow)
    }

    /// Can the player use weapons?
    pub fn are_weapons_allowed(&self) -> bool {
        functions::ArePlayerWeaponsAllowed(self)
    }

    /// Check if the player is using the official SA-MP client.
    pub fn is_using_official_client(&self) -> bool {
        functions::IsPlayerUsingOfficialClient(self)
    }

    pub fn get_animation_data(&self) -> PlayerAnimationData {
        functions::GetPlayerAnimationData(self)
    }

    /// Check if the player is in driveby mode.
    pub fn is_in_drive_by_mode(&self) -> bool {
        functions::IsPlayerInDriveByMode(self)
    }

    /// Checks if the player special action is cuffed.
    pub fn is_cuffed(&self) -> bool {
        functions::IsPlayerCuffed(self)
    }

    /// Returns the class of the players custom skin downloaded from the server.
    pub fn get_custom_skin(&self) -> isize {
        functions::GetPlayerCustomSkin(self)
    }

    /// Redirect a player custom AddCharModel or AddSimpleModel download to a specific HTTP webpage.
    pub fn redirect_download(&self, url: &str) -> bool {
        functions::RedirectDownload(self, url)
    }

    // Player Checkpoints methods
    /// Sets a checkpoint (red cylinder) for a player.
    pub fn set_player_checkpoint(&self, centre_position: Vector3, radius: f32) {
        checkpoints::functions::SetPlayerCheckpoint(self, centre_position, radius);
    }

    /// Disables (hides/destroys) a player's set checkpoint.
    pub fn disable_player_checkpoint(&self) {
        checkpoints::functions::DisablePlayerCheckpoint(self);
    }

    /// Check if the player is currently inside a checkpoint, this could be used for properties or teleport points for example.
    pub fn is_player_in_checkpoint(&self) -> bool {
        checkpoints::functions::IsPlayerInCheckpoint(self)
    }

    /// Creates a race checkpoint.
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

    /// Disable any initialized race checkpoints for a specific player, since you can only have one at any given time.
    pub fn disable_player_race_checkpoint(&self) {
        checkpoints::functions::DisablePlayerRaceCheckpoint(self)
    }

    /// Check if the player is inside their current set race checkpoint (SetPlayerRaceCheckpoint).
    pub fn is_player_in_race_checkpoint(&self) -> bool {
        checkpoints::functions::IsPlayerInRaceCheckpoint(self)
    }

    /// Check if the player currently has a checkpoint visible.
    pub fn is_player_checkpoint_active(&self) -> bool {
        checkpoints::functions::IsPlayerCheckpointActive(self)
    }

    /// Get the location of the current checkpoint.
    pub fn get_player_checkpoint(&self) -> PlayerCheckPointData {
        let mut center_pos = Vector3::default();
        let mut radius = 0.0;
        checkpoints::functions::GetPlayerCheckpoint(self, &mut center_pos, &mut radius);
        PlayerCheckPointData::new(center_pos, radius)
    }

    /// Check if the player currently has a race checkpoint visible.
    pub fn is_player_race_checkpoint_active(&self) -> bool {
        checkpoints::functions::IsPlayerRaceCheckpointActive(self)
    }

    /// Get the location of the current race checkpoint.
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

    /// This function can be used to change the spawn information of a specific player.
    pub fn set_spawn_info(&self, player_class: PlayerClass) {
        classes::functions::SetSpawnInfo(self, player_class)
    }

    /// Return the current spawn data for a player, where they will spawn next.
    pub fn get_spawn_info(&self) -> PlayerClass {
        let mut data = PlayerClass::default();
        classes::functions::GetSpawnInfo(self, &mut data);
        data
    }

    /// Shows the player a synchronous (only one at a time) dialog box.
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

    /// Get the ID of the dialog currently show to the player.
    pub fn get_dialog_id(&self) -> i16 {
        dialogs::functions::GetPlayerDialogID(self)
    }

    /// Hides any dialog the player may currently be able to see.
    pub fn hide_dialog(&self) -> bool {
        dialogs::functions::HidePlayerDialog(self)
    }

    pub fn get_id(&self) -> usize {
        functions::GetPlayerID(self)
    }
    pub fn from_id(playerid: isize) -> Option<Player> {
        functions::GetPlayerFromID(playerid)
    }
    /// Gets the ID of the menu the player is currently viewing (shown by ShowMenuForPlayer).
    pub fn get_menu(&self) -> Option<Menu> {
        menus::functions::GetPlayerMenu(self)
    }

    /// Allows a player to edit an object (position and rotation) using their mouse on a GUI (Graphical User Interface).
    pub fn edit_object(&self, object: &Object) {
        objects::functions::EditObject(self, object)
    }
    /// Display the cursor and allow the player to select an object.
    pub fn select_object(&self) {
        objects::functions::SelectObject(self)
    }
    /// Cancel object edition mode for a player.
    pub fn end_object_editing(&self) {
        objects::functions::EndObjectEditing(self)
    }
    /// Creates an object which will be visible to only one player.
    pub fn create_player_object(
        &self,
        modelid: isize,
        position: Vector3,
        rotation: Vector3,
        drawDistance: f32,
    ) -> Option<PlayerObject> {
        objects::functions::CreatePlayerObject(self, modelid, position, rotation, drawDistance)
    }
    /// Destroy a player-object created using CreatePlayerObject.
    pub fn destroy_player_object(&self, object: PlayerObject) {
        objects::functions::DestroyPlayerObject(self, &object);
    }
    /// Allows players to edit a player-object (position and rotation) with a GUI and their mouse.
    pub fn edit_player_object(&self, object: &PlayerObject) {
        objects::functions::EditPlayerObject(self, object)
    }

    /// Get PlayerObject from an id
    pub fn get_player_object_from_id(&self, id: isize) -> Option<PlayerObject> {
        objects::functions::GetPlayerObjectFromID(self, id)
    }

    /// Creates a textdraw for a single player.
    pub fn create_player_text_draw(&self, position: Vector2, text: &str) -> Option<PlayerTextDraw> {
        textdraws::functions::CreatePlayerTextDraw(self, position, text)
    }
    /// Destroy a player-textdraw.
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
    /// Creates a 3D Text Label only for a specific player.
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
    /// Destroy a 3D text label that was created using CreatePlayer3DTextLabel.
    pub fn delete_player_text_label(&self, textlabel: PlayerTextLabel) {
        textlabels::functions::DeletePlayer3DTextLabel(self, &textlabel)
    }

    /// Check if the player is in the mod shop.
    pub fn is_player_in_mod_shop(&self) -> bool {
        vehicles::functions::IsPlayerInModShop(self)
    }
    /// Gets the siren state of the player's vehicle.
    pub fn get_player_siren_state(&self) -> isize {
        vehicles::functions::GetPlayerSirenState(self)
    }
    /// Gets the landing gear state of the current player's vehicle.
    pub fn get_player_landing_gear_state(&self) -> isize {
        vehicles::functions::GetPlayerLandingGearState(self)
    }
    /// Gets the hydra reactor angle of the player's vehicle.
    pub fn get_player_hydra_reactor_angle(&self) -> isize {
        vehicles::functions::GetPlayerHydraReactorAngle(self)
    }
    /// Gets the speed of the player's train.
    pub fn get_player_train_speed(&self) -> f32 {
        vehicles::functions::GetPlayerTrainSpeed(self)
    }

    /// Gets the amount of data (in bytes) that the server has received from the player.
    pub fn net_stats__bytes_received(&self) -> isize {
        functions::NetStats_BytesReceived(self)
    }
    /// Gets the amount of data (in bytes) that the server has sent to the player.
    pub fn net_stats__bytes_sent(&self) -> isize {
        functions::NetStats_BytesSent(self)
    }
    /// Gets the player's current connection status.
    pub fn net_stats__connection_status(&self) -> isize {
        functions::NetStats_ConnectionStatus(self)
    }
    /// Gets the amount of time (in milliseconds) that a player has been connected to the server for.
    pub fn net_stats__get_connected_time(&self) -> isize {
        functions::NetStats_GetConnectedTime(self)
    }
    /// Get a player's IP and port.
    pub fn net_stats__get_ip_port(&self) -> String {
        let mut output = String::new();
        functions::NetStats_GetIpPort(self, &mut output);
        output
    }
    /// Gets the number of messages the server has received from the player.
    pub fn net_stats__messages_received(&self) -> isize {
        functions::NetStats_MessagesReceived(self)
    }
    /// Gets the number of messages the player has received in the last second.
    pub fn net_stats__messages_recv_per_second(&self) -> isize {
        functions::NetStats_MessagesRecvPerSecond(self)
    }
    /// Gets the number of messages the server has sent to the player.
    pub fn net_stats__messages_sent(&self) -> isize {
        functions::NetStats_MessagesSent(self)
    }
    /// Gets the packet loss percentage of a player.
    pub fn net_stats__packet_loss_percent(&self) -> f32 {
        functions::NetStats_PacketLossPercent(self)
    }
    /// Sends a message in the name of a player to all other players on the server.
    pub fn send_message_to_all(&self, message: &str) {
        functions::SendPlayerMessageToAll(self, message)
    }

    /// Attach an object to a specific bone on a player.
    pub fn set_attached_object(&self, index: isize, attachment: ObjectAttachmentSlotData) {
        functions::SetPlayerAttachedObject(self, index, attachment)
    }
    /// Gets the player attachment object data by index.
    pub fn get_attached_object(&self, index: isize) -> ObjectAttachmentSlotData {
        functions::GetPlayerAttachedObject(self, index)
    }
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum MapIconStyle {
    Local,
    Global,
    LocalCheckpoint,
    GlobalCheckpoint,
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ClientVersion {
    Samp037,
    Samp03dl,
    Openmp,
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PlayerCameraCutType {
    Cut,
    Move,
}

/// The player's name status returned when updating their name
#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PlayerNameStatus {
    /// The name has successfully been updated
    Updated,
    /// The name is already taken by another player
    Taken,
    /// The name is invalid
    Invalid,
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PlayerAnimationSyncType {
    NoSync,
    Sync,
    SyncOthers,
}

#[repr(C)]
#[derive(Default, Clone, Copy, Debug)]
pub struct WeaponSlotData {
    id: PlayerWeapon,
    ammo: u32,
}

impl WeaponSlotData {
    pub fn new(id: PlayerWeapon, ammo: u32) -> Self {
        Self { id, ammo }
    }
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct PlayerAnimationData {
    ID: u16,
    flags: u16,
}
#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PlayerFightingStyle {
    Normal = 4,
    Boxing = 5,
    KungFu = 6,
    KneeHead = 7,
    GrabKick = 15,
    Elbow = 16,
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PlayerState {
    None = 0,
    OnFoot = 1,
    Driver = 2,
    Passenger = 3,
    ExitVehicle = 4,
    EnterVehicleDriver = 5,
    EnterVehiclePassenger = 6,
    Wasted = 7,
    Spawned = 8,
    Spectating = 9,
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PlayerWeaponSkill {
    Pistol,
    SilencedPistol,
    DesertEagle,
    Shotgun,
    SawnOff,
    Spas12,
    Uzi,
    Mp5,
    Ak47,
    M4,
    Sniper,
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PlayerSpecialAction {
    None,
    Duck,
    Jetpack,
    EnterVehicle,
    ExitVehicle,
    Dance1,
    Dance2,
    Dance3,
    Dance4,
    HandsUp = 10,
    Cellphone,
    Sitting,
    StopCellphone,
    Beer = 20,
    Smoke,
    Wine,
    Sprunk,
    Cuffed,
    Carry,
    Pissing = 68,
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct PlayerSurfingData {
    surftype: isize,
    ID: isize,
    offset: Vector3,
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct PlayerKeyData {
    pub keys: u32,
    pub upDown: i16,
    pub leftRight: i16,
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct PlayerBulletData {
    origin: Vector3,
    hitPos: Vector3,
    offset: Vector3,
    weapon: PlayerWeapon,
    hitType: PlayerBulletHitType,
    hitID: u16,
}
#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PlayerBulletHitType {
    None,
    Player = 1,
    Vehicle = 2,
    Object = 3,
    PlayerObject = 4,
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum SpectateType {
    None,
    Vehicle,
    Player,
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct PlayerSpectateData {
    spectating: bool,
    spectateID: isize,
    spectate_type: SpectateType,
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PlayerSpectateMode {
    Normal = 1,
    Fixed,
    Side,
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
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
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PlayerWeaponState {
    Unknown = -1,
    NoBullets,
    LastBullet,
    MoreBullets,
    Reloading,
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum BodyPart {
    Torso = 3,
    Groin,
    LeftArm,
    RightArm,
    LeftLeg,
    RightLeg,
    Head,
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PlayerClickSource {
    Scoreboard,
}

pub type WeaponSlots = StaticArray<WeaponSlotData, 13>;

#[repr(u8)]
#[derive(PartialEq, Copy, Clone, Default, Debug)]
pub enum PlayerWeapon {
    #[default]
    Fist,
    BrassKnuckle,
    GolfClub,
    NiteStick,
    Knife,
    Bat,
    Shovel,
    PoolStick,
    Katana,
    Chainsaw,
    Dildo,
    Dildo2,
    Vibrator,
    Vibrator2,
    Flower,
    Cane,
    Grenade,
    Teargas,
    Moltov,
    Colt45 = 22,
    Silenced,
    Deagle,
    Shotgun,
    Sawedoff,
    Shotgspa,
    UZI,
    MP5,
    AK47,
    M4,
    TEC9,
    Rifle,
    Sniper,
    RocketLauncher,
    HeatSeeker,
    FlameThrower,
    Minigun,
    Satchel,
    Bomb,
    SprayCan,
    FireExtinguisher,
    Camera,
    NightVisGoggles,
    ThermalGoggles,
    Parachute,
    Vehicle = 49,
    Heliblades,
    Explosion,
    Drown = 53,
    Collision,
    End,
    Connect = 200,
    Disconnect,
    Suicide = 255,
}

// Player Keys
pub mod PlayerKeys {
    pub const ACTION: u32 = 1;
    pub const CROUCH: u32 = 2;
    pub const FIRE: u32 = 4;
    pub const SPRINT: u32 = 8;
    pub const SECONDARY_ATTACK: u32 = 16;
    pub const JUMP: u32 = 32;
    pub const LOOK_RIGHT: u32 = 64;
    pub const HANDBRAKE: u32 = 128;
    pub const LOOK_LEFT: u32 = 256;
    pub const SUBMISSION: u32 = 512;
    pub const LOOK_BEHIND: u32 = 512;
    pub const WALK: u32 = 1024;
    pub const ANALOG_UP: u32 = 2048;
    pub const ANALOG_DOWN: u32 = 4096;
    pub const ANALOG_LEFT: u32 = 8192;
    pub const ANALOG_RIGHT: u32 = 16384;
    pub const YES: u32 = 65536;
    pub const NO: u32 = 131072;
    pub const CTRL_BACK: u32 = 262144;
    pub const UP: isize = -128;
    pub const DOWN: isize = 128;
    pub const LEFT: isize = -128;
    pub const RIGHT: isize = 128;
}
