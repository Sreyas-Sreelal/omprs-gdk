pub mod events;
pub mod functions;

use std::ffi::c_void;

pub use functions::load_functions;

use crate::{players::Player, types::vector::Vector2};

pub struct Menu {
    handle: *const c_void,
}

impl Menu {
    pub fn get_handle(&self) -> *const c_void {
        self.handle
    }

    pub fn new(handle: *const c_void) -> Self {
        Self { handle }
    }

    pub fn create(
        title: &str,
        columns: u32,
        position: Vector2,
        column1_widthh: f32,
        column2_width: f32,
    ) -> Option<Menu> {
        functions::CreateMenu(title, columns, position, column1_widthh, column2_width)
    }

    pub fn destroy(&self) {
        functions::DestroyMenu(self)
    }

    pub fn add_item(&self, column: u8, text: &str) -> isize {
        functions::AddMenuItem(self, column, text)
    }

    pub fn set_column_header(&self, column: u8, header_title: &str) {
        functions::SetMenuColumnHeader(self, column, header_title)
    }

    pub fn show_for_player(&self, player: &Player) {
        functions::ShowMenuForPlayer(self, player)
    }

    pub fn hide_for_player(&self, player: &Player) {
        functions::HideMenuForPlayer(self, player)
    }

    pub fn disable(&self) {
        functions::DestroyMenu(self)
    }

    pub fn disable_row(&self, row: u8) {
        functions::DisableMenuRow(self, row)
    }

    pub fn is_valid(&self) -> bool {
        functions::IsValidMenu(self)
    }

    pub fn is_disabled(&self) -> bool {
        functions::IsMenuDisabled(self)
    }

    pub fn is_row_disabled(&self, row: isize) -> bool {
        functions::IsMenuRowDisabled(self, row)
    }

    pub fn get_columns(&self) -> isize {
        functions::GetMenuColumns(self)
    }

    pub fn get_items(&self, column: isize) -> isize {
        functions::GetMenuItems(self, column)
    }

    pub fn get_pos(&self) -> Vector2 {
        let mut pos = Vector2::default();
        functions::GetMenuPos(self, &mut pos);
        pos
    }

    pub fn get_column_width(&self) -> (f32, f32) {
        let (mut column1_width, mut column2_width) = (0.0, 0.0);
        functions::GetMenuColumnWidth(self, &mut column1_width, &mut column2_width);
        (column1_width, column2_width)
    }

    pub fn get_column_header(&self, column: isize) -> String {
        let mut header = String::new();
        functions::GetMenuColumnHeader(self, column, &mut header);
        header
    }

    pub fn get_item(&self, column: isize, row: isize) -> String {
        let mut title = String::new();
        functions::GetMenuItem(self, column, row, &mut title);
        title
    }

    pub fn get_id(&self) -> isize {
        functions::GetMenuID(self)
    }
}
