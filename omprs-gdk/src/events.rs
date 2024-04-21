use crate::{
    network::PeerDisconnectReason, vector::Vector3, BodyPart, Object, Player, PlayerBulletData,
    PlayerClickSource, PlayerObject, PlayerState, Vehicle,
};

#[allow(unused_variables)]
pub trait Events {
    /// Player spawn event handlers
    fn on_player_request_spawn(&mut self, player: Player) -> bool {
        return true;
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
        return true;
    }
    fn on_player_command_text(&mut self, player: Player, message: String) -> bool {
        return false;
    }

    /// Player shooting event handlers
    fn on_player_shot_missed(&mut self, player: Player, bullet_data: PlayerBulletData) -> bool {
        return true;
    }
    fn on_player_shot_player(
        &mut self,
        player: Player,
        target: Player,
        bullet_data: PlayerBulletData,
    ) -> bool {
        return true;
    }
    fn on_player_shot_vehicle(
        &mut self,
        player: Player,
        target: Vehicle,
        bullet_data: PlayerBulletData,
    ) -> bool {
        return true;
    }
    fn on_player_shot_object(
        &mut self,
        player: Player,
        target: Object,
        bullet_data: PlayerBulletData,
    ) -> bool {
        return true;
    }
    fn on_player_shot_player_object(
        &mut self,
        player: Player,
        target: PlayerObject,
        bullet_data: PlayerBulletData,
    ) -> bool {
        return true;
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
        return true;
    }
}
