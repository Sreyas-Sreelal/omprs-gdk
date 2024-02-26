use omprs_gdk::{callback, main};

#[callback]
fn OnGameModeInit() {
    omprs_gdk::Console_Print("OnGameModeInit called");
}

#[callback]
fn OnPlayerConnect(playerid: isize) -> bool {
    let mut name = String::new();
    omprs_gdk::GetPlayerName(playerid, &mut name);

    omprs_gdk::Console_Print(&format!("Player name is {name}"));
    omprs_gdk::SendClientMessage(playerid, -1, &format!("Welcome {name} to GrandLarc"));

    true
}

#[main]
fn entry() {
    omprs_gdk::Console_Print("Hello world");
}
