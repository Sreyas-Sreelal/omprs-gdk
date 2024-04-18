use std::os::raw::c_void;

use crate::{
    actors::Actor,
    animationdata::AnimationData,
    colour::Colour,
    functions,
    objects::{Object, PlayerObject},
    vector::{Vector3, Vector4},
    vehicles::Vehicle,
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
        functions::players::SendClientMessage(self, colour, message);
    }

    pub fn get_name(&self) -> String {
        let mut name = String::new();
        functions::players::GetPlayerName(self, &mut name);
        name
    }

    /* pub fn send_client_message_to_all(&self, colour: Colour, message: String) {
        todo!()
    } */

    pub fn set_camera_pos(&self, pos: Vector3) {
        functions::players::SetPlayerCameraPos(self, pos);
    }

    pub fn set_drunk_level(&self, level: isize) {
        functions::players::SetPlayerDrunkLevel(self, level);
    }

    pub fn set_interior(&self, interiorid: isize) {
        functions::players::SetPlayerInterior(self, interiorid);
    }

    pub fn set_wanted_level(&self, level: isize) {
        functions::players::SetPlayerWantedLevel(self, level);
    }

    pub fn set_weather(&self, weatherid: isize) {
        functions::players::SetPlayerWeather(self, weatherid);
    }

    pub fn get_weather(&self) -> isize {
        functions::players::GetPlayerWeather(self)
    }

    pub fn set_skin(&self, skinid: isize) {
        functions::players::SetPlayerSkin(self, skinid);
    }

    pub fn set_shop_name(&self, shopname: &str) {
        functions::players::SetPlayerShopName(self, shopname)
    }

    pub fn give_money(&self, amount: isize) {
        functions::players::GivePlayerMoney(self, amount)
    }

    pub fn set_camera_look_at(&self, pos: Vector3, cut: isize) {
        functions::players::SetPlayerCameraLookAt(self, pos, cut)
    }

    pub fn set_camera_behind_player(&self) {
        functions::players::SetCameraBehindPlayer(self)
    }

    pub fn create_explosion(&self, pos: Vector3, explosion_type: isize, radius: f32) {
        functions::players::CreateExplosionForPlayer(self, pos, explosion_type, radius);
    }
    /* pub fn create_explosion_for_all(&self, pos: Vector3, explosion_type: isize, radius: f32) {
        todo!()
    } */
    pub fn play_audio_stream(&self, url: &str, pos: Vector3, distance: f32, use_pos: bool) {
        functions::players::PlayAudioStreamForPlayer(self, url, pos, distance, use_pos);
    }

    pub fn stop_audio_stream(&self) {
        functions::players::StopAudioStreamForPlayer(self)
    }

    pub fn send_death_message(&self, killee: Player, weapon: isize) {
        functions::players::SendDeathMessage(self, &killee, weapon);
    }

    pub fn toggle_widescreen(&self, enable: bool) {
        functions::players::TogglePlayerWidescreen(self, enable)
    }

    pub fn is_widescreen_toggled(&self) -> bool {
        functions::players::IsPlayerWidescreenToggled(self)
    }

    pub fn set_health(&self, health: f32) {
        functions::players::SetPlayerHealth(self, health);
    }

    pub fn get_health(&self) -> f32 {
        functions::players::GetPlayerHealth(self)
    }

    pub fn set_armour(&self, armour: f32) {
        functions::players::SetPlayerArmour(self, armour)
    }

    pub fn get_armour(&self) -> f32 {
        functions::players::GetPlayerArmour(self)
    }

    pub fn set_team(&self, teamid: isize) {
        functions::players::SetPlayerTeam(self, teamid)
    }

    pub fn get_team(&self) -> isize {
        functions::players::GetPlayerTeam(self)
    }

    pub fn set_score(&self, score: isize) {
        functions::players::SetPlayerScore(self, score)
    }

    pub fn get_score(&self) -> isize {
        functions::players::GetPlayerScore(self)
    }

    pub fn get_skin(&self) -> isize {
        functions::players::GetPlayerSkin(self)
    }

    pub fn set_color(&self, colour: Colour) {
        functions::players::SetPlayerColor(self, colour)
    }

    pub fn get_color(&self) -> isize {
        functions::players::GetPlayerColor(self)
    }

    pub fn get_default_colour(&self) -> isize {
        functions::players::GetDefaultPlayerColour(self)
    }

    pub fn get_drunk_level(&self) -> isize {
        functions::players::GetPlayerDrunkLevel(self)
    }

    pub fn give_weapon(&self, data: WeaponSlotData) {
        functions::players::GivePlayerWeapon(self, data)
    }

    pub fn remove_weapon(&self, weaponid: u8) {
        functions::players::RemovePlayerWeapon(self, weaponid)
    }

    pub fn get_money(&self) -> isize {
        functions::players::GetPlayerMoney(self)
    }

    pub fn reset_money(&self) {
        functions::players::ResetPlayerMoney(self)
    }

    pub fn set_name(&self, name: &str) -> EPlayerNameStatus {
        functions::players::SetPlayerName(self, name)
    }

    pub fn get_state(&self) -> PlayerState {
        functions::players::GetPlayerState(self)
    }

    pub fn get_ping(&self) -> isize {
        functions::players::GetPlayerPing(self)
    }

    pub fn get_weapon(&self) -> isize {
        functions::players::GetPlayerWeapon(self)
    }

    pub fn set_time(&self, hour: isize, minute: isize) {
        functions::players::SetPlayerTime(self, hour, minute)
    }

    pub fn get_time(&self, hour: &mut isize, minute: &mut isize) {
        functions::players::GetPlayerTime(self, hour, minute)
    }

    pub fn toggle_clock(&self, enable: bool) {
        functions::players::TogglePlayerClock(self, enable)
    }

    pub fn has_clock_enabled(&self) -> bool {
        functions::players::PlayerHasClockEnabled(self)
    }

    pub fn force_class_selection(&self) {
        functions::players::ForceClassSelection(self)
    }

    pub fn get_wanted_level(&self) -> isize {
        functions::players::GetPlayerWantedLevel(self)
    }

    pub fn set_fighting_style(&self, style: PlayerFightingStyle) {
        functions::players::SetPlayerFightingStyle(self, style)
    }

    pub fn get_fighting_style(&self) -> PlayerFightingStyle {
        functions::players::GetPlayerFightingStyle(self)
    }

    pub fn set_velocity(&self, pos: Vector3) {
        functions::players::SetPlayerVelocity(self, pos)
    }

    pub fn get_velocity(&self) -> Vector3 {
        let mut pos = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        functions::players::GetPlayerVelocity(self, &mut pos);
        pos
    }

    pub fn get_camera_pos(&self) -> Vector3 {
        let mut pos = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        functions::players::GetPlayerCameraPos(self, &mut pos);
        pos
    }

    pub fn get_distance_from_point(&self, pos: Vector3) -> f32 {
        functions::players::GetPlayerDistanceFromPoint(self, pos)
    }

    pub fn get_interior(&self) -> isize {
        functions::players::GetPlayerInterior(self)
    }

    pub fn set_pos(&self, pos: Vector3) {
        functions::players::SetPlayerPos(self, pos)
    }

    pub fn get_pos(&self) -> Vector3 {
        let mut pos = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        functions::players::GetPlayerPos(self, &mut pos);
        pos
    }

    pub fn get_virtual_world(&self) -> isize {
        functions::players::GetPlayerVirtualWorld(self)
    }

    pub fn is_npc(&self) -> bool {
        functions::players::IsPlayerNPC(self)
    }

    pub fn is_streamed_in(&self, other: Player) -> bool {
        functions::players::IsPlayerStreamedIn(self, &other)
    }

    pub fn play_sound(&self, sound: usize, pos: Vector3) {
        functions::players::PlayerPlaySound(self, sound, pos)
    }

    pub fn spectate_player(&self, target: Player, mode: PlayerSpectateMode) {
        functions::players::PlayerSpectatePlayer(self, &target, mode)
    }

    pub fn spectate_vehicle(&self, vehicle: Vehicle, mode: PlayerSpectateMode) {
        functions::players::PlayerSpectateVehicle(self, &vehicle, mode)
    }

    pub fn set_virtual_world(&self, vw: isize) {
        functions::players::SetPlayerVirtualWorld(self, vw)
    }

    pub fn set_world_bounds(&self, coords: Vector4) {
        functions::players::SetPlayerWorldBounds(self, coords)
    }

    pub fn clear_world_bounds(&self) {
        functions::players::ClearPlayerWorldBounds(self)
    }

    pub fn get_world_bounds(&self) -> Vector4 {
        let mut pos = Vector4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };
        functions::players::GetPlayerWorldBounds(self, &mut pos);
        pos
    }

    pub fn clear_animations(&self, sync_type: PlayerAnimationSyncType) {
        functions::players::ClearAnimations(self, sync_type)
    }

    pub fn get_last_shot_vectors(&self) -> PlayerBulletData {
        functions::players::GetPlayerLastShotVectors(self)
    }

    pub fn get_camera_target_player(&self) -> Player {
        functions::players::GetPlayerCameraTargetPlayer(self)
    }

    pub fn get_camera_target_actor(&self) -> Actor {
        functions::players::GetPlayerCameraTargetActor(self)
    }

    pub fn get_camera_target_object(&self) -> Object {
        functions::players::GetPlayerCameraTargetObject(self)
    }

    pub fn get_camera_target_vehicle(&self) -> Vehicle {
        functions::players::GetPlayerCameraTargetVehicle(self)
    }

    pub fn put_in_vehicle(&self, vehicle: Vehicle, seat_id: isize) {
        functions::players::PutPlayerInVehicle(self, &vehicle, seat_id)
    }

    pub fn remove_building(&self, model: isize, pos: Vector3, radius: f32) {
        functions::players::RemoveBuildingForPlayer(self, model, pos, radius)
    }

    pub fn get_buildings_removed(&self) -> isize {
        functions::players::GetPlayerBuildingsRemoved(self)
    }

    pub fn remove_from_vehicle(&self, force: bool) {
        functions::players::RemovePlayerFromVehicle(self, force)
    }

    pub fn remove_map_icon(&self, icon_id: isize) {
        functions::players::RemovePlayerMapIcon(self, icon_id)
    }

    pub fn set_map_icon(
        &self,
        icon_id: isize,
        pos: Vector3,
        icon_type: isize,
        colour: Colour,
        style: MapIconStyle,
    ) {
        functions::players::SetPlayerMapIcon(self, icon_id, pos, icon_type, colour, style)
    }

    pub fn reset_weapons(&self) {
        functions::players::ResetPlayerWeapons(self)
    }

    pub fn set_ammo(&self, data: WeaponSlotData) {
        functions::players::SetPlayerAmmo(self, data)
    }

    pub fn set_armed_weapon(&self, weapon: isize) {
        functions::players::SetPlayerArmedWeapon(self, weapon)
    }

    pub fn set_chat_bubble(
        &self,
        text: &str,
        colour: Colour,
        drawdistance: f32,
        expiretime: isize,
    ) {
        functions::players::SetPlayerChatBubble(self, text, colour, drawdistance, expiretime)
    }

    pub fn set_pos_find_z(&self, pos: Vector3) {
        functions::players::SetPlayerPosFindZ(self, pos)
    }

    pub fn set_skill_level(&self, weapon: PlayerWeaponSkill, level: isize) {
        functions::players::SetPlayerSkillLevel(self, weapon, level)
    }

    pub fn set_special_action(&self, action: PlayerSpecialAction) {
        functions::players::SetPlayerSpecialAction(self, action)
    }

    pub fn show_name_tag(&self, other: Player, enable: bool) {
        functions::players::ShowPlayerNameTagForPlayer(self, &other, enable)
    }

    pub fn toggle_controllable(&self, enable: bool) {
        functions::players::TogglePlayerControllable(self, enable)
    }

    pub fn toggle_spectating(&self, enable: bool) {
        functions::players::TogglePlayerSpectating(self, enable)
    }

    pub fn apply_animation(&self, animation_data: AnimationData, sync: PlayerAnimationSyncType) {
        functions::players::ApplyAnimation(self, animation_data, sync)
    }

    /* pub fn get_animation_name(&self, index: isize, lib: &mut String, name: &mut String) {
        functions::players::GetAnimationName(lib, name)
    } */

    pub fn edit_attached_object(&self, index: isize) {
        functions::players::EditAttachedObject(self, index)
    }

    pub fn enable_camera_target(&self, enable: bool) {
        functions::players::EnablePlayerCameraTarget(self, enable)
    }

    pub fn enable_stunt_bonus(&self, enable: bool) {
        functions::players::EnableStuntBonusForPlayer(self, enable)
    }

    /*  pub fn enable_stunt_bonus_for_all(&self, enable: bool) {
        todo!()
    } */

    pub fn get_ammo(&self) -> isize {
        functions::players::GetPlayerAmmo(self)
    }

    pub fn get_animation_index(&self) -> isize {
        functions::players::GetPlayerAnimationIndex(self)
    }

    pub fn get_facing_angle(&self) -> f32 {
        functions::players::GetPlayerFacingAngle(self)
    }

    pub fn get_ip(&self) -> String {
        let mut ip = String::new();
        functions::players::GetPlayerIp(self, &mut ip);
        ip
    }

    pub fn get_special_action(&self) -> PlayerSpecialAction {
        functions::players::GetPlayerSpecialAction(self)
    }

    pub fn get_vehicle_id(&self) -> isize {
        functions::players::GetPlayerVehicleID(self)
    }

    pub fn get_vehicle_seat(&self) -> isize {
        functions::players::GetPlayerVehicleSeat(self)
    }

    pub fn get_weapon_data(&self, slot: isize) -> WeaponSlotData {
        let mut weapon: WeaponSlotData = WeaponSlotData { ammo: 0, id: 0 };
        functions::players::GetPlayerWeaponData(self, slot, &mut weapon);
        weapon
    }

    pub fn get_weapon_state(&self) -> isize {
        functions::players::GetPlayerWeaponState(self)
    }

    pub fn interpolate_camera_pos(
        &self,
        from: Vector3,
        to: Vector3,
        time: isize,
        cut: PlayerCameraCutType,
    ) {
        functions::players::InterpolateCameraPos(self, from, to, time, cut)
    }

    pub fn interpolate_camera_look_at(
        &self,
        from: Vector3,
        to: Vector3,
        time: isize,
        cut: PlayerCameraCutType,
    ) -> bool {
        functions::players::InterpolateCameraLookAt(self, from, to, time, cut)
    }

    pub fn is_attached_object_slot_used(&self, index: isize) -> bool {
        functions::players::IsPlayerAttachedObjectSlotUsed(self, index)
    }

    pub fn attach_camera_to_object(&self, object: Object) {
        functions::players::AttachCameraToObject(self, &object)
    }

    pub fn attach_camera_to_player_object(&self, object: PlayerObject) {
        functions::players::AttachCameraToPlayerObject(self, &object)
    }

    pub fn get_aim_data(&self) -> PlayerAimData {
        functions::players::GetPlayerAimData(self)
    }

    pub fn get_keys(
        &self,
        keys: &mut isize,
        updown: &mut isize,
        leftright: &mut isize,
    ) -> PlayerKeyData {
        functions::players::GetPlayerKeys(self, keys, updown, leftright)
    }

    pub fn get_surfing_data(&self) -> PlayerSurfingData {
        functions::players::GetPlayerSurfingData(self)
    }

    pub fn get_target_player(&self) -> Player {
        functions::players::GetPlayerTargetPlayer(self)
    }

    pub fn get_target_actor(&self) -> Actor {
        functions::players::GetPlayerTargetActor(self)
    }

    pub fn is_in_vehicle(&self, target_vehicle: Vehicle) -> bool {
        functions::players::IsPlayerInVehicle(self, &target_vehicle)
    }

    pub fn is_in_any_vehicle(&self) -> bool {
        functions::players::IsPlayerInAnyVehicle(self)
    }

    pub fn is_in_range_of_point(&self, range: f32, coord: Vector3) -> bool {
        functions::players::IsPlayerInRangeOfPoint(self, range, coord)
    }

    pub fn play_crime_report(&self, suspect: Player, crime: isize) -> bool {
        functions::players::PlayCrimeReportForPlayer(self, &suspect, crime)
    }

    pub fn remove_attached_object(&self, index: isize) {
        functions::players::RemovePlayerAttachedObject(self, index)
    }
    //pub fn SetPlayerAttachedObject(&self, index: isize, attachment: ObjectAttachmentSlotData) { todo!() }
    //pub fn GetPlayerAttachedObject(&self, index: isize) -> ObjectAttachmentSlotData { todo!() }
    pub fn set_facing_angle(&self, angle: f32) {
        functions::players::SetPlayerFacingAngle(self, angle)
    }

    pub fn set_marker_for_player(&self, other: Player, colour: Colour) {
        functions::players::SetPlayerMarkerForPlayer(self, &other, colour)
    }

    pub fn get_marker_for_player(&self, other: Player) -> isize {
        functions::players::GetPlayerMarkerForPlayer(self, &other)
    }

    pub fn allow_teleport(&self, allow: bool) {
        functions::players::AllowPlayerTeleport(self, allow)
    }

    pub fn is_teleport_allowed(&self) -> bool {
        functions::players::IsPlayerTeleportAllowed(self)
    }

    pub fn set_remote_vehicle_collisions(&self, collide: bool) {
        functions::players::SetRemoteVehicleCollisions(self, collide)
    }

    pub fn select_text_draw(&self, hover_colour: Colour) {
        functions::players::SelectTextDraw(self, hover_colour)
    }

    pub fn cancel_select_text_draw(&self) {
        functions::players::CancelSelectTextDraw(self)
    }

    pub fn send_client_check(
        &self,
        action_type: isize,
        address: isize,
        offset: isize,
        count: isize,
    ) {
        functions::players::SendClientCheck(self, action_type, address, offset, count)
    }

    pub fn spawn(&self) {
        functions::players::SpawnPlayer(self)
    }

    pub fn gpci(&self) -> String {
        let mut output = String::new();
        functions::players::gpci(self, &mut output);
        output
    }

    pub fn is_admin(&self) -> bool {
        functions::players::IsPlayerAdmin(self)
    }
    pub fn kick(&self) {
        functions::players::Kick(self)
    }

    pub fn game_text(&self, msg: &str, time: isize, style: isize) {
        functions::players::GameTextForPlayer(self, msg, time, style)
    }

    pub fn hide_game_text(&self, style: isize) {
        functions::players::HideGameTextForPlayer(self, style)
    }

    pub fn has_game_text(&self, style: isize) -> bool {
        functions::players::HasGameText(self, style)
    }

    pub fn get_game_text(
        &self,
        style: isize,
        message: &mut String,
        time: &mut isize,
        remaining: &mut isize,
    ) -> bool {
        functions::players::GetGameText(self, style, message, time, remaining)
    }

    pub fn ban(&self) {
        functions::players::Ban(self)
    }

    pub fn ban_ex(&self, msg: &str) {
        functions::players::BanEx(self, msg)
    }

    pub fn send_death_message_to_player(&self, killer: Player, killee: Player, weapon: isize) {
        functions::players::SendDeathMessageToPlayer(self, &killer, &killee, weapon)
    }

    pub fn send_message_to_player(&self, sender: Player, message: &str) {
        functions::players::SendPlayerMessageToPlayer(self, &sender, message)
    }

    pub fn get_version(&self, output: &mut String) {
        functions::players::GetPlayerVersion(self, output)
    }

    pub fn get_skill_level(&self, skill: isize) -> isize {
        functions::players::GetPlayerSkillLevel(self, skill)
    }

    pub fn get_spectate_id(&self) -> isize {
        functions::players::GetPlayerSpectateID(self)
    }

    pub fn get_spectate_data(&self) -> PlayerSpectateData {
        functions::players::GetPlayerSpectateData(self)
    }

    pub fn get_raw_ip(&self) -> isize {
        functions::players::GetPlayerRawIp(self)
    }

    pub fn set_gravity(&self, gravity: f32) {
        functions::players::SetPlayerGravity(self, gravity)
    }

    pub fn get_gravity(&self) -> f32 {
        functions::players::GetPlayerGravity(self)
    }

    pub fn set_admin(&self, set: bool) {
        functions::players::SetPlayerAdmin(self, set)
    }

    pub fn is_spawned(&self) -> bool {
        functions::players::IsPlayerSpawned(self)
    }

    pub fn is_controllable(&self) -> bool {
        functions::players::IsPlayerControllable(self)
    }

    pub fn is_camera_target_enabled(&self) -> bool {
        functions::players::IsPlayerCameraTargetEnabled(self)
    }

    pub fn toggle_ghost_mode(&self, toggle: bool) {
        functions::players::TogglePlayerGhostMode(self, toggle)
    }

    pub fn get_ghost_mode(&self) -> bool {
        functions::players::GetPlayerGhostMode(self)
    }

    pub fn allow_weapons(&self, allow: bool) {
        functions::players::AllowPlayerWeapons(self, allow)
    }

    pub fn are_weapons_allowed(&self) -> bool {
        functions::players::ArePlayerWeaponsAllowed(self)
    }

    pub fn is_using_official_client(&self) -> bool {
        functions::players::IsPlayerUsingOfficialClient(self)
    }

    pub fn get_animation_data(&self) -> PlayerAnimationData {
        functions::players::GetPlayerAnimationData(self)
    }

    pub fn is_in_drive_by_mode(&self) -> bool {
        functions::players::IsPlayerInDriveByMode(self)
    }

    pub fn is_cuffed(&self) -> bool {
        functions::players::IsPlayerCuffed(self)
    }

    pub fn get_custom_skin(&self) -> isize {
        functions::players::GetPlayerCustomSkin(self)
    }

    pub fn redirect_download(&self,url:&str) -> bool {
        functions::players::RedirectDownload(self,url)
    }
}

unsafe impl Send for Player {}
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
pub struct WeaponSlotData {
    id: u8,
    ammo: u32,
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
