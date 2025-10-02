use cxx_qt::CxxQtType;
use cxx_qt_lib::QByteArray;
use cxx_qt_lib_additions::ItemDataRole;
use crate::locale::{LocaleInformation, get_locale_information, OutputFor};

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
        type FormatDialogModel = super::FormatDialogModelRust;

        #[cxx_override]
        #[rust_name = "column_count"]
        fn columnCount(self: &FormatDialogModel, parent: &QModelIndex) -> i32;

        #[cxx_override]
        #[rust_name = "row_count"]
        fn rowCount(self: &FormatDialogModel, parent: &QModelIndex) -> i32;

        #[cxx_override]
        #[rust_name = "data"]
        fn data(self: &FormatDialogModel, index: &QModelIndex, role: i32) -> QVariant;

        #[cxx_override]
        #[rust_name = "role_names"]
        fn roleNames(self: &FormatDialogModel) -> QHash_i32_QByteArray;

        #[rust_name = "get_current_index"]
        #[qinvokable]
        fn getCurrentIndex(self: &FormatDialogModel) -> i32;

    }
}

pub struct FormatDialogModelRust {
    locals:  Vec<LocaleInformation>,
    current_index : Option<i32>,

}

fn find_en_locale(ls: &Vec<LocaleInformation>) -> Option<i32> {
     let mut index = 0;
    
    for l in ls.iter() {
        if l.locale_name == "en" {
            return Some(index)
        }      
        index += 1;
    }
    
    None
}

impl Default for FormatDialogModelRust {
    fn default() -> Self {
        let ls = get_locale_information(OutputFor::AllLocales);
        let mut format_dialog_model = FormatDialogModelRust {
            locals: ls,
            current_index: None,
        };

        let en_index = find_en_locale(&format_dialog_model.locals);

        format_dialog_model.current_index = en_index;
        format_dialog_model
    }
}


use qobject::*;

impl qobject::FormatDialogModel {

    fn get_current_index(self: &FormatDialogModel) -> i32 {
        if let Some(index) = self.rust().current_index {
            return index;
        }
        0
    }

    fn column_count(self: &FormatDialogModel, _parent: &QModelIndex) -> i32 {
        1
    }

    fn row_count(self: &FormatDialogModel, _parent: &QModelIndex) -> i32 {
        self.rust().locals.len() as i32
    }

    fn role_names(self: &FormatDialogModel) -> QHash_i32_QByteArray {
        let mut hash = QHash_i32_QByteArray::default();
        hash.insert(ItemDataRole::DisplayRole.repr, QByteArray::from("text"));
        hash.insert(ItemDataRole::EditRole.repr, QByteArray::from("value"));

        hash
    }

    fn data(self: &FormatDialogModel, index: &QModelIndex, role: i32) -> QVariant {
        if role != ItemDataRole::DisplayRole.repr {
            return QVariant::default();
        }
        let locale = &self.rust().locals[index.row() as usize];
        QVariant::from(&QString::from(locale.locale_display_name.as_str()))
    }
}