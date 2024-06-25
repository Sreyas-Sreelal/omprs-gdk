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

    /// Creates a menu
    pub fn create(
        title: &str,
        columns: u32,
        position: Vector2,
        column1_widthh: f32,
        column2_width: f32,
    ) -> Option<Menu> {
        let mut _id = 0;
        functions::Menu_Create(
            title,
            columns,
            position.x,
            position.y,
            column1_widthh,
            column2_width,
            &mut _id,
        )
    }

    /// Destroys a menu
    pub fn destroy(&self) -> bool {
        functions::Menu_Destroy(self)
    }

    /// Add item to menu
    pub fn add_item(&self, column: u8, text: &str) -> i32 {
        functions::Menu_AddItem(self, column, text)
    }

    /// Sets the caption of a column in a menu.
    pub fn set_column_header(&self, column: u8, header_title: &str) -> bool {
        functions::Menu_SetColumnHeader(self, column, header_title)
    }

    /// Shows the menu to a player
    pub fn show_for_player(&self, player: &Player) -> bool {
        functions::Menu_ShowForPlayer(self, player)
    }

    /// Hides the menu from a player
    pub fn hide_for_player(&self, player: &Player) -> bool {
        functions::Menu_HideForPlayer(self, player)
    }

    /// Disable a menu, same as destroying it
    pub fn disable(&self) -> bool {
        functions::Menu_Disable(self)
    }

    /// Disable a particular row in menu
    pub fn disable_row(&self, row: u8) -> bool {
        functions::Menu_DisableRow(self, row)
    }

    /// Checks if a menu disabled or not
    pub fn is_disabled(&self) -> bool {
        functions::Menu_IsDisabled(self)
    }

    /// Checks if a row in menu is disabled or not
    pub fn is_row_disabled(&self, row: i32) -> bool {
        functions::Menu_IsRowDisabled(self, row)
    }

    /// Get the number of active columns.
    pub fn get_columns(&self) -> i32 {
        functions::Menu_GetColumns(self)
    }

    /// Get the number of rows in the given column.
    pub fn get_items(&self, column: i32) -> i32 {
        functions::Menu_GetItems(self, column)
    }

    /// Get position of the menu
    pub fn get_pos(&self) -> Vector2 {
        let mut pos = Vector2::default();
        functions::Menu_GetPos(self, &mut pos.x, &mut pos.y);
        pos
    }

    /// Get the column width of menu
    pub fn get_column_width(&self) -> (f32, f32) {
        let (mut column1_width, mut column2_width) = (0.0, 0.0);
        functions::Menu_GetColumnWidth(self, &mut column1_width, &mut column2_width);
        (column1_width, column2_width)
    }

    /// Get caption of the menu
    pub fn get_column_header(&self, column: i32) -> String {
        let mut header = String::new();
        functions::Menu_GetColumnHeader(self, column, &mut header);
        header
    }

    /// Get an item at particular row and column in menu
    pub fn get_item(&self, column: i32, row: i32) -> String {
        let mut title = String::new();
        functions::Menu_GetItem(self, column, row, &mut title);
        title
    }

    /// Get the id of the menu object
    pub fn get_id(&self) -> i32 {
        functions::Menu_GetID(self)
    }

    /// Get menu instance from id
    pub fn from_id(id: i32) -> Option<Menu> {
        functions::Menu_FromID(id)
    }
}
