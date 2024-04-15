use omprs_codegen::native;

native!(GetPlayerName,playerid: isize, name: mut str, -> isize );
native!(SendClientMessage,playerid: isize, colour: usize, message: str, -> isize);
native!(SendClientMessageToAll,colour:usize, message:str, -> bool);
native!(SetPlayerCameraPos,playerid:isize, x:f32, y:f32, z:f32, -> bool);
native!(SetPlayerDrunkLevel,playerid:isize, level:isize, -> bool);
native!(SetPlayerInterior,playerid:isize, interiorid:isize, -> bool);
native!(SetPlayerWantedLevel,playerid:isize, level:isize, -> bool);
native!(SetPlayerWeather,playerid:isize, weatherid:isize, -> bool);
native!(GetPlayerWeather,playerid:isize, -> isize);
native!(SetPlayerSkin,playerid:isize, skinid:isize, -> bool);
native!(SetPlayerShopName,playerid:isize, shopname:str, -> bool);
native!(GivePlayerMoney,playerid:isize, amount:isize, -> bool);
native!(SetPlayerCameraLookAt,playerid:isize, x:f32, y:f32, z:f32, cut:isize, -> bool);
native!(SetCameraBehindPlayer,playerid:isize, -> bool);
native!(CreateExplosionForPlayer,playerid:isize, x:f32, y:f32, z:f32, category:isize, radius:f32, -> bool);
native!(CreateExplosion,x:f32, y:f32, z:f32, category:isize, radius:f32, -> bool);
native!(PlayAudioStreamForPlayer,playerid:isize, url:str, x:f32, y:f32, z:f32, distance:f32, usePos:bool, -> bool);
native!(StopAudioStreamForPlayer,playerid:isize, -> bool);
native!(SendDeathMessage,killerid:isize, playerid:isize, weapon:isize, -> bool);
native!(TogglePlayerWidescreen,playerid:isize, enable:bool, -> bool);
native!(IsPlayerWidescreenToggled,playerid:isize, -> bool);
native!(SetPlayerHealth,playerid:isize, health:f32, -> bool);
native!(GetPlayerHealth,playerid:isize, health:mut f32, -> bool);
native!(SetPlayerArmour,playerid:isize, armour:f32, -> bool);
native!(GetPlayerArmour,playerid:isize, armour:mut f32, -> bool);
native!(SetPlayerTeam,playerid:isize, teamid:isize, -> bool);
native!(GetPlayerTeam,playerid:isize, -> isize);
native!(SetPlayerScore,playerid:isize, score:isize, -> bool);
native!(GetPlayerScore,playerid:isize, -> isize);
native!(GetPlayerSkin,playerid:isize, -> isize);
native!(SetPlayerColor,playerid:isize, colour:usize, -> bool);
native!(GetPlayerColor,playerid:isize, -> isize);
native!(GetDefaultPlayerColour,playerid:isize, -> isize);
native!(GetPlayerDrunkLevel,playerid:isize, -> isize);
native!(GivePlayerWeapon,playerid:isize, weaponid:isize, ammo:isize, -> bool);
native!(RemovePlayerWeapon,playerid:isize, weaponid:isize, -> bool);
native!(GetPlayerMoney,playerid:isize, -> isize);
native!(ResetPlayerMoney,playerid:isize, -> bool);
native!(SetPlayerName,playerid:isize, name:str, -> isize);
native!(GetPlayerState,playerid:isize, -> isize);
native!(GetPlayerPing,playerid:isize, -> isize);
native!(GetPlayerWeapon,playerid:isize, -> isize);
native!(SetPlayerTime,playerid:isize, hour:isize, minute:isize, -> bool);
native!(GetPlayerTime,playerid:isize, hour:mut isize, minute:mut isize, -> bool);
native!(TogglePlayerClock,playerid:isize, enable:bool, -> bool);
native!(PlayerHasClockEnabled,playerid:isize, -> bool);
native!(ForceClassSelection,playerid:isize, -> bool);
native!(GetPlayerWantedLevel,playerid:isize, -> isize);
native!(SetPlayerFightingStyle,playerid:isize, style:isize, -> bool);
native!(GetPlayerFightingStyle,playerid:isize, -> isize);
native!(SetPlayerVelocity,playerid:isize, x:f32, y:f32, z:f32, -> bool);
native!(GetPlayerVelocity,playerid:isize, x:mut f32, y:mut f32, z:mut f32, -> bool);
native!(GetPlayerCameraPos,playerid:isize, x:mut f32, y:mut f32, z:mut f32, -> bool);
native!(GetPlayerDistanceFromPoint,playerid:isize, x:f32, y:f32, z:f32, -> f32);
native!(GetPlayerInterior,playerid:isize, -> isize);
native!(SetPlayerPos,playerid:isize, x:f32, y:f32, z:f32, -> bool);
native!(GetPlayerPos,playerid:isize, x:mut f32, y:mut f32, z:mut f32, -> bool);
native!(GetPlayerVirtualWorld,playerid:isize, -> isize);
native!(IsPlayerNPC,playerid:isize, -> bool);
native!(IsPlayerStreamedIn,playerid:isize, otherid:isize, -> bool);
native!(PlayerPlaySound,playerid:isize, sound:isize, x:f32, y:f32, z:f32, -> bool);
native!(PlayerSpectatePlayer,playerid:isize, targetid:isize, mode:isize, -> bool);
native!(PlayerSpectateVehicle,playerid:isize, targetvehicleid:isize, mode:isize, -> bool);
native!(SetPlayerVirtualWorld,playerid:isize, vw:isize, -> bool);
native!(SetPlayerWorldBounds,playerid:isize, xMax:f32, xMin:f32, yMax:f32, yMin:f32, -> bool);
native!(ClearPlayerWorldBounds,playerid:isize, -> bool);
native!(GetPlayerWorldBounds,playerid:isize, xMax:mut f32, xMin:mut f32, yMax:mut f32, yMin:mut f32, -> bool);
native!(ClearAnimations,playerid:isize, synccategory:isize, -> bool);
native!(GetPlayerLastShotVectors,playerid:isize, fOriginX:mut f32, fOriginY:mut f32, fOriginZ:mut f32, fHitPosX:mut f32, fHitPosY:mut f32, fHitPosZ:mut f32, -> bool);
native!(GetPlayerCameraTargetPlayer,playerid:isize, -> isize);
native!(GetPlayerCameraTargetActor,playerid:isize, -> isize);
native!(GetPlayerCameraTargetObject,playerid:isize, -> isize);
native!(GetPlayerCameraTargetVehicle,playerid:isize, -> isize);
native!(IsPlayerConnected,playerid:isize, -> bool);
native!(PutPlayerInVehicle,playerid:isize, vehicleid:isize, seatID:isize, -> bool);
native!(RemoveBuildingForPlayer,playerid:isize, model:isize, x:f32, y:f32, z:f32, radius:f32, -> bool);
native!(GetPlayerBuildingsRemoved,playerid:isize, -> isize);
native!(RemovePlayerFromVehicle,playerid:isize, force:bool, -> bool);
native!(RemovePlayerMapIcon,playerid:isize, iconID:isize, -> bool);
native!(SetPlayerMapIcon,playerid:isize, iconID:isize, x:f32,y:f32,z:f32, category:isize, colour:usize, style:isize, -> bool);
native!(ResetPlayerWeapons,playerid:isize, -> bool);
native!(SetPlayerAmmo,playerid:isize, id:isize, ammo:isize, -> bool);
native!(SetPlayerArmedWeapon,playerid:isize, weapon:isize, -> bool);
native!(SetPlayerChatBubble,playerid:isize, text:mut str, colour:usize, drawdistance:f32, expiretime:isize, -> bool);
native!(SetPlayerPosFindZ,playerid:isize, x:f32, y:f32, z:f32, -> bool);
native!(SetPlayerSkillLevel,playerid:isize, weapon:isize, level:isize, -> bool);
native!(SetPlayerSpecialAction,playerid:isize, action:isize, -> bool);
native!(ShowPlayerNameTagForPlayer,playerid:isize, otherid:isize, enable:bool, -> bool);
native!(TogglePlayerControllable,playerid:isize, enable:bool, -> bool);
native!(TogglePlayerSpectating,playerid:isize, enable:bool, -> bool);
native!(ApplyAnimation,playerid:isize, animlib:str, animname:str, delta:f32, looping:bool, lockX:bool, lockY:bool, freeze:bool, time:isize, sync:isize, -> bool);
native!(GetAnimationName,index:isize, lib:mut str, name:mut str,-> bool);
native!(EditAttachedObject,playerid:isize, index:isize, -> bool);
native!(EnablePlayerCameraTarget,playerid:isize, enable:bool, -> bool);
native!(EnableStuntBonusForPlayer,playerid:isize, enable:bool, -> bool);
native!(EnableStuntBonusForAll,enable:bool, -> bool);
native!(GetPlayerAmmo,playerid:isize, -> isize);
native!(GetPlayerAnimationIndex,playerid:isize, -> isize);
native!(GetPlayerFacingAngle,playerid:isize,angle:mut f32, -> bool);
native!(GetPlayerIp,playerid:isize, ip:mut str, -> isize);
native!(GetPlayerSpecialAction,playerid:isize, -> isize);
native!(GetPlayerVehicleID,playerid:isize, -> isize);
native!(GetPlayerVehicleSeat,playerid:isize, -> isize);
native!(GetPlayerWeaponData,playerid:isize, slot:isize, weaponid:mut isize, ammo:mut isize, -> bool);
native!(GetPlayerWeaponState,playerid:isize, -> isize);
native!(InterpolateCameraPos,playerid:isize, FromX:f32, FromY:f32, FromZ:f32, ToX:f32, ToY:f32, ToZ:f32, time:isize, cut:isize, -> bool);
native!(InterpolateCameraLookAt,playerid:isize, FromX:f32, FromY:f32, FromZ:f32, ToX:f32, ToY:f32, ToZ:f32, time:isize, cut:isize, -> bool);
native!(IsPlayerAttachedObjectSlotUsed,playerid:isize, index:isize, -> bool);
native!(AttachCameraToObject,playerid:isize, objectid:isize, -> bool);
native!(AttachCameraToPlayerObject,playerid:isize, objectid:isize, -> bool);
native!(GetPlayerCameraAspectRatio,playerid:isize, -> f32);
native!(GetPlayerCameraFrontVector,playerid:isize, x:mut f32, y:mut f32, z:mut f32, -> bool);
native!(GetPlayerCameraMode,playerid:isize, -> isize);
native!(GetPlayerKeys,playerid:isize, keys:mut isize, updown:mut isize, leftright:mut isize, -> bool);
native!(GetPlayerSurfingVehicleID,playerid:isize, -> isize);
native!(GetPlayerSurfingObjectID,playerid:isize, -> isize);
native!(GetPlayerTargetPlayer,playerid:isize, -> isize);
native!(GetPlayerTargetActor,playerid:isize, -> isize);
native!(IsPlayerInVehicle,playerid:isize, vehicleid:isize, -> bool);
native!(IsPlayerInAnyVehicle,playerid:isize, -> bool);
native!(IsPlayerInRangeOfPoint,playerid:isize, range:f32, x:f32, y:f32, z:f32, -> bool);
native!(PlayCrimeReportForPlayer,playerid:isize, suspectid:isize, crime:isize, -> bool);
native!(RemovePlayerAttachedObject,playerid:isize, index:isize, -> bool);
native!(SetPlayerAttachedObject,playerid:isize, index:isize, modelid:isize, bone:isize, fOffsetX:f32, fOffsetY:f32, fOffsetZ:f32, fRotX:f32, fRotY:f32, fRotZ:f32, fScaleX:f32, fScaleY:f32, fScaleZ:f32, materialcolor1:isize, materialcolor2:isize, -> bool);
native!(GetPlayerAttachedObject,playerid:isize, index:isize, modelid:mut isize, bone:mut isize, fOffsetX:mut f32, fOffsetY:mut f32, fOffsetZ:mut f32, fRotX:mut f32, fRotY:mut f32, fRotZ:mut f32, fScaleX:mut f32, fScaleY:mut f32, fScaleZ:mut f32, materialcolor1:mut isize, materialcolor2:mut isize, -> bool);
native!(SetPlayerFacingAngle,playerid:isize, angle:f32, -> bool);
native!(SetPlayerMarkerForPlayer,playerid:isize, otherid:isize, colour:usize, -> bool);
native!(GetPlayerMarkerForPlayer,playerid:isize, otherid:isize, -> isize);
native!(AllowPlayerTeleport,playerid:isize, allow:bool, -> bool);
native!(IsPlayerTeleportAllowed,playerid:isize, -> bool);
native!(DisableRemoteVehicleCollisions,playerid:isize, disable:bool, -> bool);
native!(GetPlayerCameraZoom,playerid:isize, -> f32);
native!(SelectTextDraw,playerid:isize, hoverColour:usize, -> bool);
native!(CancelSelectTextDraw,playerid:isize, -> bool);
native!(SendClientCheck,playerid:isize, actioncategory:isize, address:isize, offset:isize, count:isize, -> bool);
native!(SpawnPlayer,playerid:isize, -> bool);
native!(gpci,playerid:isize, output:mut str,-> isize);
native!(IsPlayerAdmin,playerid:isize, -> bool);
native!(Kick,playerid:isize, -> bool);
native!(GameTextForPlayer,playerid:isize, message:mut str, time:isize, style:isize, -> bool);
native!(HideGameTextForPlayer,playerid:isize, style:isize, -> bool);
native!(HasGameText,playerid:isize, style:isize, -> bool);
native!(GetGameText,playerid:isize, style:isize, message:mut str,time:isize, remaining:isize, -> bool);
native!(Ban,playerid:isize, -> bool);
native!(BanEx,playerid:isize, message:mut str, -> bool);
native!(SendDeathMessageToPlayer,playerid:isize, killerid:isize, killeeid:isize, weapon:isize, -> bool);
native!(SendPlayerMessageToPlayer,playerid:isize, senderid:isize, message:str, -> bool);
native!(SendPlayerMessageToPlayerf,playerid:isize, senderid:isize, message:str, -> bool);
native!(GetPlayerVersion,playerid:isize, version:mut str, -> isize);
native!(GetPlayerSkillLevel,playerid:isize, skill:isize, -> isize);
native!(GetPlayerZAim,playerid:isize, -> f32);
native!(GetPlayerSurfingOffsets,playerid:isize, x:mut f32, y:mut f32, z:mut f32, -> bool);
native!(GetPlayerRotationQuat,playerid:isize, w:mut f32, x:mut f32, y:mut f32, z:mut f32, -> bool);
native!(GetPlayerSpectateID,playerid:isize, -> isize);
native!(GetPlayerSpectateType,playerid:isize, -> isize);
native!(GetPlayerRawIp,playerid:isize, -> isize);
native!(SetPlayerGravity,playerid:isize, gravity:f32, -> bool);
native!(GetPlayerGravity,playerid:isize, -> f32);
native!(SetPlayerAdmin,playerid:isize, set:bool, -> bool);
native!(IsPlayerSpawned,playerid:isize, -> bool);
native!(IsPlayerControllable,playerid:isize, -> bool);
native!(IsPlayerCameraTargetEnabled,playerid:isize, -> bool);
native!(TogglePlayerGhostMode,playerid:isize, toggle:bool, -> bool);
native!(GetPlayerGhostMode,playerid:isize, -> bool);
native!(AllowPlayerWeapons,playerid:isize, allow:bool, -> bool);
native!(ArePlayerWeaponsAllowed,playerid:isize, -> bool);
native!(IsPlayerUsingOfficialClient,playerid:isize, -> isize);
native!(GetPlayerAnimFlags,playerid:isize, -> isize);
native!(GetPlayerAnimationFlags,playerid:isize, -> isize);
native!(IsPlayerInDriveByMode,playerid:isize, -> bool);
native!(IsPlayerCuffed,playerid:isize, -> bool);

