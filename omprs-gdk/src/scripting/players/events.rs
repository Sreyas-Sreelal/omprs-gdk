use super::{BodyPart, Player, PlayerBulletData, PlayerClickSource, PlayerState};
use crate::objects::{Object, PlayerObject};
use crate::types::network::PeerDisconnectReason;
use crate::types::vector::Vector3;
use crate::vehicles::Vehicle;
use omprs_codegen::callback;

callback!(OnPlayerConnect, player:Player);
callback!(OnIncomingConnection, player:Player, ip:String, port:u16);
callback!(OnPlayerDisconnect, player:Player, reason:PeerDisconnectReason);
callback!(OnPlayerStreamIn, player:Player, for_player:Player);
callback!(OnPlayerStreamOut, player:Player, for_player:Player);
callback!(OnPlayerRequestSpawn, player:Player,->bool);
callback!(OnPlayerSpawn, player:Player);
callback!(OnPlayerText, player:Player, text:String,->bool);
callback!(OnPlayerCommandText, player:Player, cmd:String,->bool);
callback!(OnPlayerInteriorChange, player:Player, new_interior:usize, old_interior:usize);
callback!(OnPlayerStateChange, player:Player, new_state:PlayerState,olde_state:PlayerState);
callback!(OnPlayerKeyStateChange, player:Player, new_keys:u32,old_keys:u32);
callback!(OnPlayerDeath, player:Player, killer:Option<Player>, reason:isize);
callback!(OnPlayerTakeDamage, player:Player, from:Option<Player>, amount:f32, weapon:usize, part:BodyPart);
callback!(OnPlayerGiveDamage, player:Player, to:Player, amount:f32, weapon:usize, part:BodyPart);
callback!(OnPlayerClickMap, player:Player, pos:Vector3);
callback!(OnPlayerClickPlayer, player:Player, clicked:Player, source:PlayerClickSource);
callback!(OnClientCheckResponse, player:Player, action_type:isize, address:isize, results:isize);
callback!(OnPlayerUpdate, player:Player,now:isize,->bool);
callback!(onPlayerShotMissed, player: Player, bulletData: PlayerBulletData,-> bool);
callback!(onPlayerShotPlayer, player: Player, target: Player, bulletData: PlayerBulletData,-> bool);
callback!(onPlayerShotVehicle, player: Player, target: Vehicle, bulletData: PlayerBulletData,-> bool);
callback!(onPlayerShotObject,player: Player, target: Object, bulletData: PlayerBulletData,-> bool);
callback!(onPlayerShotPlayerObject,player: Player, target: PlayerObject, bulletData: PlayerBulletData,-> bool);
callback!(onPlayerScoreChange,player: Player, score: isize);
callback!(onPlayerNameChange,player: Player, oldName: String);
