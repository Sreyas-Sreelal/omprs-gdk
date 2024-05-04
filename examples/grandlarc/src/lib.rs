mod spawns;
use spawns::SpawnLocations;
use std::{collections::HashMap, time::Instant};

use omprs::{
    classes::CreateClass,
    core::{
        DisableInteriorEnterExits, EnableStuntBonusForAll, SetGameModeText, SetNameTagDrawDistance, SetWeather, SetWorldTime, ShowNameTags, ShowPlayerMarkers
    },
    events::Events,
    main,
    players::{Player, PlayerState, WeaponSlotData, WeaponSlots},
    register,
    textdraws::{TextDraw, TextDrawStyle},
    types::{
        colour::Colour,
        vector::{Vector2, Vector3},
    },
};

enum Cities {
    LosSantos,
    SanFierro,
    LasVenturas,
}

struct PlayerData {
    pub selected_city: Option<Cities>,
    pub last_city_selection_tick: Instant,
    pub has_city_selected: bool,
}

struct GrandLarc {
    colour_white: Colour,
    players_data: HashMap<usize, PlayerData>,
    class_selection_helper_td: TextDraw,
    los_santos_td: TextDraw,
    san_fierro_td: TextDraw,
    las_venturas_td: TextDraw,
    spawn_locations: SpawnLocations,
}

impl GrandLarc {
    pub fn setup_char_selection(&self, player: &Player) {
        match self.players_data[&player.get_id()].selected_city {
            Some(Cities::LosSantos) => {
                player.set_interior(11);
                player.set_pos(Vector3::new(508.7362, -87.4335, 998.9609));
                player.set_facing_angle(0.0);
                player.set_camera_pos(Vector3::new(508.7362, -83.4335, 998.9609));
                player.set_camera_look_at(Vector3::new(508.7362, -87.4335, 998.9609), 1);
            }
            Some(Cities::SanFierro) => {
                player.set_interior(3);
                player.set_pos(Vector3::new(-2673.8381, 1399.7424, 918.3516));
                player.set_facing_angle(181.0);
                player.set_camera_pos(Vector3::new(-2673.2776, 1394.3859, 918.3516));
                player.set_camera_look_at(Vector3::new(-2673.8381, 1399.7424, 918.3516), 1);
            }
            Some(Cities::LasVenturas) => {
                player.set_interior(3);
                player.set_pos(Vector3::new(349.0453, 193.2271, 1014.1797));
                player.set_facing_angle(286.25);
                player.set_camera_pos(Vector3::new(352.9164, 194.5702, 1014.1875));
                player.set_camera_look_at(Vector3::new(349.0453, 193.2271, 1014.1797), 1);
            }
            None => {}
        }
    }

    pub fn init_city_name_td(&self, td: &TextDraw) {
        td.use_box(false);
        td.set_letter_size(Vector2::new(1.25, 3.0));
        td.set_style(TextDrawStyle::FontBeckettRegular);
        td.set_shadow(0);
        td.set_outline(1);
        td.set_color(Colour::from_rgba(0xEEEEEEFF));
        self.class_selection_helper_td
            .set_background_color(Colour::from_rgba(0x000000FF));
    }

    pub fn setup_selected_city(&mut self, player: &Player) {
        let playerid = player.get_id();
        if self.players_data[&playerid].selected_city.is_none() {
            self.players_data
                .get_mut(&player.get_id())
                .unwrap()
                .selected_city = Some(Cities::LosSantos);
        }

        match self.players_data[&playerid].selected_city {
            Some(Cities::LosSantos) => {
                player.set_interior(0);
                player.set_camera_pos(Vector3::new(1630.6136, -2286.0298, 110.0));
                player.set_camera_look_at(Vector3::new(1887.6034, -1682.1442, 47.6167), 1);
                self.los_santos_td.show_for_player(&player);
                self.san_fierro_td.hide_for_player(&player);
                self.las_venturas_td.hide_for_player(&player);
            }

            Some(Cities::SanFierro) => {
                player.set_interior(0);
                player.set_camera_pos(Vector3::new(-1300.8754, 68.0546, 129.4823));
                player.set_camera_look_at(Vector3::new(-1817.9412, 769.3878, 132.6589), 1);
                self.los_santos_td.hide_for_player(&player);
                self.san_fierro_td.show_for_player(&player);
                self.las_venturas_td.hide_for_player(&player);
            }
            Some(Cities::LasVenturas) => {
                player.set_interior(0);
                player.set_camera_pos(Vector3::new(1310.6155, 1675.9182, 110.7390));
                player.set_camera_look_at(Vector3::new(2285.2944, 1919.3756, 68.2275), 1);
                self.los_santos_td.hide_for_player(&player);
                self.san_fierro_td.hide_for_player(&player);
                self.las_venturas_td.show_for_player(&player);
            }
            None => {}
        }
    }