pub fn load_player_functions() {
    load_function!(GetPlayerName);
    load_function!(SendClientMessage);
    load_function!(SendClientMessageToAll);
    load_function!(SetPlayerCameraPos);
    load_function!(SetPlayerDrunkLevel);
    load_function!(SetPlayerInterior);
    load_function!(SetPlayerWantedLevel);
    load_function!(SetPlayerWeather);
    load_function!(GetPlayerWeather);
    load_function!(SetPlayerSkin);
    load_function!(SetPlayerShopName);
    load_function!(GivePlayerMoney);
    load_function!(SetPlayerCameraLookAt);
    load_function!(SetCameraBehindPlayer);
    load_function!(CreateExplosionForPlayer);
    load_function!(CreateExplosion);
    load_function!(PlayAudioStreamForPlayer);
    load_function!(StopAudioStreamForPlayer);
    load_function!(SendDeathMessage);
    load_function!(TogglePlayerWidescreen);
    load_function!(IsPlayerWidescreenToggled);
    load_function!(SetPlayerHealth);
    load_function!(GetPlayerHealth);
    load_function!(SetPlayerArmour);
    load_function!(GetPlayerArmour);
    load_function!(SetPlayerTeam);
    load_function!(GetPlayerTeam);
    load_function!(SetPlayerScore);
    load_function!(GetPlayerScore);
    load_function!(GetPlayerSkin);
    load_function!(SetPlayerColor);
    load_function!(GetPlayerColor);
    load_function!(GetDefaultPlayerColour);
    load_function!(GetPlayerDrunkLevel);
    load_function!(GivePlayerWeapon);
    load_function!(RemovePlayerWeapon);
    load_function!(GetPlayerMoney);
    load_function!(ResetPlayerMoney);
    load_function!(SetPlayerName);
    load_function!(GetPlayerState);
    load_function!(GetPlayerPing);
    load_function!(GetPlayerWeapon);
    load_function!(SetPlayerTime);
    load_function!(GetPlayerTime);
    load_function!(TogglePlayerClock);
    load_function!(PlayerHasClockEnabled);
    load_function!(ForceClassSelection);
    load_function!(GetPlayerWantedLevel);
    load_function!(SetPlayerFightingStyle);
    load_function!(GetPlayerFightingStyle);
    load_function!(SetPlayerVelocity);
    load_function!(GetPlayerVelocity);
    load_function!(GetPlayerCameraPos);
    load_function!(GetPlayerDistanceFromPoint);
    load_function!(GetPlayerInterior);
    load_function!(SetPlayerPos);
    load_function!(GetPlayerPos);
    load_function!(GetPlayerVirtualWorld);
    load_function!(IsPlayerNPC);
    load_function!(IsPlayerStreamedIn);
    load_function!(PlayerPlaySound);
    load_function!(PlayerSpectatePlayer);
    load_function!(PlayerSpectateVehicle);
    load_function!(SetPlayerVirtualWorld);
    load_function!(SetPlayerWorldBounds);
    load_function!(ClearPlayerWorldBounds);
    load_function!(GetPlayerWorldBounds);
    load_function!(ClearAnimations);
    load_function!(GetPlayerLastShotVectors);
    load_function!(GetPlayerCameraTargetPlayer);
    load_function!(GetPlayerCameraTargetActor);
    load_function!(GetPlayerCameraTargetObject);
    load_function!(GetPlayerCameraTargetVehicle);
    load_function!(IsPlayerConnected);
    load_function!(PutPlayerInVehicle);
    load_function!(RemoveBuildingForPlayer);
    load_function!(GetPlayerBuildingsRemoved);
    load_function!(RemovePlayerFromVehicle);
    load_function!(RemovePlayerMapIcon);
    load_function!(SetPlayerMapIcon);
    load_function!(ResetPlayerWeapons);
    load_function!(SetPlayerAmmo);
    load_function!(SetPlayerArmedWeapon);
    load_function!(SetPlayerChatBubble);
    load_function!(SetPlayerPosFindZ);
    load_function!(SetPlayerSkillLevel);
    load_function!(SetPlayerSpecialAction);
    load_function!(ShowPlayerNameTagForPlayer);
    load_function!(TogglePlayerControllable);
    load_function!(TogglePlayerSpectating);
    load_function!(ApplyAnimation);
    load_function!(GetAnimationName);
    load_function!(EditAttachedObject);
    load_function!(EnablePlayerCameraTarget);
    load_function!(EnableStuntBonusForPlayer);
    load_function!(EnableStuntBonusForAll);
    load_function!(GetPlayerAmmo);
    load_function!(GetPlayerAnimationIndex);
    load_function!(GetPlayerFacingAngle);
    load_function!(GetPlayerIp);
    load_function!(GetPlayerSpecialAction);
    load_function!(GetPlayerVehicleID);
    load_function!(GetPlayerVehicleSeat);
    load_function!(GetPlayerWeaponData);
    load_function!(GetPlayerWeaponState);
    load_function!(InterpolateCameraPos);
    load_function!(InterpolateCameraLookAt);
    load_function!(IsPlayerAttachedObjectSlotUsed);
    load_function!(AttachCameraToObject);
    load_function!(AttachCameraToPlayerObject);
    load_function!(GetPlayerCameraAspectRatio);
    load_function!(GetPlayerCameraFrontVector);
    load_function!(GetPlayerCameraMode);
    load_function!(GetPlayerKeys);
    load_function!(GetPlayerSurfingVehicleID);
    load_function!(GetPlayerSurfingObjectID);
    load_function!(GetPlayerTargetPlayer);
    load_function!(GetPlayerTargetActor);
    load_function!(IsPlayerInVehicle);
    load_function!(IsPlayerInAnyVehicle);
    load_function!(IsPlayerInRangeOfPoint);
    load_function!(PlayCrimeReportForPlayer);
    load_function!(RemovePlayerAttachedObject);
    load_function!(SetPlayerAttachedObject);
    load_function!(GetPlayerAttachedObject);
    load_function!(SetPlayerFacingAngle);
    load_function!(SetPlayerMarkerForPlayer);
    load_function!(GetPlayerMarkerForPlayer);
    load_function!(AllowPlayerTeleport);
    load_function!(IsPlayerTeleportAllowed);
    load_function!(DisableRemoteVehicleCollisions);
    load_function!(GetPlayerCameraZoom);
    load_function!(SelectTextDraw);
    load_function!(CancelSelectTextDraw);
    load_function!(SendClientCheck);
    load_function!(SpawnPlayer);
    load_function!(gpci);
    load_function!(IsPlayerAdmin);
    load_function!(Kick);
    load_function!(GameTextForPlayer);
    load_function!(HideGameTextForPlayer);
    load_function!(HasGameText);
    load_function!(GetGameText);
    load_function!(Ban);
    load_function!(BanEx);
    load_function!(SendDeathMessageToPlayer);
    load_function!(SendPlayerMessageToPlayer);
    load_function!(SendPlayerMessageToPlayerf);
    load_function!(GetPlayerVersion);
    load_function!(GetPlayerSkillLevel);
    load_function!(GetPlayerZAim);
    load_function!(GetPlayerSurfingOffsets);
    load_function!(GetPlayerRotationQuat);
    load_function!(GetPlayerSpectateID);
    load_function!(GetPlayerSpectateType);
    load_function!(GetPlayerRawIp);
    load_function!(SetPlayerGravity);
    load_function!(GetPlayerGravity);
    load_function!(SetPlayerAdmin);
    load_function!(IsPlayerSpawned);
    load_function!(IsPlayerControllable);
    load_function!(IsPlayerCameraTargetEnabled);
    load_function!(TogglePlayerGhostMode);
    load_function!(GetPlayerGhostMode);
    load_function!(AllowPlayerWeapons);
    load_function!(ArePlayerWeaponsAllowed);
    load_function!(IsPlayerUsingOfficialClient);
    load_function!(GetPlayerAnimFlags);
    load_function!(GetPlayerAnimationFlags);
    load_function!(IsPlayerInDriveByMode);
    load_function!(IsPlayerCuffed);
}

