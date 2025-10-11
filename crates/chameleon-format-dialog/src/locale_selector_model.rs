use cxx_qt::CxxQtType;
use cxx_qt_lib::QByteArray;
use cxx_qt_lib_additions::ItemDataRole;
use crate::locale::{LocaleInformation, get_locale_information, OutputFor};

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

        #[namespace = "Qt"]
        type Orientation = cxx_qt_lib_additions::Orientation;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[base = QAbstractTableModel]
        type LocaleSelectorModel = super::LocaleSelectorModelRust;

        #[cxx_override]
        #[rust_name = "column_count"]
        fn columnCount(self: &LocaleSelectorModel, parent: &QModelIndex) -> i32;

        #[cxx_override]
        #[rust_name = "row_count"]
        fn rowCount(self: &LocaleSelectorModel, parent: &QModelIndex) -> i32;

        #[cxx_override]
        #[rust_name = "data"]
        fn data(self: &LocaleSelectorModel, index: &QModelIndex, role: i32) -> QVariant;

        #[cxx_override]
        #[rust_name = "role_names"]
        fn roleNames(self: &LocaleSelectorModel) -> QHash_i32_QByteArray;


    }
}

pub struct LocaleSelectorModelRust {
    locals:  Vec<LocaleInformation>,
}

impl LocaleSelectorModelRust {
    pub(crate) fn default_locale_index(&self) -> Option<i32> {
        for (index, locale) in self.locals.iter().enumerate() {
            if locale.locale_name == "en" {
                return Some(index as i32);
            }
        }
        None
    }

    pub(crate) fn find_locale_index(&self, locale_name: &str) -> Option<i32> {
        for (index, locale) in self.locals.iter().enumerate() {
            if locale.locale_name == locale_name {
                return Some(index as i32);
            }
        }
        None
    }

    pub(crate) fn find_locale_name(&self, index: i32) -> Option<String> {
        if index < 0 || index >= self.locals.len() as i32 {
            return None;
        }
        Some(self.locals[index as usize].locale_name.clone())
    }
}


impl Default for LocaleSelectorModelRust {
    fn default() -> Self {
        let mut ls = get_locale_information(OutputFor::AllLocales);
        ls.sort_by(|a, b| a.locale_display_name.cmp(&b.locale_display_name));

        LocaleSelectorModelRust {
            locals: ls,
        }
    }
}


use qobject::*;

impl qobject::LocaleSelectorModel {

    fn column_count(self: &LocaleSelectorModel, _parent: &QModelIndex) -> i32 {
        1
    }

    fn row_count(self: &LocaleSelectorModel, _parent: &QModelIndex) -> i32 {
        self.rust().locals.len() as i32
    }

    fn role_names(self: &LocaleSelectorModel) -> QHash_i32_QByteArray {
        let mut hash = QHash_i32_QByteArray::default();
        hash.insert(ItemDataRole::DisplayRole.repr, QByteArray::from("text"));
        hash.insert(ItemDataRole::EditRole.repr, QByteArray::from("value"));

        hash
    }

    fn data(self: &LocaleSelectorModel, index: &QModelIndex, role: i32) -> QVariant {
        if role != ItemDataRole::DisplayRole.repr {
            return QVariant::default();
        }
        let locale = &self.rust().locals[index.row() as usize];
        QVariant::from(&QString::from(locale.locale_display_name.as_str()))
    }
}