    pub fn switch_to_next_city(&mut self, player: &Player) {
        match self.players_data[&player.get_id()].selected_city {
            Some(Cities::LosSantos) => {
                self.players_data
                    .get_mut(&player.get_id())
                    .unwrap()
                    .selected_city = Some(Cities::SanFierro);
            }
            Some(Cities::SanFierro) => {
                self.players_data
                    .get_mut(&player.get_id())
                    .unwrap()
                    .selected_city = Some(Cities::LasVenturas);
            }
            Some(Cities::LasVenturas) => {
                self.players_data
                    .get_mut(&player.get_id())
                    .unwrap()
                    .selected_city = Some(Cities::LosSantos);
            }
            None => {
                self.players_data
                    .get_mut(&player.get_id())
                    .unwrap()
                    .selected_city = Some(Cities::LosSantos);
            }
        }
        player.play_sound(1052, Vector3::default());
        self.players_data
            .get_mut(&player.get_id())
            .unwrap()
            .last_city_selection_tick = Instant::now();
        self.setup_selected_city(&player);
    }

    pub fn switch_to_previous_city(&mut self, player: &Player) {
        match self.players_data[&player.get_id()].selected_city {
            Some(Cities::LosSantos) => {
                self.players_data
                    .get_mut(&player.get_id())
                    .unwrap()
                    .selected_city = Some(Cities::LasVenturas);
            }
            Some(Cities::SanFierro) => {
                self.players_data
                    .get_mut(&player.get_id())
                    .unwrap()
                    .selected_city = Some(Cities::LosSantos);
            }
            Some(Cities::LasVenturas) => {
                self.players_data
                    .get_mut(&player.get_id())
                    .unwrap()
                    .selected_city = Some(Cities::SanFierro);
            }
            None => {}
        }
        player.play_sound(1053, Vector3::default());
        self.players_data
            .get_mut(&player.get_id())
            .unwrap()
            .last_city_selection_tick = Instant::now();
        self.setup_selected_city(&player);
    }

    pub fn handle_city_selection(&mut self, player: &Player) {
        let keydata = player.get_keys();
        if self.players_data[&player.get_id()].selected_city.is_none() {
            self.switch_to_next_city(&player);
            return;
        }

        if self.players_data[&player.get_id()]
            .last_city_selection_tick
            .elapsed()
            .as_millis()
            < 500
        {
            return;
        }

        if (keydata.keys & 4) != 0 {
            self.players_data
                .get_mut(&player.get_id())
                .unwrap()
                .has_city_selected = true;
            self.los_santos_td.hide_for_player(&player);
            self.san_fierro_td.hide_for_player(&player);
            self.las_venturas_td.hide_for_player(&player);
            player.toggle_spectating(false);
            return;
        }

        if keydata.leftRight > 0 {
            self.switch_to_next_city(&player);
        } else if keydata.leftRight < 0 {
            self.switch_to_previous_city(&player);
        }
    }
}

impl Events for GrandLarc {
    fn on_player_connect(&mut self, player: Player) {
        player.game_text("~w~Grand Larceny", 3000, 4);
        player.send_client_message(
            self.colour_white,
            "Welcome to {88AA88}G{FFFFFF}rand {88AA88}L{FFFFFF}arceny",
        );
        self.players_data.insert(
            player.get_id(),
            PlayerData {
                selected_city: None,
                last_city_selection_tick: Instant::now(),
                has_city_selected: false,
            },
        );
    }

    fn on_player_spawn(&mut self, player: Player) {
        if player.is_npc() {
            return;
        }

        player.set_interior(0);
        player.toggle_clock(false);
        player.reset_money();
        player.give_money(30000);

        match self.players_data[&player.get_id()].selected_city {
            Some(Cities::LosSantos) => {
                let coords = self.spawn_locations.get_random_ls();
                player.set_pos(coords.0);
                player.set_facing_angle(coords.1);
            }
            Some(Cities::SanFierro) => {
                let coords = self.spawn_locations.get_random_sf();
                player.set_pos(coords.0);
                player.set_facing_angle(coords.1);
            }
            Some(Cities::LasVenturas) => {
                let coords = self.spawn_locations.get_random_lv();
                player.set_pos(coords.0);
                player.set_facing_angle(coords.1);
            }
            None => {}
        }

        player.give_weapon(WeaponSlotData::new(22, 100));
        player.toggle_clock(false);
    }