#[repr(C)]
pub enum MapIconStyle{
	MapIconStyleLocal,
	MapIconStyleGlobal,
	MapIconStyleLocalCheckpoint,
	MapIconStyleGlobalCheckpoint
}

#[repr(u8)]
pub enum ClientVersion
{
	ClientVersionSamp037,
	ClientVersionSamp03dl,
	ClientVersionOpenmp
}

#[repr(C)]
pub enum PlayerCameraCutType
{
	PlayerCameraCutTypeCut,
	PlayerCameraCutTypeMove
}

/// The player's name status returned when updating their name
#[repr(C)]
pub enum EPlayerNameStatus
{
	Updated, // The name has successfully been updated
	Taken, // The name is already taken by another player
	Invalid // The name is invalid
}

pub enum PlayerAnimationSyncType
{
	PlayerAnimationSyncTypeNoSync,
	PlayerAnimationSyncTypeSync,
	PlayerAnimationSyncTypeSyncOthers
}

#[repr(C)]
pub struct WeaponSlotData {
    id:u8,
	ammo:u32,
}

#[repr(C)]
pub struct PlayerAnimationData{
	ID:u16,
	flags:u16,

}
#[repr(C)]
pub enum PlayerFightingStyle
{
	PlayerFightingStyleNormal = 4,
	PlayerFightingStyleBoxing = 5,
	PlayerFightingStyleKungFu = 6,
	PlayerFightingStyleKneeHead = 7,
	PlayerFightingStyleGrabKick = 15,
	PlayerFightingStyleElbow = 16
}

