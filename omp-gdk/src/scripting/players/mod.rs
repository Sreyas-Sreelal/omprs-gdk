pub mod events;
pub mod functions;

pub use functions::load_functions;

use crate::{
    actors::Actor,
    classes::PlayerClass,
    dialogs::{self, DialogStyle},
    models,
    objects::{self, Object, ObjectAttachmentSlotData, PlayerObject},
    textdraws::{self, PlayerTextDraw},
    types::{
        animationdata::AnimationData,
        colour::Colour,
        staticarray::StaticArray,
        vector::{Vector2, Vector3, Vector4},
    },
    vehicles::Vehicle,
};
use std::{mem::transmute, os::raw::c_void};

use super::{
    checkpoints::{self, PlayerCheckPointData, PlayerRaceCheckPointData, RaceCheckpointType},
    menus::Menu,
    textlabels::{self, PlayerTextLabel},
};

#[derive(Clone, Copy)]
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
    pub fn send_client_message(&self, colour: Colour, message: &str) -> bool {
        functions::Player_SendClientMessage(self, colour.rgba(), message)
    }

    /// Get a player's name.
    pub fn get_name(&self) -> String {
        let mut name = String::new();
        functions::Player_GetName(self, &mut name);
        name
    }

    /// Sets the camera to a specific position for a player.
    pub fn set_camera_pos(&self, pos: Vector3) -> bool {
        functions::Player_SetCameraPos(self, pos.x, pos.y, pos.z)
    }

    /// Sets the drunk level of a player which makes the player's camera sway and vehicles hard to control.
    pub fn set_drunk_level(&self, level: i32) -> bool {
        functions::Player_SetDrunkLevel(self, level)
    }

    /// Set a player's interior.
    pub fn set_interior(&self, interiorid: i32) -> bool {
        functions::Player_SetInterior(self, interiorid)
    }

    /// Set a player's wanted level (6 brown stars under HUD).
    pub fn set_wanted_level(&self, level: i32) -> bool {
        functions::Player_SetWantedLevel(self, level)
    }

    /// Set a player's weather.
    pub fn set_weather(&self, weatherid: i32) -> bool {
        functions::Player_SetWeather(self, weatherid)
    }

    /// Get a player's weather.
    pub fn get_weather(&self) -> i32 {
        functions::Player_GetWeather(self)
    }

    /// Set the skin of a player.
    pub fn set_skin(&self, skinid: i32) -> bool {
        functions::Player_SetSkin(self, skinid)
    }

    /// Loads or unloads an interior script for a player (for example the ammunation menu).
    pub fn set_shop_name(&self, shopname: &str) -> bool {
        functions::Player_SetShopName(self, shopname)
    }

    /// Give money to or take money from a player.
    pub fn give_money(&self, amount: i32) -> bool {
        functions::Player_GiveMoney(self, amount)
    }

    /// Set the direction a player's camera looks at.
    pub fn set_camera_look_at(&self, pos: Vector3, cut: PlayerCameraCutType) -> bool {
        functions::Player_SetCameraLookAt(self, pos.x, pos.y, pos.z, cut as i32)
    }

    /// Restore the camera to a place behind the player, after using a function like SetPlayerCameraPos.
    pub fn set_camera_behind_player(&self) -> bool {
        functions::Player_SetCameraBehind(self)
    }

    /// Creates an explosion that is only visible to a single player.
    pub fn create_explosion(&self, pos: Vector3, explosion_type: i32, radius: f32) -> bool {
        functions::Player_CreateExplosion(self, pos.x, pos.y, pos.z, explosion_type, radius)
    }

    /// Play an 'audio stream' for a player.
    pub fn play_audio_stream(&self, url: &str, pos: Vector3, distance: f32, use_pos: bool) -> bool {
        functions::Player_PlayAudioStream(self, url, pos.x, pos.y, pos.z, distance, use_pos)
    }

    /// Stops the current audio stream for a player.
    pub fn stop_audio_stream(&self) -> bool {
        functions::Player_StopAudioStream(self)
    }

    /// Adds a death to the 'killfeed' on the right-hand side of the screen for all players.
    pub fn send_death_message(
        &self,
        killer: &Player,
        killee: &Player,
        weapon: PlayerWeapon,
    ) -> bool {
        functions::Player_SendDeathMessage(self, killer, killee, weapon as i32)
    }

    /// Toggle player's widescreen.
    pub fn toggle_widescreen(&self, enable: bool) -> bool {
        functions::Player_ToggleWidescreen(self, enable)
    }

    /// Checks if a player widescreen is on or off.
    pub fn is_widescreen_toggled(&self) -> bool {
        functions::Player_IsWidescreenToggled(self)
    }

    /// Set the health of a player.
    pub fn set_health(&self, health: f32) -> bool {
        functions::Player_SetHealth(self, health)
    }

    /// The function GetPlayerHealth allows you to retrieve the health of a player.
    pub fn get_health(&self) -> f32 {
        functions::Player_GetHealth(self)
    }

    /// Set a player's armor level.
    pub fn set_armour(&self, armour: f32) -> bool {
        functions::Player_SetArmor(self, armour)
    }

    /// This function stores the armour of a player into a variable.
    pub fn get_armour(&self) -> f32 {
        functions::Player_GetArmor(self)
    }

    /// Set the team of a player.
    pub fn set_team(&self, teamid: i32) -> bool {
        functions::Player_SetTeam(self, teamid)
    }

    /// Get the ID of the team the player is on.
    pub fn get_team(&self) -> i32 {
        functions::Player_GetTeam(self)
    }

    /// Set a player's score.
    pub fn set_score(&self, score: i32) -> bool {
        functions::Player_SetScore(self, score)
    }

    /// This function returns a player's score as it was set using SetPlayerScore.
    pub fn get_score(&self) -> i32 {
        functions::Player_GetScore(self)
    }

    /// Returns the class of the players skin.
    pub fn get_skin(&self) -> i32 {
        functions::Player_GetSkin(self)
    }

    /// Set the colour of a player's nametag and marker (radar blip).
    pub fn set_color(&self, colour: Colour) -> bool {
        functions::Player_SetColor(self, colour.rgba())
    }

    /// Gets the color of the player's name and radar marker.
    pub fn get_color(&self) -> Colour {
        Colour::from_rgba(functions::Player_GetColor(self))
    }

    /// Gets the default colour for the player ID.
    pub fn get_default_colour(&self) -> Colour {
        Colour::from_rgba(functions::Player_GetDefaultColor(self))
    }

    /// Checks the player's level of drunkenness.
    pub fn get_drunk_level(&self) -> i32 {
        functions::Player_GetDrunkLevel(self)
    }

    /// Give a player a weapon with a specified amount of ammo.
    pub fn give_weapon(&self, data: WeaponSlotData) -> bool {
        functions::Player_GiveWeapon(self, data.id as i32, data.ammo as i32)
    }

    /// Remove a specified weapon from a player.
    pub fn remove_weapon(&self, weaponid: u8) -> bool {
        functions::Player_RemoveWeapon(self, weaponid as i32)
    }

    /// Retrieves the amount of money a player has.
    pub fn get_money(&self) -> i32 {
        functions::Player_GetMoney(self)
    }

    /// Reset a player's money to $0.
    pub fn reset_money(&self) -> bool {
        functions::Player_ResetMoney(self)
    }

    /// Sets the name of a player.
    pub fn set_name(&self, name: &str) -> PlayerNameStatus {
        PlayerNameStatus::from(functions::Player_SetName(self, name))
    }

    /// Get a player's current state.
    pub fn get_state(&self) -> PlayerState {
        PlayerState::from(functions::Player_GetState(self))
    }

    /// Get the ping of a player.
    pub fn get_ping(&self) -> i32 {
        functions::Player_GetPing(self)
    }

    /// Returns the ID of the weapon a player is currently holding.
    pub fn get_weapon(&self) -> PlayerWeapon {
        // May be i should stop  being lazy and implement from trait..maybe
        unsafe { transmute(functions::Player_GetWeapon(self) as u8) }
    }

    /// Sets the game time for a player.
    pub fn set_time(&self, hour: i32, minute: i32) -> bool {
        functions::Player_SetTime(self, hour, minute)
    }

    /// Get the player's current game time.
    pub fn get_time(&self) -> (i32, i32) {
        let mut hour = 0;
        let mut minute = 0;
        functions::Player_GetTime(self, &mut hour, &mut minute);
        (hour, minute)
    }

    /// Toggle the in-game clock (top-right corner) for a specific player.
    pub fn toggle_clock(&self, enable: bool) -> bool {
        functions::Player_ToggleClock(self, enable)
    }

    /// Checks whether the player has their in-game clock enabled.
    pub fn has_clock_enabled(&self) -> bool {
        functions::Player_HasClock(self)
    }

    /// Forces a player to go back to class selection.
    pub fn force_class_selection(&self) -> bool {
        functions::Player_ForceClassSelection(self)
    }

    /// Gets the wanted level of a player.
    pub fn get_wanted_level(&self) -> i32 {
        functions::Player_GetWantedLevel(self)
    }

    /// Set a player's special fighting style.
    pub fn set_fighting_style(&self, style: PlayerFightingStyle) -> bool {
        functions::Player_SetFightingStyle(self, style as i32)
    }

    /// Get the fighting style the player currently using.
    pub fn get_fighting_style(&self) -> PlayerFightingStyle {
        unsafe { transmute(functions::Player_GetFightingStyle(self)) }
    }

    /// Set a player's velocity on the X, Y and Z axes.
    pub fn set_velocity(&self, pos: Vector3) -> bool {
        functions::Player_SetVelocity(self, pos.x, pos.y, pos.z)
    }

    /// Get the velocity (speed) of a player on the X, Y and Z axes.
    pub fn get_velocity(&self) -> Vector3 {
        let mut pos = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        functions::Player_GetVelocity(self, &mut pos.x, &mut pos.y, &mut pos.z);
        pos
    }

    /// Get the position of the player's camera.
    pub fn get_camera_pos(&self) -> Vector3 {
        let mut pos = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        functions::Player_GetCameraPos(self, &mut pos.x, &mut pos.y, &mut pos.z);
        pos
    }

    /// Calculate the distance between a player and a map coordinate.
    pub fn get_distance_from_point(&self, pos: Vector3) -> f32 {
        functions::Player_GetDistanceFromPoint(self, pos.x, pos.y, pos.z)
    }

    /// Retrieves the player's current interior.
    pub fn get_interior(&self) -> i32 {
        functions::Player_GetInterior(self)
    }

    /// Set a player's position.
    pub fn set_pos(&self, pos: Vector3) -> bool {
        functions::Player_SetPos(self, pos.x, pos.y, pos.z)
    }

    /// Get the position of a player, represented by X, Y and Z coordinates.
    pub fn get_pos(&self) -> Vector3 {
        let mut pos = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        functions::Player_GetPos(self, &mut pos.x, &mut pos.y, &mut pos.z);
        pos
    }

    /// Retrieves the current virtual world the player is in.
    pub fn get_virtual_world(&self) -> i32 {
        functions::Player_GetVirtualWorld(self)
    }

    /// Check if a player is an actual player or an NPC.
    pub fn is_npc(&self) -> bool {
        functions::Player_IsNPC(self)
    }

    /// Checks if a player is streamed in another player's client.
    pub fn is_streamed_in(&self, other: &Player) -> bool {
        functions::Player_IsStreamedIn(self, other)
    }

    /// Plays the specified sound for a player.
    pub fn play_sound(&self, sound: usize, pos: Vector3) -> bool {
        functions::Player_PlayGameSound(self, sound as i32, pos.x, pos.y, pos.z)
    }

    /// Makes a player spectate (watch) another player.
    pub fn spectate_player(&self, target: &Player, mode: PlayerSpectateMode) -> bool {
        functions::Player_SpectatePlayer(self, target, mode as i32)
    }

    /// Sets a player to spectate another vehicle.
    pub fn spectate_vehicle(&self, vehicle: &Vehicle, mode: PlayerSpectateMode) -> bool {
        functions::Player_SpectateVehicle(self, vehicle, mode as i32)
    }

    /// Set the virtual world of a player.
    pub fn set_virtual_world(&self, vw: i32) -> bool {
        functions::Player_SetVirtualWorld(self, vw)
    }

    /// Set the world boundaries for a player.
    pub fn set_world_bounds(&self, coords: Vector4) -> bool {
        functions::Player_SetWorldBounds(self, coords.x, coords.y, coords.z, coords.w)
    }

    /// Reset the player's world boundaries to default world boundaries.
    pub fn clear_world_bounds(&self) -> bool {
        functions::Player_ClearWorldBounds(self)
    }

    /// Get a player's world boundaries.
    pub fn get_world_bounds(&self) -> Vector4 {
        let mut pos = Vector4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };
        functions::Player_GetWorldBounds(self, &mut pos.x, &mut pos.y, &mut pos.z, &mut pos.w);
        pos
    }

    /// Clears all animations for the given player (it also cancels all current tasks such as jetpacking, parachuting, entering vehicles, driving (removes player out of vehicle), swimming, etc).
    pub fn clear_animations(&self, sync_type: PlayerAnimationSyncType) -> bool {
        functions::Player_ClearAnimations(self, sync_type as i32)
    }

    /// Retrieves the start and end (hit) position of the last bullet a player fired.
    pub fn get_last_shot_vectors(&self) -> PlayerBulletData {
        let mut data = PlayerBulletData::default();
        functions::Player_GetLastShotVectors(
            self,
            &mut data.origin.x,
            &mut data.origin.y,
            &mut data.origin.z,
            &mut data.hitPos.x,
            &mut data.hitPos.y,
            &mut data.hitPos.z,
        );
        data
    }

    /// Allows you to retrieve the ID of the player the playerid is looking at.
    pub fn get_camera_target_player(&self) -> Option<Player> {
        functions::Player_GetCameraTargetPlayer(self)
    }

    /// Allows you to retrieve the ID of the actor the player is looking at (in any).
    pub fn get_camera_target_actor(&self) -> Option<Actor> {
        functions::Player_GetCameraTargetActor(self)
    }

    /// Allows you to retrieve the ID of the object the player is looking at.
    pub fn get_camera_target_object(&self) -> Option<Object> {
        functions::Player_GetCameraTargetObject(self)
    }

    /// Get the ID of the vehicle the player is looking at.
    pub fn get_camera_target_vehicle(&self) -> Option<Vehicle> {
        functions::Player_GetCameraTargetVehicle(self)
    }

    /// Puts a player in a vehicle.
    pub fn put_in_vehicle(&self, vehicle: &Vehicle, seat_id: i32) -> bool {
        functions::Player_PutInVehicle(self, vehicle, seat_id)
    }

    /// Removes a standard San Andreas model for a single player within a specified range.
    pub fn remove_building(&self, model: i32, pos: Vector3, radius: f32) -> bool {
        functions::Player_RemoveBuilding(self, model, pos.x, pos.y, pos.z, radius)
    }

    /// Gets the number of removed buildings for a player.
    pub fn get_buildings_removed(&self) -> i32 {
        functions::Player_GetBuildingsRemoved(self)
    }

    /// Removes/ejects a player from their vehicle.
    pub fn remove_from_vehicle(&self, force: bool) -> bool {
        functions::Player_RemoveFromVehicle(self, force)
    }

    /// Removes a map icon that was set earlier for a player using SetPlayerMapIcon.
    pub fn remove_map_icon(&self, icon_id: i32) -> bool {
        functions::Player_RemoveMapIcon(self, icon_id)
    }

    /// Place an icon/marker on a player's map.
    pub fn set_map_icon(
        &self,
        icon_id: i32,
        pos: Vector3,
        icon_type: i32,
        colour: Colour,
        style: MapIconStyle,
    ) -> bool {
        functions::Player_SetMapIcon(
            self,
            icon_id,
            pos.x,
            pos.y,
            pos.z,
            icon_type,
            colour.rgba(),
            style as i32,
        )
    }

    /// Removes all weapons from a player.
    pub fn reset_weapons(&self) -> bool {
        functions::Player_ResetWeapons(self)
    }

    /// Set the ammo of a player's weapon.
    pub fn set_ammo(&self, data: WeaponSlotData) -> bool {
        functions::Player_SetAmmo(self, data.id as u8, data.ammo)
    }

    /// Sets which weapon (that a player already has) the player is holding.
    pub fn set_armed_weapon(&self, weapon: PlayerWeapon) -> bool {
        functions::Player_SetArmedWeapon(self, weapon as u8)
    }

    /// Creates a chat bubble above a player's name tag.
    pub fn set_chat_bubble(
        &self,
        text: &str,
        colour: Colour,
        drawdistance: f32,
        expiretime: i32,
    ) -> bool {
        functions::Player_SetChatBubble(self, text, colour.rgba(), drawdistance, expiretime)
    }

    /// This sets the players position then adjusts the players z-coordinate to the nearest solid ground under the position.
    pub fn set_pos_find_z(&self, pos: Vector3) -> bool {
        functions::Player_SetPosFindZ(self, pos.x, pos.y, pos.z)
    }

    /// Set the skill level of a certain weapon type for a player.
    pub fn set_skill_level(&self, weapon: PlayerWeaponSkill, level: i32) -> bool {
        functions::Player_SetSkillLevel(self, weapon as u8, level)
    }

    /// This function allows to set players special action.
    pub fn set_special_action(&self, action: PlayerSpecialAction) -> bool {
        functions::Player_SetSpecialAction(self, action as u32)
    }

    /// This functions allows you to toggle the drawing of player nametags, healthbars and armor bars which display above their head.
    pub fn show_name_tag(&self, other: &Player, enable: bool) -> bool {
        functions::Player_ShowNameTagForPlayer(self, other, enable)
    }

    /// Toggles whether a player can control their character or not.
    pub fn toggle_controllable(&self, enable: bool) -> bool {
        functions::Player_ToggleControllable(self, enable)
    }

    /// Toggle whether a player is in spectator mode or not.
    pub fn toggle_spectating(&self, enable: bool) -> bool {
        functions::Player_ToggleSpectating(self, enable)
    }

    /// Apply an animation to a player.
    pub fn apply_animation(
        &self,
        animation_data: AnimationData,
        sync: PlayerAnimationSyncType,
    ) -> bool {
        functions::Player_ApplyAnimation(
            self,
            &animation_data.get_animation_library(),
            &animation_data.get_name(),
            animation_data.delta,
            animation_data.looping,
            animation_data.lockX,
            animation_data.lockY,
            animation_data.freeze,
            animation_data.time as u32,
            sync as i32,
        )
    }

    /// Enter edition mode for an attached object.
    pub fn edit_attached_object(&self, index: i32) -> bool {
        functions::Player_EditAttachedObject(self, index)
    }

    /// Toggle camera targeting functions for a player.
    pub fn enable_camera_target(&self, enable: bool) -> bool {
        functions::Player_EnableCameraTarget(self, enable)
    }

    /// Toggle stunt bonuses for a player.
    pub fn enable_stunt_bonus(&self, enable: bool) -> bool {
        functions::Player_EnableStuntBonus(self, enable)
    }

    /// Gets the amount of ammo in a player's current weapon.
    pub fn get_ammo(&self) -> i32 {
        functions::Player_GetPlayerAmmo(self)
    }

    /// Returns the index of any running applied animations.
    pub fn get_animation_index(&self) -> i32 {
        functions::Player_GetAnimationIndex(self)
    }

    /// Gets the angle a player is facing.
    pub fn get_facing_angle(&self) -> f32 {
        functions::Player_GetFacingAngle(self)
    }

    /// Get the specified player's IP address and store it in a string.
    pub fn get_ip(&self) -> String {
        let mut ip = String::new();
        functions::Player_GetIp(self, &mut ip);
        ip
    }

    /// Retrieves a player's current special action.
    pub fn get_special_action(&self) -> PlayerSpecialAction {
        unsafe { transmute(functions::Player_GetSpecialAction(self)) }
    }

    /// This function gets the ID of the vehicle the player is currently in.
    pub fn get_vehicle_id(&self) -> i32 {
        functions::Player_GetVehicleID(self)
    }

    /// Find out which seat a player is in.
    pub fn get_vehicle_seat(&self) -> i32 {
        functions::Player_GetVehicleSeat(self)
    }

    /// Get the weapon and ammo in a specific player's weapon slot (e.
    pub fn get_weapon_data(&self, slot: i32) -> WeaponSlotData {
        let mut ammo: i32 = 0;
        let mut id: i32 = 0;
        functions::Player_GetWeaponData(self, slot, &mut id, &mut ammo);
        WeaponSlotData {
            ammo: ammo as _,
            id: unsafe { transmute(id as u8) },
        }
    }

    /// Check the state of a player's weapon.
    pub fn get_weapon_state(&self) -> i32 {
        functions::Player_GetWeaponState(self)
    }

    /// Move a player's camera from one position to another, within the set time.
    pub fn interpolate_camera_pos(
        &self,
        from: Vector3,
        to: Vector3,
        time: i32,
        cut: PlayerCameraCutType,
    ) -> bool {
        functions::Player_InterpolateCameraPos(
            self, from.x, from.y, from.z, to.x, to.y, to.z, time, cut as i32,
        )
    }

    /// Interpolate a player's camera's 'look at' point between two coordinates with a set speed.
    pub fn interpolate_camera_look_at(
        &self,
        from: Vector3,
        to: Vector3,
        time: i32,
        cut: PlayerCameraCutType,
    ) -> bool {
        functions::Player_InterpolateCameraLookAt(
            self, from.x, from.y, from.z, to.x, to.y, to.z, time, cut as i32,
        )
    }

    /// Check if a player has an object attached in the specified index (slot).
    pub fn is_attached_object_slot_used(&self, index: i32) -> bool {
        functions::Player_IsPlayerAttachedObjectSlotUsed(self, index)
    }

    /// You can use this function to attach the player camera to objects.
    pub fn attach_camera_to_object(&self, object: &Object) -> bool {
        functions::Player_AttachCameraToObject(self, object)
    }

    /// Attaches a player's camera to a player-object.
    pub fn attach_camera_to_player_object(&self, object: &PlayerObject) -> bool {
        functions::Player_AttachCameraToPlayerObject(self, object)
    }

    /* pub fn get_aim_data(&self) -> PlayerAimData {
        functions::GetPlayerAimData(self)
    } */

    /// Check which keys a player is pressing.
    pub fn get_keys(&self) -> PlayerKeyData {
        let mut keys = 0;
        let mut updown = 0;
        let mut leftright = 0;
        functions::Player_GetKeys(self, &mut keys, &mut updown, &mut leftright);
        PlayerKeyData {
            keys: keys as _,
            leftRight: leftright as _,
            upDown: updown as _,
        }
    }

    pub fn get_surfing_object(&self) -> Option<Object> {
        functions::Player_GetSurfingObject(self)
    }

    pub fn get_surfing_vehicle(&self) -> Option<Vehicle> {
        functions::Player_GetSurfingVehicle(self)
    }

    /// Check who a player is aiming at.
    pub fn get_target_player(&self) -> Option<Player> {
        functions::Player_GetCameraTargetPlayer(self)
    }

    /// Gets id of an actor which is aimed by certain player.
    pub fn get_target_actor(&self) -> Option<Actor> {
        functions::Player_GetCameraTargetActor(self)
    }

    /// Checks if a player is in a specific vehicle.
    pub fn is_in_vehicle(&self, target_vehicle: &Vehicle) -> bool {
        functions::Player_IsInVehicle(self, target_vehicle)
    }

    /// Check if a player is inside any vehicle (as a driver or passenger).
    pub fn is_in_any_vehicle(&self) -> bool {
        functions::Player_IsInAnyVehicle(self)
    }

    /// Checks if a player is in range of a point.
    pub fn is_in_range_of_point(&self, range: f32, coord: Vector3) -> bool {
        functions::Player_IsInRangeOfPoint(self, range, coord.x, coord.y, coord.z)
    }

    /// This function plays a crime report for a player - just like in single-player when CJ commits a crime.
    pub fn play_crime_report(&self, suspect: &Player, crime: i32) -> bool {
        functions::Player_PlayCrimeReport(self, suspect, crime)
    }

    /// Remove an attached object from a player.
    pub fn remove_attached_object(&self, index: i32) -> bool {
        functions::Player_RemoveAttachedObject(self, index)
    }

    /// Set a player's facing angle (Z rotation).
    pub fn set_facing_angle(&self, angle: f32) -> bool {
        functions::Player_SetFacingAngle(self, angle)
    }

    /// Change the colour of a player's nametag and radar blip for another player.
    pub fn set_marker_for_player(&self, other: &Player, colour: Colour) -> bool {
        functions::Player_SetMarkerForPlayer(self, other, colour.rgba())
    }

    /// Get the colour of a player's nametag and radar blip for another player.
    pub fn get_marker_for_player(&self, other: &Player) -> u32 {
        functions::Player_GetMarkerForPlayer(self, other)
    }

    /// Enable/Disable the teleporting ability for a player by right-clicking on the map.
    pub fn allow_teleport(&self, allow: bool) -> bool {
        functions::Player_AllowTeleport(self, allow)
    }

    /// Can this player teleport by right-clicking on the map?
    pub fn is_teleport_allowed(&self) -> bool {
        functions::Player_IsTeleportAllowed(self)
    }

    pub fn set_remote_vehicle_collisions(&self, collide: bool) -> bool {
        functions::Player_DisableRemoteVehicleCollisions(self, !collide)
    }

    /// Display the cursor and allow the player to select a textdraw.
    pub fn select_text_draw(&self, hover_colour: Colour) -> bool {
        functions::Player_SelectTextDraw(self, hover_colour.rgba())
    }

    /// Cancel textdraw selection with the mouse.
    pub fn cancel_select_text_draw(&self) -> bool {
        functions::Player_CancelSelectTextDraw(self)
    }

    /// Perform a memory check on the client.
    pub fn send_client_check(
        &self,
        action_type: i32,
        address: i32,
        offset: i32,
        count: i32,
    ) -> bool {
        functions::Player_SendClientCheck(self, action_type, address, offset, count)
    }

    /// (Re)Spawns a player.
    pub fn spawn(&self) -> bool {
        functions::Player_Spawn(self)
    }

    /// Fetch the CI (computer/client identification) of a user, this is linked to their SAMP/GTA on their computer.
    pub fn gpci(&self) -> String {
        let mut output = String::new();
        functions::Player_GPCI(self, &mut output);
        output
    }

    /// Check if a player is logged in as an RCON admin.
    pub fn is_admin(&self) -> bool {
        functions::Player_IsAdmin(self)
    }
    /// Kicks a player from the server. They will have to quit the game and re-connect if they wish to continue playing.
    pub fn kick(&self) -> bool {
        functions::Player_Kick(self)
    }

    /// Shows 'game text' (on-screen text) for a certain length of time for a specific player.
    pub fn show_game_text(&self, msg: &str, time: i32, style: i32) -> bool {
        functions::Player_ShowGameText(self, msg, time, style)
    }

    /// Stop showing a gametext style to a player.
    pub fn hide_game_text(&self, style: i32) -> bool {
        functions::Player_HideGameText(self, style)
    }

    /// Does the player currently have text in the given gametext style displayed?
    pub fn has_game_text(&self, style: i32) -> bool {
        functions::Player_HasGameText(self, style)
    }

    /// Returns all the information on the given game text style.
    pub fn get_game_text(
        &self,
        style: i32,
        message: &mut String,
        time: &mut i32,
        remaining: &mut i32,
    ) -> bool {
        functions::Player_GetGameText(self, style, message, time, remaining)
    }

    /// Ban a player who is currently in the server.
    pub fn ban(&self) -> bool {
        functions::Player_Ban(self)
    }

    /// Ban a player with a reason.
    pub fn ban_ex(&self, msg: &str) -> bool {
        functions::Player_BanEx(self, msg)
    }

    /// Sends a message in the name of a player to another player on the server.
    pub fn send_message_to_player(&self, sender: &Player, message: &str) -> bool {
        functions::Player_SendMessageToPlayer(self, sender, message)
    }

    /// Returns the SA-MP client version, as reported by the player.
    pub fn get_version(&self) -> String {
        let mut output = String::new();
        functions::Player_GetVersion(self, &mut output);
        output
    }

    /// Get the player skill level of a certain weapon type.
    pub fn get_skill_level(&self, skill: i32) -> i32 {
        functions::Player_GetSkillLevel(self, skill)
    }

    /// Gets the ID of the player or vehicle the player is spectating (watching).
    pub fn get_spectate_id(&self) -> i32 {
        functions::Player_GetPlayerSpectateID(self)
    }

    pub fn get_spectate_data(&self) -> PlayerSpectateData {
        let mut spectate_data = PlayerSpectateData {
            spectating: false,
            spectateID: self.get_spectate_id(),
            spectate_type: unsafe { transmute(functions::Player_GetSpectateType(self)) },
        };
        spectate_data.spectating = spectate_data.spectateID != 0xFFFF;

        spectate_data
    }

    /// Get the specified player's Raw IP address (v4).
    pub fn get_raw_ip(&self) -> u32 {
        functions::Player_GetRawIp(self)
    }

    /// Set a player's gravity.
    pub fn set_gravity(&self, gravity: f32) -> bool {
        functions::Player_SetGravity(self, gravity)
    }

    /// Get a player's gravity.
    pub fn get_gravity(&self) -> f32 {
        functions::Player_GetGravity(self)
    }

    /// Sets the player as an RCON admin.
    pub fn set_admin(&self, set: bool) -> bool {
        functions::Player_SetAdmin(self, set)
    }

    /// Checks if a player is spawned.
    pub fn is_spawned(&self) -> bool {
        functions::Player_IsSpawned(self)
    }

    /// Check if the player is controllable.
    pub fn is_controllable(&self) -> bool {
        functions::Player_IsControllable(self)
    }

    /// Check if the player camera target is enabled.
    pub fn is_camera_target_enabled(&self) -> bool {
        functions::Player_IsCameraTargetEnabled(self)
    }

    /// Toggle player's ghost mode.
    pub fn toggle_ghost_mode(&self, toggle: bool) -> bool {
        functions::Player_ToggleGhostMode(self, toggle)
    }

    /// Get player's ghost mode.
    pub fn get_ghost_mode(&self) -> bool {
        functions::Player_GetGhostMode(self)
    }

    /// Enable/Disable weapons for a player.
    pub fn allow_weapons(&self, allow: bool) -> bool {
        functions::Player_AllowWeapons(self, allow)
    }

    /// Can the player use weapons?
    pub fn are_weapons_allowed(&self) -> bool {
        functions::Player_AreWeaponsAllowed(self)
    }

    /// Check if the player is using the official SA-MP client.
    pub fn is_using_official_client(&self) -> bool {
        functions::Player_IsPlayerUsingOfficialClient(self)
    }

    pub fn get_animation_data(&self) -> PlayerAnimationData {
        PlayerAnimationData {
            id: functions::Player_GetAnimationIndex(self) as u16,
            flags: functions::Player_GetAnimationFlags(self) as u16,
        }
    }

    /// Check if the player is in driveby mode.
    pub fn is_in_drive_by_mode(&self) -> bool {
        functions::Player_IsInDriveByMode(self)
    }

    /// Checks if the player special action is cuffed.
    pub fn is_cuffed(&self) -> bool {
        functions::Player_IsCuffed(self)
    }

    /// Returns the class of the players custom skin downloaded from the server.
    pub fn get_custom_skin(&self) -> i32 {
        functions::Player_GetCustomSkin(self)
    }

    /// Redirect a player custom AddCharModel or AddSimpleModel download to a specific HTTP webpage.
    pub fn redirect_download(&self, url: &str) -> bool {
        models::functions::CustomModel_RedirectDownload(self, url)
    }

    // Player Checkpoints methods
    /// Sets a checkpoint (red cylinder) for a player.
    pub fn set_player_checkpoint(&self, centre_position: Vector3, radius: f32) -> bool {
        checkpoints::functions::Checkpoint_Set(
            self,
            centre_position.x,
            centre_position.y,
            centre_position.z,
            radius,
        )
    }

    /// Disables (hides/destroys) a player's set checkpoint.
    pub fn disable_player_checkpoint(&self) -> bool {
        checkpoints::functions::Checkpoint_Disable(self)
    }

    /// Check if the player is currently inside a checkpoint, this could be used for properties or teleport points for example.
    pub fn is_player_in_checkpoint(&self) -> bool {
        checkpoints::functions::Checkpoint_IsPlayerIn(self)
    }

    /// Creates a race checkpoint.
    pub fn set_player_race_checkpoint(
        &self,
        race_check_point_type: RaceCheckpointType,
        centre_position: Vector3,
        next_position: Vector3,
        radius: f32,
    ) -> bool {
        checkpoints::functions::RaceCheckpoint_Set(
            self,
            race_check_point_type as i32,
            centre_position.x,
            centre_position.y,
            centre_position.z,
            next_position.x,
            next_position.y,
            next_position.z,
            radius,
        )
    }

    /// Disable any initialized race checkpoints for a specific player, since you can only have one at any given time.
    pub fn disable_player_race_checkpoint(&self) -> bool {
        checkpoints::functions::Checkpoint_Disable(self)
    }

    /// Check if the player is inside their current set race checkpoint (SetPlayerRaceCheckpoint).
    pub fn is_player_in_race_checkpoint(&self) -> bool {
        checkpoints::functions::Checkpoint_IsPlayerIn(self)
    }

    /// Check if the player currently has a checkpoint visible.
    pub fn is_player_checkpoint_active(&self) -> bool {
        checkpoints::functions::Checkpoint_IsActive(self)
    }

    /// Get the location of the current checkpoint.
    pub fn get_player_checkpoint(&self) -> PlayerCheckPointData {
        let mut center_pos = Vector3::default();
        let mut radius = 0.0;
        checkpoints::functions::Checkpoint_Get(
            self,
            &mut center_pos.x,
            &mut center_pos.y,
            &mut center_pos.z,
            &mut radius,
        );
        PlayerCheckPointData::new(center_pos, radius)
    }

    /// Check if the player currently has a race checkpoint visible.
    pub fn is_player_race_checkpoint_active(&self) -> bool {
        checkpoints::functions::RaceCheckpoint_IsActive(self)
    }

    /// Get the location of the current race checkpoint.
    pub fn get_player_race_checkpoint(&self) -> PlayerRaceCheckPointData {
        let mut center_pos = Vector3::default();
        let mut next_pos = Vector3::default();
        let mut radius = 0.0;
        checkpoints::functions::RaceCheckpoint_Get(
            self,
            &mut center_pos.x,
            &mut center_pos.y,
            &mut center_pos.z,
            &mut next_pos.x,
            &mut next_pos.y,
            &mut next_pos.z,
            &mut radius,
        );
        PlayerRaceCheckPointData::new(center_pos, next_pos, radius)
    }

    /// This function can be used to change the spawn information of a specific player.
    pub fn set_spawn_info(&self, player_class: PlayerClass) -> bool {
        functions::Player_SetSpawnInfo(
            self,
            player_class.team,
            player_class.skin,
            player_class.spawn.x,
            player_class.spawn.y,
            player_class.spawn.z,
            player_class.angle,
            player_class.weapons[0].id as u8,
            player_class.weapons[0].ammo,
            player_class.weapons[1].id as u8,
            player_class.weapons[1].ammo,
            player_class.weapons[2].id as u8,
            player_class.weapons[2].ammo,
        )
    }

    /// Return the current spawn data for a player, where they will spawn next.
    pub fn get_spawn_info(&self) -> PlayerClass {
        let mut player_class = PlayerClass::default();
        let (mut weapon1, mut ammo1, mut weapon2, mut ammo2, mut weapon3, mut ammo3): (
            u8,
            u32,
            u8,
            u32,
            u8,
            u32,
        ) = (0, 0, 0, 0, 0, 0);

        functions::Player_GetSpawnInfo(
            self,
            &mut player_class.team,
            &mut player_class.skin,
            &mut player_class.spawn.x,
            &mut player_class.spawn.y,
            &mut player_class.spawn.z,
            &mut player_class.angle,
            &mut weapon1,
            &mut ammo1,
            &mut weapon2,
            &mut ammo2,
            &mut weapon3,
            &mut ammo3,
        );
        player_class.weapons[0].id = unsafe { transmute(weapon1) };
        player_class.weapons[0].ammo = ammo1;
        player_class.weapons[1].id = unsafe { transmute(weapon2) };
        player_class.weapons[1].ammo = ammo2;
        player_class.weapons[2].id = unsafe { transmute(weapon3) };
        player_class.weapons[2].ammo = ammo3;
        player_class
    }

    /// Shows the player a synchronous (only one at a time) dialog box.
    pub fn show_dialog(
        &self,
        dialog: i32,
        style: DialogStyle,
        title: &str,
        body: &str,
        button1: &str,
        button2: &str,
    ) -> bool {
        dialogs::functions::Dialog_Show(self, dialog, style as i32, title, body, button1, button2)
    }

    /// Get the ID of the dialog currently show to the player.
    pub fn get_dialog_id(&self) -> i32 {
        functions::Player_GetDialog(self)
    }

    /// Hides any dialog the player may currently be able to see.
    pub fn hide_dialog(&self) -> bool {
        dialogs::functions::Dialog_Hide(self)
    }

    pub fn get_id(&self) -> usize {
        functions::Player_GetID(self) as usize
    }
    pub fn from_id(playerid: i32) -> Option<Player> {
        functions::Player_FromID(playerid)
    }
    /// Gets the ID of the menu the player is currently viewing (shown by ShowMenuForPlayer).
    pub fn get_menu(&self) -> Option<Menu> {
        functions::Player_GetMenu(self)
    }

    /// Allows a player to edit an object (position and rotation) using their mouse on a GUI (Graphical User Interface).
    pub fn edit_object(&self, object: &Object) -> bool {
        objects::functions::Object_BeginEditing(self, object)
    }
    /// Display the cursor and allow the player to select an object.
    pub fn select_object(&self) -> bool {
        objects::functions::Object_BeginSelecting(self)
    }
    /// Cancel object edition mode for a player.
    pub fn end_object_editing(&self) -> bool {
        objects::functions::Object_EndEditing(self)
    }
    /// Creates an object which will be visible to only one player.
    pub fn create_player_object(
        &self,
        modelid: i32,
        position: Vector3,
        rotation: Vector3,
        drawDistance: f32,
    ) -> Option<PlayerObject> {
        let mut _id = -1;
        objects::functions::PlayerObject_Create(
            self,
            modelid,
            position.x,
            position.y,
            position.z,
            rotation.x,
            rotation.y,
            rotation.z,
            drawDistance,
            &mut _id,
        )
    }
    /// Destroy a player-object created using CreatePlayerObject.
    pub fn destroy_player_object(&self, object: PlayerObject) -> bool {
        objects::functions::PlayerObject_Destroy(self, &object)
    }
    /// Allows players to edit a player-object (position and rotation) with a GUI and their mouse.
    pub fn edit_player_object(&self, object: &PlayerObject) -> bool {
        objects::functions::PlayerObject_BeginEditing(self, object)
    }

    /* /// Get PlayerObject from an id
    pub fn get_player_object_from_id(&self, id: i32) -> Option<PlayerObject> {
        objects::functions::GetPlayerObjectFromID(self, id)
    } */

    /// Creates a textdraw for a single player.
    pub fn create_player_text_draw(&self, position: Vector2, text: &str) -> Option<PlayerTextDraw> {
        let mut _id = 0;
        textdraws::functions::PlayerTextDraw_Create(self, position.x, position.y, text, &mut _id)
    }
    /// Destroy a player-textdraw.
    pub fn player_text_draw_destroy(&self, textdraw: &PlayerTextDraw) -> bool {
        textdraws::functions::PlayerTextDraw_Destroy(self, textdraw)
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
        let mut _id = 0;
        textlabels::functions::PlayerTextLabel_Create(
            self,
            text,
            colour.rgba(),
            position.x,
            position.y,
            position.z,
            drawDistance,
            attachedPlayer,
            &Vehicle::new(std::ptr::null()),
            los,
            &mut _id,
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
        let mut _id = 0;
        textlabels::functions::PlayerTextLabel_Create(
            self,
            text,
            colour.rgba(),
            position.x,
            position.y,
            position.z,
            drawDistance,
            &Player::new(std::ptr::null()),
            attachedVehicle,
            los,
            &mut _id,
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
        let mut _id = 0;
        textlabels::functions::PlayerTextLabel_Create(
            self,
            text,
            colour.rgba(),
            position.x,
            position.y,
            position.z,
            drawDistance,
            &Player::new(std::ptr::null()),
            &Vehicle::new(std::ptr::null()),
            los,
            &mut _id,
        )
    }
    /// Destroy a 3D text label that was created using CreatePlayer3DTextLabel.
    pub fn delete_player_text_label(&self, textlabel: PlayerTextLabel) -> bool {
        textlabels::functions::PlayerTextLabel_Destroy(self, &textlabel)
    }

    /// Check if the player is in the mod shop.
    pub fn is_player_in_mod_shop(&self) -> bool {
        functions::Player_IsInModShop(self)
    }
    /// Gets the siren state of the player's vehicle.
    pub fn get_player_siren_state(&self) -> i32 {
        functions::Player_GetSirenState(self)
    }
    /// Gets the landing gear state of the current player's vehicle.
    pub fn get_player_landing_gear_state(&self) -> i32 {
        functions::Player_GetLandingGearState(self)
    }
    /// Gets the hydra reactor angle of the player's vehicle.
    pub fn get_player_hydra_reactor_angle(&self) -> u32 {
        functions::Player_GetHydraReactorAngle(self)
    }
    /// Gets the speed of the player's train.
    pub fn get_player_train_speed(&self) -> f32 {
        functions::Player_GetTrainSpeed(self)
    }

    /// Get a player's network stats.
    pub fn get_net_stats(&self) -> String {
        let mut output = String::new();
        functions::Player_GetNetworkStats(self, &mut output);
        output
    }

    /// Get a player's IP and port.
    pub fn net_stats_get_ip_port(&self) -> String {
        let mut output = String::new();
        functions::Player_NetStatsGetIpPort(self, &mut output);
        output
    }

    /* /// Sends a message in the name of a player to all other players on the server.
    pub fn send_message_to_all(&self, message: &str) -> bool {
        core::functions::All_SendClientMessage(self, message)
    } */

    /// Attach an object to a specific bone on a player.
    pub fn set_attached_object(&self, index: i32, attachment: ObjectAttachmentSlotData) -> bool {
        functions::Player_SetAttachedObject(
            self,
            index,
            attachment.model,
            attachment.bone,
            attachment.offset.x,
            attachment.offset.y,
            attachment.offset.z,
            attachment.rotation.x,
            attachment.rotation.y,
            attachment.rotation.z,
            attachment.scale.x,
            attachment.scale.y,
            attachment.scale.z,
            attachment.colour1.argb(),
            attachment.colour2.argb(),
        )
    }
    /// Gets the player attachment object data by index.
    pub fn get_attached_object(&self, index: i32) -> ObjectAttachmentSlotData {
        let mut attachment = ObjectAttachmentSlotData::default();
        let mut colour1 = 0;
        let mut colour2 = 0;
        functions::Player_GetAttachedObject(
            self,
            index,
            &mut attachment.model,
            &mut attachment.bone,
            &mut attachment.offset.x,
            &mut attachment.offset.y,
            &mut attachment.offset.z,
            &mut attachment.rotation.x,
            &mut attachment.rotation.y,
            &mut attachment.rotation.z,
            &mut attachment.scale.x,
            &mut attachment.scale.y,
            &mut attachment.scale.z,
            &mut colour1,
            &mut colour2,
        );
        attachment.colour1 = Colour::from_argb(colour1 as u32);
        attachment.colour2 = Colour::from_argb(colour2 as u32);

        attachment
    }
}

/// Map Icon Styles
#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum MapIconStyle {
    Local,
    Global,
    LocalCheckpoint,
    GlobalCheckpoint,
}

