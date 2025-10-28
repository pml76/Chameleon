use crate::default_table::DefaultTable;
use crate::table_manager_model::qobject::{
    Orientation, QHash_i32_QByteArray, QModelIndex, QVariant, TableManagerModel,
};
use crate::table_model::TableModel;
use anyhow::{Result, bail};
use chameleon_format_dialog::format::LocalizedNumberFormatter;
use chrono::NaiveDate;
use chrono_tz::Tz;
use cxx_qt::CxxQtType;
use cxx_qt_lib::{QString, QTime};
use polars::datatypes::AnyValue::{
    Boolean, Categorical, CategoricalOwned, Date, Datetime, DatetimeOwned, Duration, Enum,
    EnumOwned, Float32, Float64, Int8, Int16, Int32, Int64, String, Time, UInt8, UInt16, UInt32,
    UInt64,
};
use polars::datatypes::{AnyValue, CatSize, CategoricalMapping, TimeUnit, TimeZone};
use std::collections::HashMap;
use std::sync::Arc;

#[cxx_qt::bridge]
mod qobject {

    unsafe extern "C++" {
        include!(<QAbstractTableModel>);
        type QAbstractTableModel;

        include!("cxx-qt-lib/qmodelindex.h");
        type QModelIndex = cxx_qt_lib::QModelIndex;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;

        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qhash.h");
        type QHash_i32_QByteArray = cxx_qt_lib::QHash<cxx_qt_lib::QHashPair_i32_QByteArray>;

        #[namespace = "Qt"]
        type Orientation = cxx_qt_lib_additions::Orientation;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[base = QAbstractTableModel]
        type TableManagerModel = super::TableManagerModelRust;

        #[cxx_override]
        #[rust_name = "column_count"]
        fn columnCount(self: &TableManagerModel, _parent: &QModelIndex) -> i32;

        #[cxx_override]
        #[rust_name = "row_count"]
        fn rowCount(self: &TableManagerModel, _parent: &QModelIndex) -> i32;

        #[cxx_override]
        #[rust_name = "data"]
        fn data(self: &TableManagerModel, index: &QModelIndex, role: i32) -> QVariant;

        #[cxx_override]
        #[rust_name = "role_names"]
        fn roleNames(self: &TableManagerModel) -> QHash_i32_QByteArray;

        #[cxx_override]
        #[rust_name = "header_data"]
        fn headerData(
            self: &TableManagerModel,
            section: i32,
            orientation: Orientation,
            role: i32,
        ) -> QVariant;
    }
}

pub struct TableManagerModelRust {
    table_model: Option<Box<dyn TableModel>>,
}

impl Default for TableManagerModelRust {
    fn default() -> Self {
        Self {
            table_model: Some(Box::new(DefaultTable::default())),
        }
    }
}

impl TableModel for TableManagerModelRust {
    fn column_count(&self) -> Result<usize> {
        if let Some(table_model) = &self.table_model {
            table_model.column_count()
        } else {
            bail!("No table model set".to_string());
        }
    }

    fn row_count(&self) -> Result<usize> {
        if let Some(table_model) = &self.table_model {
            table_model.row_count()
        } else {
            bail!("No table model set".to_string());
        }
    }

    fn data(&self, row: usize, column: usize, role: i32) -> Result<AnyValue<'_>> {
        match &self.table_model {
            None => bail!("No table model set"),
            Some(table_model) => {
                let max_row = self.row_count()?;
                let max_column = self.column_count()?;

                if row >= max_row || column >= max_column {
                    bail!("Invalid row or column")
                } else {
                    table_model.data(row, column, role)
                }
            }
        }
    }

    fn get_and_erase_base_model(&mut self) -> Option<Box<dyn TableModel>> {
        match &self.table_model {
            None => None,
            Some(_) => self.table_model.take(),
        }
    }

    fn role_names(&self) -> Result<HashMap<i32, std::string::String>> {
        match &self.table_model {
            None => bail!("No table model set"),
            Some(table_model) => table_model.role_names(),
        }
    }

    fn header_data(
        &self,
        section: i32,
        orientation: Orientation,
        role: i32,
    ) -> Result<AnyValue<'_>> {
        match &self.table_model {
            None => bail!("No table model set".to_string()),
            Some(table_model) => table_model.header_data(section, orientation, role),
        }
    }
}
trait FromAnyValue {
    fn from_any_value(any_value: &AnyValue) -> Self;
    fn from_datetime(d: &i64, time_unit: &TimeUnit, time_zone: &Option<&TimeZone>) -> Self;

    fn from_categorical(idx: &CatSize, categories: &Arc<CategoricalMapping>) -> Self;
}

