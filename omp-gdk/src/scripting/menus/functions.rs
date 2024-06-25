use omp_codegen::native;
use std::ffi::c_void;

use crate::players::Player;

use super::Menu;

native!(Menu_Create, title: str, columns: u32, x: f32, y: f32, column1Width: f32, column2Width: f32, id: mut i32, -> struct Menu);
native!(Menu_Destroy, menu: struct Menu, -> bool);
native!(Menu_FromID, menuid: i32, -> struct Menu);
native!(Menu_GetID, menu: struct Menu, -> i32);
native!(Menu_AddItem, menu: struct Menu, column: u8, text: str, -> i32);
native!(Menu_SetColumnHeader, menu: struct Menu, column: u8, headerTitle: str, -> bool);
native!(Menu_ShowForPlayer, menu: struct Menu, player: struct Player, -> bool);
native!(Menu_HideForPlayer, menu: struct Menu, player: struct Player, -> bool);
native!(Menu_Disable, menu: struct Menu, -> bool);
native!(Menu_DisableRow, menu: struct Menu, row: u8, -> bool);
native!(Menu_IsValid, menu: struct Menu, -> bool);
native!(Menu_IsDisabled, menu: struct Menu, -> bool);
native!(Menu_IsRowDisabled, menu: struct Menu, row: i32, -> bool);
native!(Menu_GetColumns, menu: struct Menu, -> i32);
native!(Menu_GetItems, menu: struct Menu, column: i32, -> i32);
native!(Menu_GetPos, menu: struct Menu, x: mut f32, y: mut f32, -> bool);
native!(Menu_GetColumnWidth, menu: struct Menu, column1Width: mut f32, column2Width: mut f32, -> bool);
native!(Menu_GetColumnHeader, menu: struct Menu, column: i32, header: mut str, -> bool);
native!(Menu_GetItem, menu: struct Menu, column: i32, row: i32, item: mut str, -> bool);

#[doc(hidden)]
pub fn load_functions() {
    load_function!(Menu_Create);
    load_function!(Menu_Destroy);
    load_function!(Menu_FromID);
    load_function!(Menu_GetID);
    load_function!(Menu_AddItem);
    load_function!(Menu_SetColumnHeader);
    load_function!(Menu_ShowForPlayer);
    load_function!(Menu_HideForPlayer);
    load_function!(Menu_Disable);
    load_function!(Menu_DisableRow);
    load_function!(Menu_IsValid);
    load_function!(Menu_IsDisabled);
    load_function!(Menu_IsRowDisabled);
    load_function!(Menu_GetColumns);
    load_function!(Menu_GetItems);
    load_function!(Menu_GetPos);
    load_function!(Menu_GetColumnWidth);
    load_function!(Menu_GetColumnHeader);
    load_function!(Menu_GetItem);
}
