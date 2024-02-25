use omprs_gdk::{callback, main};

#[callback]
fn OnGameModeInit() {
    omprs_gdk::Console_Print("OnGameModeInit called");
}

#[callback]
fn OnPlayerConnect(playerid: isize) -> bool {
    omprs_gdk::SendClientMessage(playerid, -1, "Welcome to Grandlarc");
    true
}

#[main]
fn entry() {
    omprs_gdk::Console_Print("Hello world");
    omprs_gdk::Console_Print("Hello world");
    omprs_gdk::Console_Print("Hello world");
    omprs_gdk::Console_Print("Hello world");
}
