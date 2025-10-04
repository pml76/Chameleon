use crate::dialogs::format_dialog::notion_selector_model::ffi::ENotion;

/**
  * Holds the Model for the notion of a number format.
  */

#[cxx_qt::bridge]
mod ffi {
    #[repr(i32)]
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
    }

}

pub struct NotionSelectorModelRust {
    notions: Vec<ENotion>,
    strings: Vec<String>,
}

impl Default for NotionSelectorModelRust {
    fn default() -> Self {
        Self {
            notions: vec![ENotion::Scientific, ENotion::Engineering, ENotion::CompactShort, ENotion::CompactLong, ENotion::Simple],
            strings: vec!["Scientific".to_string(), "Engineering".to_string(), "Compact Short".to_string(), "Compact Long".to_string(), "Simple".to_string()],
        }
    }
}