/// Client Versions
#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ClientVersion {
    Samp037,
    Samp03dl,
    Openmp,
}

/// Camera Cut Types
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

impl From<i32> for PlayerNameStatus {
    fn from(value: i32) -> Self {
        match value {
            0 => PlayerNameStatus::Updated,
            1 => PlayerNameStatus::Taken,
            _ => PlayerNameStatus::Invalid,
        }
    }
}
/// Animation sync type
#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PlayerAnimationSyncType {
    /// No sync
    NoSync,
    // Make server sync the animation with all other players in streaming radius
    Sync,
    /// works same as Sync, but will ONLY apply the animation to streamed-in players, but NOT the actual player being animated (useful for npc animations and persistent animations when players are being streamed)
    SyncOthers,
}

/// Weapon Information
#[repr(C)]
#[derive(Default, Clone, Copy, Debug)]
pub struct WeaponSlotData {
    /// weapon id
    pub id: PlayerWeapon,
    /// amount of ammunition
    pub ammo: u32,
}

impl WeaponSlotData {
    pub fn new(id: PlayerWeapon, ammo: u32) -> Self {
        Self { id, ammo }
    }
}

/// Animation Data Of Player
#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct PlayerAnimationData {
    pub id: u16,
    /// Animation flags
    /// FREEZE_FLAG -> 0b0000000000000100
    /// LOCK_X_FLAG -> 0b0010000000000
    /// LOCK_Y_FLAG -> 0b0001000000000
    /// LOOP_FLAG -> 0b0000100000000
    pub flags: u16,
}

