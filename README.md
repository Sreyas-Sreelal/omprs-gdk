# omprs

[<img alt="crates.io" src="https://img.shields.io/crates/v/omp.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/omp)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-omp-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/omp)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/sreyas-sreelal/omprs-gdk/build.yml?branch=master&style=for-the-badge" height="20">](https://github.com/sreyas-sreelal/omprs-gdk/actions?query=branch%3Amaster)
[<img alt="patreon" src="https://img.shields.io/badge/patreon-sreyas_sreelal-pink?style=for-the-badge&logo=patreon" height="20">](https://www.patreon.com/sreyas_sreelal)
[<img alt="kofi" src="https://img.shields.io/badge/kofi-sreyas-blue?style=for-the-badge&logo=kofi" height="20">](https://ko-fi.com/sreyas)


omprs is a tool to develop open.mp gamemodes in Rust.

## Structure
|**Crate**|**Description**|
|-----|----------------------------------------------------------------------|
|`omp-codegen`| Generates exported functions and FFI related code automatically|
|`omp-sdk`| GDK crate, that does the core functionality like loading function address, executing, providing necessary types etc|
|`omp`| The main crate the is supposed to be used by the players, neatly exposing all of the functionalities and APIs.


## Writing my first gamemode in Rust
1. Download the omprs component from [here](https://github.com/Sreyas-Sreelal/omprs/releases)
2. Place the `Rust.dll` or `Rust.so` component in `components` folder
3. Create a new rust project
   `cargo new mygm --lib`
4. Add `omp` to dependecies
    `cargo add omp`
5. Add this to your `Cargo.toml`
    ```toml
    [lib]
    crate-type = ["cdylib"]
    ```
6. Write a basic code like this
    ```Rust
    use omp::{events::Events, main, register, types::colour::Colour};

    struct MyGM;

    impl Events for MyGM {
        fn on_player_connect(&mut self, player: omp::players::Player) {
            player.send_client_message(Colour::from_rgba(0xFFFFFFFF), "Welcome to my server!");
        }
    }

    #[main]
    pub fn game_main() {
        register!(MyGM);
    }
    ```
7. Build the gamemode

   `cargo +stable-i686 build`
8. Put the compile `mygm.dll` or `mygm.so` to `gamemodes` folder
9. Goto `config.json` add following to it
    ```json
    "rust":{
        "gamemode":"mygm"
    }
    ```
10. Run your server



