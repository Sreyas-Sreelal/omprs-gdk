use omp_codegen::callback;

use crate::players::Player;

use super::{UnoccupiedVehicleUpdate, Vehicle};

callback!(onVehicleStreamIn, vehicle: Vehicle, player: Player);
callback!(onVehicleStreamOut,  vehicle: Vehicle, player: Player);
callback!(onVehicleDeath, vehicle: Vehicle, player: Player);
callback!(onPlayerEnterVehicle, player: Player, vehicle: Vehicle,passenger: bool);
callback!(onPlayerExitVehicle,  player: Player, vehicle: Vehicle);
callback!(onVehicleDamageStatusUpdate, vehicle: Vehicle, player: Player);
callback!(onVehiclePaintJob,player: Player, vehicle: Vehicle, paintjob:isize,->bool);
callback!(onVehicleMod,player: Player, vehicle: Vehicle,component:isize,->bool);
callback!(onVehicleRespray,player: Player, vehicle: Vehicle, colour1:isize, colour2:isize,->bool);
callback!(onEnterExitModShop, player: Player, enterexit: bool, interior_id: isize);
callback!(onVehicleSpawn, vehicle: Vehicle);
callback!(onUnoccupiedVehicleUpdate, vehicle: Vehicle, player: Player,updateData:UnoccupiedVehicleUpdate,->bool);
callback!(onTrailerUpdate,player: Player, vehicle: Vehicle,->bool);
callback!(onVehicleSirenStateChange,player: Player, vehicle: Vehicle, sirenstate: u8,->bool);
