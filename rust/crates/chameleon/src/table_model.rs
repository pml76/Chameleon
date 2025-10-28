use std::collections::HashMap;
use polars::prelude::AnyValue;
use chameleon_format_dialog::locale_selector_model::qobject::Orientation;
use anyhow::Result;


/// This trait governs the data acquisition for the TableManagerModel.
///
/// The TableManagerModel holds a Box filled with a type implementing this trait.
/// In case the TableManagerModel is asked for some data, it delegates the request to
/// the Boxed TableModel, which in turn can delegate the request (also partially)
/// to other TableModels.
pub trait TableModel {
    fn column_count(&self) -> Result<usize>;
    fn row_count(&self) -> Result<usize>;
    fn data<'a>(&'a self, row: usize, column: usize, role: i32) -> Result<AnyValue<'a>>;
    fn get_and_erase_base_model(&mut self) -> Option<Box<dyn TableModel>>;
    fn role_names(&self) -> Result<HashMap<i32, String>>;
    fn header_data<'a>(&'a self, section: i32, orientation: Orientation, role: i32) -> Result<AnyValue<'a>>;
}