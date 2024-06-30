use omp::{
    events::Events,
    players::{Player, PlayerBulletData, PlayerWeapon},
    terminate_event,
};

pub struct MiniGunDetector {
    pub on_player_shot_minigun: fn(player: Player),
}

impl Events for MiniGunDetector {
    fn on_player_shot_missed(&mut self, player: Player, bullet_data: PlayerBulletData) -> bool {
        if bullet_data.weapon == PlayerWeapon::Minigun {
            (self.on_player_shot_minigun)(player);
            terminate_event!(true);
        }
        true
    }
}