#[repr(C)]
pub enum PlayerState
{
	PlayerStateNone = 0,
	PlayerStateOnFoot = 1,
	PlayerStateDriver = 2,
	PlayerStatePassenger = 3,
	PlayerStateExitVehicle = 4,
	PlayerStateEnterVehicleDriver = 5,
	PlayerStateEnterVehiclePassenger = 6,
	PlayerStateWasted = 7,
	PlayerStateSpawned = 8,
	PlayerStateSpectating = 9
}

#[repr(C)]
pub enum PlayerWeaponSkill
{
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
	PlayerWeaponSkillSniper
}

#[repr(C)]
pub enum PlayerSpecialAction
{
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
	SpecialActionPissing = 68
}

#[repr(C)]
pub struct PlayerSurfingData
{
	surftype: isize,
	ID:isize,
	offset:Vector3,
}

#[repr(C)]
pub struct PlayerKeyData
{
	// todo fill with union
	keys:u32,
	upDown:i16,
	leftRight:i16,
}

#[repr(C)]
pub struct PlayerBulletData
{
	origin:Vector3,
	hitPos:Vector3,
	offset:Vector3,
	weapon:u8,
	hitType:PlayerBulletHitType,
	hitID:u16,
}
#[repr(C)]
pub enum PlayerBulletHitType
{
	PlayerBulletHitTypeNone,
	PlayerBulletHitTypePlayer = 1,
	PlayerBulletHitTypeVehicle = 2,
	PlayerBulletHitTypeObject = 3,
	PlayerBulletHitTypePlayerObject = 4,
}
