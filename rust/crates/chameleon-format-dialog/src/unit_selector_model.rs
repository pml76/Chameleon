use cxx_qt::CxxQtType;
use cxx_qt_lib_additions::ItemDataRole;
use crate::units::get_unit_types;

#[cxx_qt::bridge]
pub mod qobject {

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
        type UnitTypeSelectorModel = super::UnitTypeSelectorModelRust;

        #[cxx_override]
        #[rust_name = "column_count"]
        fn columnCount(self: &UnitTypeSelectorModel, parent: &QModelIndex) -> i32;

        #[cxx_override]
        #[rust_name = "row_count"]
        fn rowCount(self: &UnitTypeSelectorModel, parent: &QModelIndex) -> i32;

        #[cxx_override]
        #[rust_name = "data"]
        fn data(self: &UnitTypeSelectorModel, index: &QModelIndex, role: i32) -> QVariant;

        #[cxx_override]
        #[rust_name = "role_names"]
        fn roleNames(self: &UnitTypeSelectorModel) -> QHash_i32_QByteArray;

    }
}

pub struct UnitTypeSelectorModelRust {
    unit_types: Vec<String>,
}

impl Default for UnitTypeSelectorModelRust {
    fn default() -> Self {
        Self {
            unit_types: get_unit_types(),
        }
    }
}

use qobject::*;

impl qobject::UnitTypeSelectorModel {
    fn role_names(self: &UnitTypeSelectorModel) -> QHash_i32_QByteArray {
        let mut role_names = QHash_i32_QByteArray::default();
        role_names.insert(ItemDataRole::DisplayRole.repr, "text".into());
        role_names.insert(ItemDataRole::EditRole.repr, "value".into());
        role_names
    }
    
    fn column_count(self: &UnitTypeSelectorModel, _parent: &QModelIndex) -> i32 {
        1
    }
    
    fn row_count(self: &UnitTypeSelectorModel, _parent: &QModelIndex) -> i32 {
        self.rust().unit_types.len() as i32
    }
    
    fn data(self: &UnitTypeSelectorModel, index: &QModelIndex, role: i32) -> QVariant {
        let role = ItemDataRole{repr: role};
        if role == ItemDataRole::DisplayRole || role == ItemDataRole::EditRole {
            if index.row() < self.rust().unit_types.len() as i32 {
                return QVariant::from(&QString::from(self.rust().unit_types[index.row() as usize].clone()));
            }
        }
        QVariant::default()
    }
}