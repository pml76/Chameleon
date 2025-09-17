use cxx_qt::CxxQtType;
use cxx_qt_lib::{QVariant, QModelIndex, QString};
use polars::prelude::*;
use crate::python_dataframe_model::qobject::DataFrameModel;

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
    }

}


pub struct DataFrameModelRust {
    df: DataFrame,
}

impl Default for DataFrameModelRust {
    fn default() -> Self {
        DataFrameModelRust {
            df: DataFrame::empty(),
        }
    }
}


impl qobject::DataFrameModel {
    fn column_count(self: &DataFrameModel, parent: &QModelIndex) -> i32 {
        self.rust().df.shape().1 as i32
    }

    fn row_count(self: &DataFrameModel, parent: &QModelIndex) -> i32 {
        self.rust().df.shape().0 as i32
    }

    fn data (self: &DataFrameModel, index: &QModelIndex, role: i32) -> QVariant {
        let row = index.row() as usize;
        let col = index.column() as usize;
        let shape = self.rust().df.shape();

        if row < shape.1 && col < shape.0 {
            let schema = self.rust().df.schema();
            if let Some((_name, dtype)) = schema.get_at_index(col) {

            }
        }

        QVariant::default()
    }
}