impl FromAnyValue for QVariant {
    fn from_any_value(any_value: &AnyValue) -> Self {
        match any_value {
            Boolean(b) => QVariant::from(b),

            String(str) => QVariant::from(&QString::from(&str.to_string())),

            UInt8(u) => QVariant::from(u),
            UInt16(u) => QVariant::from(u),
            UInt32(u) => QVariant::from(u),
            UInt64(u) => QVariant::from(u),
            Int8(i) => QVariant::from(i),
            Int16(i) => QVariant::from(i),
            Int32(i) => QVariant::from(i),
            Int64(i) => QVariant::from(i),
            Float32(f) => QVariant::from(f),

            Float64(f) => {
                let number_formatter = LocalizedNumberFormatter::new("en");
                let formatted_number = number_formatter.format_f64(*f);
                match formatted_number {
                    Ok(formatted_number) => {
                        cxx_qt_lib::QVariant::from(&QString::from(formatted_number))
                    }
                    Err(e) => cxx_qt_lib::QVariant::from(&QString::from(format!("{:?}", e))),
                }
            }

            Date(d) => {
                let o_date = NaiveDate::from_epoch_days(*d);
                let naive_date = if let Some(d) = o_date {
                    d
                } else {
                    NaiveDate::from_epoch_days(0).unwrap()
                };
                let date = naive_date.format("YYYY-MM-DD").to_string();

                println!("{}", naive_date);

                cxx_qt_lib::QVariant::from(&QString::from(date))
            }
            Datetime(d, time_unit, time_zone) => QVariant::from_datetime(d, time_unit, time_zone),
            DatetimeOwned(d, time_unit, time_zone) => {
                if let Some(tz) = time_zone {
                    QVariant::from_datetime(d, time_unit, &Some(&**tz))
                } else {
                    QVariant::from_datetime(d, time_unit, &None)
                }
            }
            Duration(t, time_unit) => {
                let unit_divisor = match time_unit {
                    TimeUnit::Nanoseconds => 1000000,
                    TimeUnit::Microseconds => 1000,
                    TimeUnit::Milliseconds => 1,
                };
                cxx_qt_lib::QVariant::from(&(t / unit_divisor))
            }
            Time(t) => cxx_qt_lib::QVariant::from(&QTime::from_msecs_since_start_of_day(
                (t / 1000000) as i32,
            )),
            Categorical(idx, categories) => QVariant::from_categorical(idx, categories),
            CategoricalOwned(idx, categories) => QVariant::from_categorical(idx, categories),
            Enum(idx, categories) => QVariant::from_categorical(idx, categories),
            EnumOwned(idx, categories) => QVariant::from_categorical(idx, categories),
            _ => cxx_qt_lib::QVariant::default(),
        }
    }
    fn from_datetime(d: &i64, time_unit: &TimeUnit, time_zone: &Option<&TimeZone>) -> Self {
        let time_zone = if let Some(tz) = time_zone {
            if let Ok(tz) = (*tz).to_chrono() {
                tz
            } else {
                Tz::UTC
            }
        } else {
            Tz::UTC
        };

        let utc_date_time = match time_unit {
            TimeUnit::Nanoseconds => chrono::DateTime::from_timestamp_nanos(*d).naive_utc(),
            TimeUnit::Microseconds => chrono::DateTime::from_timestamp_micros(*d)
                .unwrap()
                .naive_utc(),
            TimeUnit::Milliseconds => chrono::DateTime::from_timestamp_millis(*d)
                .unwrap()
                .naive_utc(),
        };

        let local_date_time = utc_date_time.and_local_timezone(time_zone).unwrap();
        let date_time_string = local_date_time.format("YYYY-MM-DD").to_string();
        cxx_qt_lib::QVariant::from(&QString::from(date_time_string))
    }

    fn from_categorical(idx: &CatSize, categories: &Arc<CategoricalMapping>) -> QVariant {
        let mut c = QString::from("");
        let oc = (*categories).cat_to_str(*idx);
        if let Some(c2) = oc {
            c = QString::from(c2);
        }
        cxx_qt_lib::QVariant::from(&c)
    }
}

impl qobject::TableManagerModel {
    fn column_count(&self, _parent: &QModelIndex) -> i32 {
        match &self.rust().table_model {
            None => 0,

            Some(table_model) => {
                if let Ok(d) = table_model.column_count() {
                    d as i32
                } else {
                    0
                }
            }
        }
    }

    fn row_count(&self, _parent: &QModelIndex) -> i32 {
        match &self.rust().table_model {
            None => 0,

            Some(table_model) => {
                if let Ok(d) = table_model.row_count() {
                    d as i32
                } else {
                    0
                }
            }
        }
    }

    fn data(self: &TableManagerModel, index: &QModelIndex, role: i32) -> QVariant {
        let row = index.row() as usize;
        let column = index.column() as usize;

        match &self.rust().table_model {
            None => QVariant::from(&QString::from("No table model set")),

            Some(table_model) => match table_model.data(row, column, role) {
                Ok(data) => QVariant::from_any_value(&data),
                Err(e) => QVariant::from(&QString::from(format!("{:?}", e))),
            },
        }
    }

    fn role_names(self: &TableManagerModel) -> QHash_i32_QByteArray {
        match &self.rust().table_model {
            None => QHash_i32_QByteArray::default(),

            Some(table_model) => {
                if let Ok(role_names) = table_model.role_names() {
                    let mut roles = QHash_i32_QByteArray::default();
                    for (role, name) in role_names.iter() {
                        roles.insert(*role, name.into());
                    }
                    roles
                } else {
                    QHash_i32_QByteArray::default()
                }
            }
        }
    }

    fn header_data(
        self: &TableManagerModel,
        section: i32,
        orientation: Orientation,
        role: i32,
    ) -> QVariant {
        match &self.rust().table_model {
            None => QVariant::from(&QString::from("No table model set")),
            Some(table_model) => match table_model.header_data(section, orientation, role) {
                Ok(data) => {
                    println!("TableManagerModel::header_data({section}, {orientation:?}, {role}) = {data:?}");
                    QVariant::from(&QString::from(format!("{:?}", data)))
                    //QVariant::from_any_value(&data)
                },
                Err(e) => QVariant::from(&QString::from(format!("{:?}", e))),
            },
        }
    }
}
