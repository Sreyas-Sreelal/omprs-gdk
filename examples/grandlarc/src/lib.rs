use std::collections::HashMap;

use omprs_gdk::{
    actors::Actor,
    animationdata::AnimationData,
    classes::CreateClass,
    colour::Colour,
    core::Print,
    dialogs::{DialogResponse, DialogStyle},
    gangzones::{self, GangZone, GangZonePos},
    main,
    players::{Player, WeaponSlotData, WeaponSlots},
    register,
    vector::{Vector2, Vector3},
    Events,
};

enum DIALOGIDS {
    LOGIN,
}

struct GrandLarc {
    game_name: String,
    activegangzone: HashMap<usize, GangZone>,
}

impl Events for GrandLarc {
    fn on_player_connect(&mut self, player: Player) {
        Print(&format!("Player name is {}", player.get_name()));
        player.send_client_message(
            Colour::from_rgba(0xFF000000),
            &format!("Welcome {} to {}", player.get_name(), self.game_name),
        );

        player.show_dialog(
            DIALOGIDS::LOGIN as i16,
            DialogStyle::Password,
            "Enter Password",
            "You need special password to enter here",
            "Access",
            "",
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
            }
            "checkpoint" => {
                let mut pos = player.get_pos();
                pos.y += 2.0;
                player.set_player_checkpoint(pos, 3.0);
            }
            "zonecreate" => {
                let mut pos = player.get_pos();
                pos.y += 2.0;
                let zone = gangzones::GangZone::create(GangZonePos::new(
                    Vector2::new(pos.x, pos.y),
                    Vector2::new(pos.x + 100.0, pos.y + 100.0),
                ));
                zone.show_for_player(&player, Colour::from_rgba(0xFFA50065));
                zone.use_check(true);
            }
            "capture" => {
                if let Some(zone) = self.activegangzone.get(&player.get_id()) {
                    if zone.is_player_in_gang_zone(&player) {
                        zone.flash_for_player(&player, Colour::from_rgba(0xFFFFFFFF));
                        player.send_client_message(Colour::from_rgba(u32::MAX), "Capturing zone");
                        return false;
                    } else {
                        player.send_client_message(
                            Colour::from_rgba(u32::MAX),
                            "You're not in any zone",
                        );
                        return false;
                    }
                }
            }
            _ => {}
        }

        player.send_client_message(
            Colour::from_rgba(0xFF000000),
            &format!("{}:{message}", player.get_name()),
        );
        false
    }

    fn on_player_death(&mut self, _player: Player, killer: Option<Player>, _reason: isize) {
        dbg!(killer.is_some());
    }

    fn on_player_spawn(&mut self, player: Player) {
        // player.set_skin(230);
    }

    fn on_player_enter_checkpoint(&mut self, player: Player) {
        player.send_client_message(Colour::from_rgba(0x0000EE00), "You reached checkpoint!!");
    }

    fn on_player_leave_checkpoint(&mut self, player: Player) {
        player.send_client_message(Colour::from_rgba(0x0000DD00), "You left checkpoint!!");
    }

    fn on_player_request_class(&mut self, player: Player, class_id: usize) -> bool {
        true
    }

    fn on_dialog_response(
        &mut self,
        player: Player,
        dialog_id: i16,
        response: omprs_gdk::dialogs::DialogResponse,
        _list_item: isize,
        input_text: String,
    ) {
        match dialog_id {
            0 => {
                if let DialogResponse::Left = response {
                    match input_text.as_str() {
                        "password" => {
                            player.send_client_message(
                                Colour::from_rgba(0x00FF0000),
                                "Access granted!!",
                            );
                        }
                        _ => {
                            player.send_client_message(
                                Colour::from_rgba(0xFF000000),
                                "Access denied! Try again",
                            );
                            player.show_dialog(
                                dialog_id,
                                DialogStyle::Password,
                                "Enter Password",
                                "You need special password to enter here",
                                "Access",
                                "",
                            );
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn on_player_enter_gang_zone(&mut self, player: Player, zone: GangZone) {
        player.send_client_message(
            Colour::from_rgba(0xFF000000),
            "You are entering a gang zone!!",
        );
        self.activegangzone.insert(player.get_id(), zone);
    }

    fn on_player_leave_gang_zone(&mut self, player: Player, zone: GangZone) {
        player.send_client_message(
            Colour::from_rgba(0x00FF0000),
            "You are leaving a gang zone!!",
        );
        self.activegangzone.remove(&player.get_id());
        if zone.is_flashing_for_player(&player) {
            zone.stop_flash_for_player(&player);
        }
    }
}

#[main]
fn entry() {
    register!(GrandLarc {
        game_name: String::from("gg"),
        activegangzone: HashMap::new(),
    });
    let pos = Vector3::new(1958.33, 1343.12, 15.36);

    let mut slots = WeaponSlots::default();
    slots[0] = WeaponSlotData::new(24, 36);
    slots[1] = WeaponSlotData::new(28, 150);
    slots[2] = WeaponSlotData::new(1, 1);
    CreateClass(255, 6, pos, 269.15, slots);
    CreateClass(1, 230, pos, 269.15, slots);
    Print("Hello world");
}
