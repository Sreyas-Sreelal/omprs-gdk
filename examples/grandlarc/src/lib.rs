use omprs_gdk::{
    actors::Actor, animationdata::AnimationData, colour::Colour, core::Print, main,
    players::Player, register, Events,
};

struct GrandLarc;

impl Events for GrandLarc {
    fn on_player_connect(&mut self, player: Player) {
        Print(&format!("Player name is {}", player.get_name()));
        player.send_client_message(
            Colour::from_rgba(0xFF000000),
            &format!("Welcome {} to GrandLarc", player.get_name()),
        );
    }

    fn on_player_text(&mut self, player: Player, message: String) -> bool {
        match message.as_str() {
            "killme" => {
                player.set_health(0.0);
            }
            "create actor" => {
                let mut pos = player.get_pos();
                pos.y += 2.0;
                let actor = Actor::create_actor(215, pos, 9.0);

                actor.set_skin(235);
                let id = actor.get_skin();
                dbg!(id);
                dbg!(actor.get_spawn_info());
                actor.apply_animation(AnimationData::new(
                    1.0,
                    true,
                    true,
                    true,
                    true,
                    9,
                    "PED",
                    "IDLE_CHAT",
                ));
                dbg!(
                    actor.get_skin(),
                    actor.get_animation().get_name(),
                    actor.get_animation().get_animation_library(),
                    actor.get_animation()
                );
            }
            "checkpoint" => {
                let mut pos = player.get_pos();
                pos.y += 2.0;
                player.set_player_checkpoint(pos, 3.0);
            }
            _ => {}
        }

        player.send_client_message(
            Colour::from_rgba(0xFF000000),
            &format!("{}:{message}", player.get_name()),
        );
        true
    }

    fn on_player_death(&mut self, _player: Player, killer: Option<Player>, _reason: isize) {
        dbg!(killer.is_some());
    }

    fn on_player_spawn(&mut self, player: Player) {
        player.set_skin(230);
    }

    fn on_player_enter_checkpoint(&mut self, player: Player) {
        player.send_client_message(Colour::from_rgba(0x0000EE00), "You reached checkpoint!!");
    }

    fn on_player_leave_checkpoint(&mut self, player: Player) {
        player.send_client_message(Colour::from_rgba(0x0000DD00), "You left checkpoint!!");
    }
}

#[main]
fn entry() {
    register!(GrandLarc);
    Print("Hello world");
}