    fn on_player_death(&mut self, player: Player, killer: Option<Player>, _reason: isize) {
        self.players_data
            .get_mut(&player.get_id())
            .unwrap()
            .has_city_selected = false;
        if killer.is_none() {
            player.reset_money();
        } else {
            let playercash = player.get_money();
            if playercash > 0 {
                killer.unwrap().give_money(playercash);
                player.reset_money();
            }
        }
    }

    fn on_player_request_class(&mut self, player: Player, _class_id: usize) -> bool {
        if player.is_npc() {
            return true;
        }
        if self.players_data[&player.get_id()].has_city_selected {
            self.setup_char_selection(&player);
            return true;
        } else {
            if player.get_state() != PlayerState::Spectating {
                player.toggle_spectating(true);
                self.class_selection_helper_td.show_for_player(&player);
                self.players_data
                    .get_mut(&player.get_id())
                    .unwrap()
                    .selected_city = None;
            }
        }
        return false;
    }

    fn on_player_update(&mut self, player: Player, _now: isize) -> bool {
        if player.is_npc() {
            return true;
        }

        if !self.players_data[&player.get_id()].has_city_selected
            && player.get_state() == PlayerState::Spectating
        {
            self.handle_city_selection(&player);
            return true;
        }

        if player.get_weapon() == 38 {
            player.kick();
            return false;
        }
        return true;
    }
}

#[main]
pub fn GameEntry() {
    SetGameModeText("Grand Larceny");
    ShowPlayerMarkers(1);
    ShowNameTags(true);
    SetNameTagDrawDistance(40.0);
    EnableStuntBonusForAll(false);
    DisableInteriorEnterExits();
    SetWeather(2);
    SetWorldTime(11);

    let game = GrandLarc{
        class_selection_helper_td: TextDraw::create(Vector2::new(10.0, 415.0), " Press ~b~~k~~GO_LEFT~ ~w~or ~b~~k~~GO_RIGHT~ ~w~to switch cities.~n~ Press ~r~~k~~PED_FIREWEAPON~ ~w~to select.").unwrap(),
        los_santos_td: TextDraw::create(Vector2::new(10.0, 380.0), "Los Santos").unwrap(),
        san_fierro_td: TextDraw::create(Vector2::new(10.0, 380.0), "San Fierro").unwrap(),
        las_venturas_td: TextDraw::create(Vector2::new(10.0, 380.0), "Las Venturas").unwrap(),
        spawn_locations: SpawnLocations::new(),
        players_data: HashMap::new(),
        colour_white: Colour::from_rgba(0xFFFFFFFF),
    };
    game.init_city_name_td(&game.los_santos_td);
    game.init_city_name_td(&game.san_fierro_td);

    game.init_city_name_td(&game.las_venturas_td);

    game.class_selection_helper_td.use_box(true);
    game.class_selection_helper_td
        .set_box_color(Colour::from_rgba(0x222222BB));
    game.class_selection_helper_td
        .set_letter_size(Vector2::new(0.3, 1.0));
    game.class_selection_helper_td
        .set_text_size(Vector2::new(400.0, 40.0));
    game.class_selection_helper_td
        .set_style(TextDrawStyle::FontBankGothic);
    game.class_selection_helper_td.set_shadow(0);
    game.class_selection_helper_td.set_outline(1);
    game.class_selection_helper_td
        .set_background_color(Colour::from_rgba(0x000000FF));
    game.class_selection_helper_td
        .set_color(Colour::from_rgba(0xFFFFFFFF));

    register!(game);
    CreateClass(
        255,
        298,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        299,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        300,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        301,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        302,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        303,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        304,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        305,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        280,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        281,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        282,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        283,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        284,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        285,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        286,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        287,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        288,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        289,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        265,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        266,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        267,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        268,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        269,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        270,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        1,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        2,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        3,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        4,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        5,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        6,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        8,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        42,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        65,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    //CreateClass(255,74,Vector3::new(1759.0189,-1898.1260,13.5622),266.4503,WeaponSlots::default());
    CreateClass(
        255,
        86,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        119,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        149,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        208,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        273,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        289,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );

    CreateClass(
        255,
        47,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        48,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        49,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        50,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        51,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        52,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        53,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        54,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        55,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        56,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        57,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        58,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        68,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        69,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        70,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        71,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        72,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        73,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        75,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        76,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        78,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        79,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        80,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        81,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        82,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        83,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        84,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        85,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        87,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        88,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        89,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        91,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        92,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        93,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        95,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        96,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        97,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        98,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
    CreateClass(
        255,
        99,
        Vector3::new(1759.0189, -1898.1260, 13.5622),
        266.4503,
        WeaponSlots::default(),
    );
}
