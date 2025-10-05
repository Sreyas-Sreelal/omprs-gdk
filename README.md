# omprs

[<img alt="crates.io" src="https://img.shields.io/crates/v/omp.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/omp)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-omp-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/omp)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/sreyas-sreelal/omprs-gdk/build.yml?branch=master&style=for-the-badge" height="20">](https://github.com/sreyas-sreelal/omprs-gdk/actions?query=branch%3Amaster)
[<img alt="patreon" src="https://img.shields.io/badge/patreon-sreyas_sreelal-pink?style=for-the-badge&logo=patreon" height="20">](https://www.patreon.com/sreyas_sreelal)
[<img alt="kofi" src="https://img.shields.io/badge/kofi-sreyas-blue?style=for-the-badge&logo=kofi" height="20">](https://ko-fi.com/sreyas)


omprs is a tool to develop open.mp gamemodes and components in Rust.

## Structure
|**Crate**|**Description**|
|-----|----------------------------------------------------------------------|
|`omp-codegen`| Generates exported functions and FFI related code automatically|
|`omp-sdk`| GDK crate, that does the core functionality like loading function address, executing, providing necessary types etc|
|`omp`| The main crate the is supposed to be used by the component or server developers, neatly exposing all of the functionalities and APIs.


## Writing my first gamemode in Rust
1. Download and install latest open.mp C-API component from [here](https://github.com/openmultiplayer/omp-capi/releases)
2. Create a new rust project
   `cargo new mygm --lib`
3. Add `omp` to dependecies
    `cargo add omp`
4. Add this to your `Cargo.toml`
    ```toml
    [lib]
    crate-type = ["cdylib"]
    ```
5. Write a basic code like this
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
6. Build your gamemode or component

   `cargo +stable-i686 build`
7. Put the compile `mygm.dll` or `mygm.so` to `components` folder

8. Run your server



