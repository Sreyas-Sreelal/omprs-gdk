use omp_codegen::native;
use std::ffi::c_void;

use crate::{players::Player, types::vector::Vector2};

use super::Menu;

native!(CreateMenu, title: str, columns: u32, position: Vector2, column1Width: f32, column2Width: f32, -> struct Menu);
native!(DestroyMenu, menu: struct Menu);
native!(AddMenuItem, menu: struct Menu, column: u8, text: str, -> isize);
native!(SetMenuColumnHeader, menu: struct Menu, column: u8, headerTitle: str);
native!(ShowMenuForPlayer, menu: struct Menu, player: struct Player);
native!(HideMenuForPlayer, menu: struct Menu, player: struct Player);
native!(DisableMenu, menu: struct Menu);
native!(DisableMenuRow, menu: struct Menu, row: u8);
native!(GetPlayerMenu, player: struct Player, -> struct Menu);
native!(IsMenuDisabled, menu: struct Menu, -> bool);
native!(IsMenuRowDisabled, menu: struct Menu, row: isize, -> bool);
native!(GetMenuColumns, menu: struct Menu, -> isize);
native!(GetMenuItems, menu: struct Menu, column: isize, -> isize);
native!(GetMenuPos, menu: struct Menu, pos: mut Vector2);
native!(GetMenuColumnWidth, menu: struct Menu, column1Width: mut f32, column2Width: mut f32);
native!(GetMenuColumnHeader, menu: struct Menu, column: isize, header: mut str);
native!(GetMenuItem, menu: struct Menu, column: isize, row: isize, item: mut str);
native!(GetMenuID, menu: struct Menu, -> isize);

#[doc(hidden)]
pub fn load_functions() {
    load_function!(CreateMenu);
    load_function!(DestroyMenu);
    load_function!(AddMenuItem);
    load_function!(SetMenuColumnHeader);
    load_function!(ShowMenuForPlayer);
    load_function!(HideMenuForPlayer);
    load_function!(DisableMenu);
    load_function!(DisableMenuRow);
    load_function!(GetPlayerMenu);
    load_function!(IsMenuDisabled);
    load_function!(IsMenuRowDisabled);
    load_function!(GetMenuColumns);
    load_function!(GetMenuItems);
    load_function!(GetMenuPos);
    load_function!(GetMenuColumnWidth);
    load_function!(GetMenuColumnHeader);
    load_function!(GetMenuItem);
    load_function!(GetMenuID);
}
