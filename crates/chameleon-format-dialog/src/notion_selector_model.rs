use cxx_qt::CxxQtType;
use cxx_qt_lib_additions::ItemDataRole;
use crate::notion_selector_model::qobject::ENotion;

/**
  * Holds the Model for the notion of a number format.
  */

#[cxx_qt::bridge]
pub mod qobject {
    #[repr(i32)]
    #[derive(Debug)]
    enum ENotion {
        Scientific = 0,
        Engineering = 1,
        CompactShort = 2,
        CompactLong = 3,
        Simple = 4,
    }

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
        type NotionSelectorModel = super::NotionSelectorModelRust;

        #[cxx_override]
        #[rust_name = "column_count"]
        fn columnCount(self: &NotionSelectorModel, parent: &QModelIndex) -> i32;

        #[cxx_override]
        #[rust_name = "row_count"]
        fn rowCount(self: &NotionSelectorModel, parent: &QModelIndex) -> i32;

        #[cxx_override]
        #[rust_name = "data"]
        fn data(self: &NotionSelectorModel, index: &QModelIndex, role: i32) -> QVariant;

        #[cxx_override]
        #[rust_name = "role_names"]
        fn roleNames(self: &NotionSelectorModel) -> QHash_i32_QByteArray;

        #[rust_name = "get_current_index"]
        #[qinvokable]
        fn getCurrentIndex(self: &NotionSelectorModel) -> i32;
    }

}

pub struct NotionSelectorModelRust {
    notions: Vec<ENotion>,
    strings: Vec<String>,
}

impl NotionSelectorModelRust {
    pub fn get_notions(&self) -> &Vec<ENotion> {
        &self.notions
    }
}

impl Default for NotionSelectorModelRust {
    fn default() -> Self {
        Self {
            notions: vec![ENotion::Scientific, ENotion::Engineering, ENotion::CompactShort, ENotion::CompactLong, ENotion::Simple],
            strings: vec!["Scientific".to_string(), "Engineering".to_string(), "Compact Short".to_string(), "Compact Long".to_string(), "Simple".to_string()],
        }
    }
}

use qobject::*;

impl NotionSelectorModel {

    fn get_current_index(self: &NotionSelectorModel) -> i32 {
        0
    }

    fn role_names(self: &NotionSelectorModel) -> QHash_i32_QByteArray {
        let mut role_names = QHash_i32_QByteArray::default();
        role_names.insert(ItemDataRole::DisplayRole.repr, "text".into());
        role_names.insert(ItemDataRole::EditRole.repr, "value".into());
        role_names
    }

    fn column_count(self: &NotionSelectorModel, _parent: &QModelIndex) -> i32 {
        1
    }

    fn row_count(self: &NotionSelectorModel, _parent: &QModelIndex) -> i32 {
        self.rust().strings.len() as i32
    }

    fn data(self: &NotionSelectorModel, index: &QModelIndex, role: i32) -> QVariant {
        let role = ItemDataRole{repr: role};
        if role == ItemDataRole::DisplayRole {
            QVariant::from(&QString::from(self.rust().strings[index.row() as usize].clone()))
        } else if role == ItemDataRole::EditRole {
            QVariant::from(&self.rust().notions[index.row() as usize].repr)
        } else {
            QVariant::default()
        }
    }
}