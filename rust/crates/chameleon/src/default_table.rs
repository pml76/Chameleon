use crate::table_model::TableModel;
use anyhow::{Result, bail};
use chrono::NaiveDate;
use cxx_qt_lib_additions::Orientation;
use polars::df;
use polars::frame::DataFrame;
use polars::prelude::AnyValue;
use std::collections::HashMap;

pub struct DefaultTable {
    df: DataFrame,
}

impl Default for DefaultTable {
    fn default() -> Self {
        DefaultTable {
            df: df!(
                "name" => ["Alice Archer", "Ben Brown", "Chloe Cooper", "Daniel Donovan"],
                "birthdate" => [
                    NaiveDate::from_ymd_opt(1997, 1, 10).unwrap(),
                    NaiveDate::from_ymd_opt(1985, 2, 15).unwrap(),
                    NaiveDate::from_ymd_opt(1983, 3, 22).unwrap(),
                    NaiveDate::from_ymd_opt(1981, 4, 30).unwrap(),
                ],
                "weight" => [57.9, 72.5, 53.6, 83.1],  // (kg)
                "height" => [1.56, 1.77, 1.65, 1.75],  // (m)
            )
            .unwrap(),
        }
    }
}

impl TableModel for DefaultTable {
    fn column_count(self: &DefaultTable) -> Result<usize> {
        Ok(self.df.shape().1)
    }

    fn row_count(self: &DefaultTable) -> Result<usize> {
        Ok(self.df.shape().0)
    }

    fn data(self: &DefaultTable, row: usize, column: usize, _role: i32) -> Result<AnyValue<'_>> {
        let max_row = self.row_count()?;
        let max_column = self.column_count()?;

        if row < max_row && column < max_column {
            let schema = self.df.schema();
            if let Some((_name, _dtype)) = schema.get_at_index(column) {
                let column = self.df.select_at_idx(column).unwrap();
                let value = &column.get(row).unwrap();
                Ok(value.clone())
            } else {
                bail!("Invalid column")
            }
        } else {
            bail!("Invalid row or column")
        }
    }

    fn get_and_erase_base_model(&mut self) -> Option<Box<dyn TableModel>> {
        None
    }

    fn role_names(self: &DefaultTable) -> Result<HashMap<i32, String>> {
        let mut hash = HashMap::default();
        hash.insert(0, "display".into());
        Ok(hash)
    }

    fn header_data(
        self: &DefaultTable,
        section: i32,
        orientation: Orientation,
        _role: i32,
    ) -> Result<AnyValue<'_>> {
        match orientation {
            Orientation::Horizontal => {
                let col = section as usize;
                if col < self.df.shape().1 {
                    let schema = self.df.schema();
                    if let Some((name, _dtype)) = schema.get_at_index(col) {
                        return Ok(AnyValue::String(name.as_str()));
                    }
                }
            }
            Orientation::Vertical => {
                let row = section as usize;
                if row < self.df.shape().0 {
                    return Ok(AnyValue::UInt64(row as u64));
                }
            }
            _ => {}
        }
        bail!("Invalid header data")
    }
}
