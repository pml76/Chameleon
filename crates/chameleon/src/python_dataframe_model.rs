use crate::python_dataframe_model::qobject::DataFrameModel;
use crate::time_and_dates::tz_to_qtimezone;
use chrono::NaiveDate;
use chrono_tz::Tz;
use cxx_qt::CxxQtType;
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

pub struct DataFrameModelRust {
    df: DataFrame,
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

fn datetime_to_qvariant(d: &i64, time_unit: &TimeUnit, time_zone: &Option<&TimeZone>) -> QVariant {
    let unit_divisor = match time_unit {
        TimeUnit::Nanoseconds => 1000000,
        TimeUnit::Microseconds => 1000,
        TimeUnit::Milliseconds => 1,
    };
    let time_zone = if let Some(tz) = time_zone {
        if let Ok(tz) = (*tz).to_chrono() {
            tz
        } else {
            Tz::UTC
        }
    } else {
        Tz::UTC
    };
    let mut qtimezone = QTimeZone::utc();
    let tz = tz_to_qtimezone(time_zone);
    if let Some(_) = tz.as_ref() {
        qtimezone = tz;
    }
    QVariant::from(&QDateTime::from_msecs_since_epoch(
        d / unit_divisor,
        qtimezone.as_ref().unwrap(),
    ))
}

fn any_value_to_qvariant(value: &AnyValue) -> QVariant {
    match value {
        Boolean(b) => QVariant::from(b),
        String(str) => QVariant::from(&QString::from(*str)),
        UInt8(u) => QVariant::from(u),
        UInt16(u) => QVariant::from(u),
        UInt32(u) => QVariant::from(u),
        UInt64(u) => QVariant::from(u),
        Int8(i) => QVariant::from(i),
        Int16(i) => QVariant::from(i),
        Int32(i) => QVariant::from(i),
        Int64(i) => QVariant::from(i),
        Float32(f) => QVariant::from(f),
        Float64(f) => QVariant::from(f),
        Date(d) => QVariant::from(&QDate::new(1970, 1, 1).add_days(*d as i64)),
        Datetime(d, time_unit, time_zone) => datetime_to_qvariant(d, time_unit, time_zone),
        DatetimeOwned(d, time_unit, time_zone) => {
            if let Some(tz) = time_zone {
                datetime_to_qvariant(d, time_unit, &Some(&**tz))
            } else {
                datetime_to_qvariant(d, time_unit, &None)
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
                return any_value_to_qvariant(value);
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
                return QVariant::from( & QString::from(name.to_string()));
                }
                }
            },
            Orientation::Vertical => {
                let row = section as usize;
                if row < self.rust().df.shape().0 {
                return QVariant::from( & (row as u64));
                }
            },
            _ => {}
        }

        QVariant::default()
    }
}
