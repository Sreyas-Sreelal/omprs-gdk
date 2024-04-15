use omprs_gdk::{callback, main, vector::Vector3, AnimationData};
use std::time::Duration;

#[callback]
fn OnGameModeInit() {
    omprs_gdk::Print("OnGameModeInit called");
}

#[callback]
fn OnPlayerConnect(playerid: isize) -> bool {
    let mut name = String::new();
    omprs_gdk::GetPlayerName(playerid, &mut name);
    omprs_gdk::Print(&format!("Player name is {name}"));
    omprs_gdk::SendClientMessage(
        playerid,
        usize::MAX,
        &format!("Welcome {name} to GrandLarc"),
    );

    true
}

#[callback]
fn OnPlayerSpawn(playerid: isize) -> bool {
    omprs_gdk::SetPlayerSkin(playerid, 230);
    std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_millis(5000));
        omprs_gdk::SendClientMessageToAll(0xFF000000, "{FF0000}Still {00FF00}running!!");
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        omprs_gdk::GetPlayerPos(playerid, &mut x, &mut y, &mut z);
        omprs_gdk::SendClientMessage(playerid, usize::MAX, &format!("POS: {x} {y} {z}"));
    });
    true
}

#[callback]
fn OnPlayerText(playerid: isize, text: String) -> bool {
    let mut name = String::new();
    omprs_gdk::GetPlayerName(playerid, &mut name);
    omprs_gdk::SendClientMessageToAll(0xFF000000, &format!("{name}:{text}"));
    true
}

#[main]
fn entry() {
    omprs_gdk::Print("Hello world");

    let component = omprs_gdk::GetActorsComponent();

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
    );
}
