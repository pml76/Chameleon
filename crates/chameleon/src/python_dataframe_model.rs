use std::string::String;
use crate::format::{format_bool, format_f32, format_f64, format_i16, format_i32, format_i64, format_i8, format_str, format_u16, format_u32, format_u64, format_u8};
use crate::python_dataframe_model::qobject::DataFrameModel;
use chrono::NaiveDate;
use chrono_tz::Tz;
use cxx_qt::{Constructor, CxxQtType};
use cxx_qt_lib::{QDate, QDateTime, QModelIndex, QString, QTime, QTimeZone, QVariant};
use polars::datatypes::AnyValue::*;
use polars::df;
use polars::prelude::{AnyValue, CatSize, CategoricalMapping, DataFrame, TimeUnit, TimeZone};
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
        type DataFrameModel = super::DataFrameModelRust;

        #[cxx_override]
        #[rust_name = "column_count"]
        fn columnCount(self: &DataFrameModel, parent: &QModelIndex) -> i32;

        #[cxx_override]
        #[rust_name = "row_count"]
        fn rowCount(self: &DataFrameModel, parent: &QModelIndex) -> i32;

        #[cxx_override]
        #[rust_name = "data"]
        fn data(self: &DataFrameModel, index: &QModelIndex, role: i32) -> QVariant;

        #[cxx_override]
        #[rust_name = "role_names"]
        fn roleNames(self: &DataFrameModel) -> QHash_i32_QByteArray;

        #[cxx_override]
        #[rust_name = "header_data"]
        fn headerData(
            self: &DataFrameModel,
            section: i32,
            orientation: Orientation,
            role: i32,
        ) -> QVariant;
    }
}

use qobject::*;

struct Attache {
    name: QString,
    bool_format: String,
    string_format: String,
    u8_format: String,
    u16_format: String,
    u32_format: String,
    u64_format: String,
    i8_format: String,
    i16_format: String,
    i32_format: String,
    i64_format: String,
    f32_format: String,
    f64_format: String,
    date_format: String,
    datetime_format: String,
    time_format: String,
}

impl Attache {
    fn new(name: QString) -> Self {
        Self { name,
            bool_format: String::from("{}"),
            string_format: String::from("{}"),
            u8_format: String::from("{}"),
            u16_format: String::from("{}"),
            u32_format: String::from("{}"),
            u64_format: String::from("{}"),
            i8_format: String::from("{}"),
            i16_format: String::from("{}"),
            i32_format: String::from("{}"),
            i64_format: String::from("{}"),
            f32_format: String::from("{}"),
            f64_format: String::from("{}"),
            date_format: String::from("%Y-%m-%d"),
            datetime_format: String::from("%Y-%m-%d %H:%M:%S"),
            time_format: String::from("%H:%M:%S"),
        }
    }

    fn name(&self) -> QVariant {
        QVariant::from(&self.name)
    }
    fn datetime_to_qvariant(&self, d: &i64, time_unit: &TimeUnit, time_zone: &Option<&TimeZone>) -> QVariant {

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
            TimeUnit::Microseconds => chrono::DateTime::from_timestamp_micros(*d).unwrap().naive_utc(),
            TimeUnit::Milliseconds => chrono::DateTime::from_timestamp_millis(*d).unwrap().naive_utc(),
        };

