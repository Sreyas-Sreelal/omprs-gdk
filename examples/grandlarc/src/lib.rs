use std::time::Duration;

use omprs_gdk::{callback, colour::Colour, main, Player};

#[callback]
fn OnGameModeInit() {
    omprs_gdk::Print("OnGameModeInit called");
}

#[callback]
fn OnPlayerConnect(player: Player) -> bool {
    omprs_gdk::Print(&format!("Player name is {}", player.get_name()));
    player.send_client_message(
        Colour::from_rgba(0xFF000000),
        &format!("Welcome {} to GrandLarc", player.get_name()),
    );

    true
}

#[callback]
fn OnPlayerSpawn(player: Player) -> bool {
    player.set_skin(230);
    std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_millis(5000));
        player.send_client_message(
            Colour::from_rgba(0xFF000000),
            "{FF0000}Still {00FF00}running!!",
        );

        let vector = player.get_pos();
        player.send_client_message(
            Colour::from_rgba(u32::MAX),
            &format!("POS: {} {} {}", vector.x, vector.y, vector.z),
        );
    });
    true
}

#[callback]
fn OnPlayerText(player: Player, text: String) -> bool {
    player.send_client_message(
        Colour::from_rgba(0xFF000000),
        &format!("{}:{text}", player.get_name()),
    );
    true
}

#[main]
fn entry() {
    omprs_gdk::Print("Hello world");

    /* let component = omprs_gdk::GetActorsComponent();

    let actor = (*component).create(
        215,
        Vector3 {
            x: 2.0,
            y: 6.0,
            z: 8.0,
        },
        9.0,
    );

    actor.setSkin(230);
    let id = actor.getSkin();
    dbg!(id);

    dbg!(actor.getSpawnData());
    actor.applyAnimation(&AnimationData::new(
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
        actor.getSkin(),
        actor.getAnimation().get_name(),
        actor.getAnimation().get_animation_library(),
        actor.getAnimation()
    ); */
}