/// Player's Fighting Style
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

/// State of the player
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

impl From<i32> for PlayerState {
    fn from(value: i32) -> Self {
        match value {
            1 => PlayerState::OnFoot,
            2 => PlayerState::Driver,
            3 => PlayerState::Passenger,
            4 => PlayerState::ExitVehicle,
            5 => PlayerState::EnterVehicleDriver,
            6 => PlayerState::EnterVehiclePassenger,
            7 => PlayerState::Wasted,
            8 => PlayerState::Spawned,
            9 => PlayerState::Spectating,
            _ => PlayerState::None,
        }
    }
}

/// a list of valid weapon skill types used by set_skill_level and get_skill_level methods
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

/// list of all the player special actions
#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PlayerSpecialAction {
    /// Clears player of special actions
    None,
    /// Detect if the player is crouching.
    Duck,
    /// Will make the player using jetpack
    Jetpack,
    /// Detect if the player is entering a vehicle via an animation.
    EnterVehicle,
    /// Detect if the player is exiting a vehicle via an animation.
    ExitVehicle,
    /// Applies dancing animation for player
    Dance1,
    /// Applies dancing animation for player
    Dance2,
    /// Applies dancing animation for player
    Dance3,
    /// Applies dancing animation for player
    Dance4,
    /// Will make the player put hands up
    HandsUp = 10,
    /// Will make the player speaking on cellphone
    Cellphone,
    /// Detects if the player is sitting
    Sitting,
    /// Makes players stop using cellphone
    StopCellphone,
    /// Will increase the player's drunk level when used
    Beer = 20,
    /// Will give the player a cigar.
    Smoke,
    /// Will give the player a wine bottle to get drunk from
    Wine,
    /// Will give the player a sprunk bottle to drink from
    Sprunk,
    /// Will force the player in to cuffs (hands are behind their back) (does not work on CJ skin).
    Cuffed,
    /// Will apply a 'carrying' animation to the player and make them unable to sprint, jump or punch (does not work on CJ skin).
    Carry,
    /// Will make the player perform the pissing animation with visible pee
    Pissing = 68,
}

/// Player surfing information
#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct PlayerSurfingData {
    pub surftype: i32,
    pub id: i32,
    pub offset: Vector3,
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct PlayerKeyData {
    pub keys: u32,
    pub upDown: i16,
    pub leftRight: i16,
}

#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug, Default)]
pub struct PlayerBulletData {
    pub origin: Vector3,
    pub hitPos: Vector3,
    pub offset: Vector3,
    pub weapon: PlayerWeapon,
    pub hitType: PlayerBulletHitType,
    pub hitID: u16,
}
#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug, Default)]
pub enum PlayerBulletHitType {
    #[default]
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
    pub spectating: bool,
    pub spectateID: i32,
    pub spectate_type: SpectateType,
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
    pub camFrontVector: Vector3,
    pub camPos: Vector3,
    pub aimZ: f32,
    pub camZoom: f32,
    pub aspectRatio: f32,
    pub weaponState: PlayerWeaponState,
    pub camMode: u8,
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

/// Player Keys
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
    pub const UP: i32 = -128;
    pub const DOWN: i32 = 128;
    pub const LEFT: i32 = -128;
    pub const RIGHT: i32 = 128;
}
