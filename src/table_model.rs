use cxx_qt::CxxQtType;
use cxx_qt_lib::{QVariant, QModelIndex, QString};

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
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[base = QAbstractTableModel]
        type TableModel = super::TableModelRust;

        #[cxx_override]
        #[rust_name = "column_count"]
        fn columnCount(self: &TableModel, parent: &QModelIndex) -> i32;

        #[cxx_override]
        #[rust_name = "row_count"]
        fn rowCount(self: &TableModel, parent: &QModelIndex) -> i32;

        #[cxx_override]
        fn data(self: &TableModel, index: &QModelIndex, role: i32) -> QVariant;

        #[cxx_override]
        #[rust_name = "role_names"]
        fn roleNames(self: &TableModel) -> QHash_i32_QByteArray;
    }
}

pub struct TableModelRust {
    column_header: Vec<QString>,
    row_header: Vec<QString>,
    contents: Vec<Vec<QVariant>>,
}

impl Default for TableModelRust {
    fn default() -> Self {
        Self {
            column_header: vec![],
            row_header: vec![],
            contents: vec![],
        }
    }
}

use qobject::*;

impl qobject::TableModel {
    fn column_count(self : &TableModel, _parent: &QModelIndex) -> i32 {
        self.rust().column_header.len() as i32
    }

    fn row_count(self : &TableModel, _parent: &QModelIndex) -> i32 {
        self.rust().row_header.len() as i32
    }

    fn data(self : &TableModel, index: &QModelIndex, _role: i32) -> QVariant {
        let row = index.row() as usize;
        let col = index.column() as usize;
        self.rust().contents[row][col].clone()
    }

    fn role_names(self: &TableModel) -> QHash_i32_QByteArray {
        let mut hash = QHash_i32_QByteArray::default();
        hash.insert(0, "text".into());
        hash
    }
}
