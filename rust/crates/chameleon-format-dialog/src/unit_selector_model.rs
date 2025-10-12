use std::pin::Pin;
use cxx_qt::CxxQtType;
use cxx_qt_lib_additions::ItemDataRole;
use crate::units::{get_available_units_for_type, get_unit_types};

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

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[base = QAbstractTableModel]
        type UnitSelectorModel = super::UnitSelectorModelRust;


        #[cxx_override]
        #[rust_name = "column_count"]
        fn columnCount(self: &UnitSelectorModel, parent: &QModelIndex) -> i32;

        #[cxx_override]
        #[rust_name = "row_count"]
        fn rowCount(self: &UnitSelectorModel, parent: &QModelIndex) -> i32;

        #[cxx_override]
        #[rust_name = "data"]
        fn data(self: &UnitSelectorModel, index: &QModelIndex, role: i32) -> QVariant;

        #[cxx_override]
        #[rust_name = "role_names"]
        fn roleNames(self: &UnitSelectorModel) -> QHash_i32_QByteArray;

        #[inherit]
        #[rust_name = "begin_reset_model"]
        fn beginResetModel(self: Pin<&mut UnitSelectorModel>);

        #[inherit]
        #[rust_name = "end_reset_model"]
        fn endResetModel(self: Pin<&mut UnitSelectorModel>);

        #[qinvokable]
        #[rust_name = "set_unit_type"]
        fn setUnitType(self: Pin<&mut UnitSelectorModel>, unit_type: &QString);
    }
}

pub struct UnitSelectorModelRust {
    units: Vec<String>,
}


pub struct UnitTypeSelectorModelRust {
    unit_types: Vec<String>,
}

impl UnitTypeSelectorModelRust {
    pub(crate) fn default_unit_type(&self) -> String {
        if self.unit_types.len() > 0 {
            return self.unit_types[0].clone();
        }
        "".to_string()
    }
}

impl Default for UnitTypeSelectorModelRust {
    fn default() -> Self {
        let mut strings = get_unit_types();
        strings.sort();

        Self {
            unit_types: strings,
        }
    }
}

impl Default for UnitSelectorModelRust {
    fn default() -> Self {
        let unit_types = UnitTypeSelectorModelRust::default();
        let unit_type = unit_types.default_unit_type();
        let mut units = get_available_units_for_type(&unit_type);
        units.sort();

        Self {
            units,
        }
    }
}

use qobject::*;


impl qobject::UnitSelectorModel {
    fn role_names(self: &UnitSelectorModel) -> QHash_i32_QByteArray {
        let mut role_names = QHash_i32_QByteArray::default();
        role_names.insert(ItemDataRole::DisplayRole.repr, "text".into());
        role_names.insert(ItemDataRole::EditRole.repr, "value".into());
        role_names
    }

    fn column_count(self: &UnitSelectorModel, _parent: &QModelIndex) -> i32 {
        1
    }

    fn row_count(self: &UnitSelectorModel, _parent: &QModelIndex) -> i32 {
        self.rust().units.len() as i32
    }

    fn data(self: &UnitSelectorModel, index: &QModelIndex, role: i32) -> QVariant {
        let role = ItemDataRole{repr: role};
        if role == ItemDataRole::DisplayRole || role == ItemDataRole::EditRole {
            if index.row() < self.rust().units.len() as i32 {
                let unit = &self.rust().units[index.row() as usize];
                return QVariant::from(&QString::from(unit));
            }
        }

        QVariant::default()
    }

    fn set_unit_type(mut self: Pin<&mut UnitSelectorModel>, unit_type: &QString) {
        
        self.as_mut().begin_reset_model();
        
        let unit_type = unit_type.to_string();
        
        self.as_mut().rust_mut().units = get_available_units_for_type(&unit_type);
        self.as_mut().rust_mut().units.sort();
        
        self.as_mut().end_reset_model();

    }
}

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