        let local_date_time = utc_date_time.and_local_timezone(time_zone).unwrap();
        let date_time_string = local_date_time.format(&self.datetime_format).to_string();
        QVariant::from(&QString::from(date_time_string))
    }
    fn prepare_data(&self, value: &AnyValue) -> QVariant {
        match value {
            Boolean(b) =>
                QVariant::from(&QString::from(format_bool(&self.bool_format, b))),

            String(str) => QVariant::from(&QString::from(format_str(&self.string_format, &str.to_string()))),

            UInt8(u) => QVariant::from(&QString::from(format_u8(&self.u8_format, u))),
            UInt16(u) => QVariant::from(&QString::from(format_u16(&self.u16_format, u))),
            UInt32(u) => QVariant::from(&QString::from(format_u32(&self.u32_format, u))),
            UInt64(u) => QVariant::from(&QString::from(format_u64(&self.u64_format, u))),
            Int8(i) => QVariant::from(&QString::from(format_i8(&self.i8_format, i))),
            Int16(i) => QVariant::from(&QString::from(format_i16(&self.i16_format, i))),
            Int32(i) => QVariant::from(&QString::from(format_i32(&self.i32_format, i))),
            Int64(i) => QVariant::from(&QString::from(format_i64(&self.i64_format, i))),
            Float32(f) => QVariant::from(&QString::from(format_f32(&self.f32_format, f))),
            Float64(f) => QVariant::from(&QString::from(format_f64(&self.f64_format, f))),
            Date(d) => {
                let o_date = NaiveDate::from_epoch_days(*d);
                let naive_date = if let Some(d) = o_date { d } else {
                    NaiveDate::from_epoch_days(0).unwrap()
                };
                let date = naive_date.format(&self.date_format)
                    .to_string();

                println!("{}", naive_date);

                QVariant::from(&QString::from(date))
            },
            Datetime(d, time_unit, time_zone) => self.datetime_to_qvariant(d, time_unit, time_zone),
            DatetimeOwned(d, time_unit, time_zone) => {
                if let Some(tz) = time_zone {
                    self.datetime_to_qvariant(d, time_unit, &Some(&**tz))
                } else {
                    self.datetime_to_qvariant(d, time_unit, &None)
                }
            }
            Duration(t, time_unit) => {
                let unit_divisor = match time_unit {
                    TimeUnit::Nanoseconds => 1000000,
                    TimeUnit::Microseconds => 1000,
                    TimeUnit::Milliseconds => 1,
                };
                QVariant::from(&(t / unit_divisor))
            }
            Time(t) => QVariant::from(&QTime::from_msecs_since_start_of_day((t / 1000000) as i32)),
            Categorical(idx, categories) => categorical_entry_to_qvariant(idx, categories),
            CategoricalOwned(idx, categories) => categorical_entry_to_qvariant(idx, categories),
            Enum(idx, categories) => categorical_entry_to_qvariant(idx, categories),
            EnumOwned(idx, categories) => categorical_entry_to_qvariant(idx, categories),
            _ => QVariant::default(),
        }
    }
}

pub struct DataFrameModelRust {
    df: DataFrame,
    attaches: Vec<Attache>,
}

impl Default for DataFrameModelRust {
    fn default() -> Self {
        DataFrameModelRust {
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
            attaches: vec![],
        }
    }
}

fn categorical_entry_to_qvariant(idx: &CatSize, categories: &Arc<CategoricalMapping>) -> QVariant {
    let mut c = QString::from("");
    let oc = (*categories).cat_to_str(*idx);
    if let Some(c2) = oc {
        c = QString::from(c2);
    }
    QVariant::from(&c)
}


impl qobject::DataFrameModel {
    fn column_count(self: &DataFrameModel, _parent: &QModelIndex) -> i32 {
        self.rust().df.shape().1 as i32
    }

    fn row_count(self: &DataFrameModel, _parent: &QModelIndex) -> i32 {
        self.rust().df.shape().0 as i32
    }

    fn data(self: &DataFrameModel, index: &QModelIndex, _role: i32) -> QVariant {
        let row = index.row() as usize;
        let col = index.column() as usize;
        let shape = self.rust().df.shape();

        if row < shape.1 && col < shape.0 {
            let schema = self.rust().df.schema();
            if let Some((_name, _dtype)) = schema.get_at_index(col) {
                let column = self.rust().df.select_at_idx(col).unwrap();
                let value = &column.get(row).unwrap();
                let attache = Attache::new(QString::from("name"));
                return attache.prepare_data(value);

            }
        }

        QVariant::default()
    }

    fn role_names(self: &DataFrameModel) -> QHash_i32_QByteArray {
        let mut hash = QHash_i32_QByteArray::default();
        hash.insert(0, "display".into());
        hash
    }

    fn header_data(
        self: &DataFrameModel,
        section: i32,
        orientation: Orientation,
        _role: i32,
    ) -> QVariant {
        match orientation {
            Orientation::Horizontal => {
                let col = section as usize;
                if col < self.rust().df.shape().1 {
                    let schema = self.rust().df.schema();
                    if let Some((name, _dtype)) = schema.get_at_index(col) {
                        return QVariant::from(&QString::from(name.to_string()));
                    }
                }
            }
            Orientation::Vertical => {
                let row = section as usize;
                if row < self.rust().df.shape().0 {
                    return QVariant::from(&(row as u64));
                }
            }
            _ => {}
        }

        QVariant::default()
    }
}
