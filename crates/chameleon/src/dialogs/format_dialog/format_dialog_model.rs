use cxx_qt::CxxQtType;
use cxx_qt_lib::QByteArray;
use cxx_qt_lib_additions::ItemDataRole;
use crate::dialogs::format_dialog_model::qobject::FormatDialogModel;
use crate::locale::{get_locale_information, LocaleInformation, OutputFor};

#[cxx_qt::bridge]
mod qobject {

    unsafe extern "C++" {
        include!(<QAbstractItemModel>);
        type QAbstractItemModel;

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
        #[base = QAbstractItemModel]
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
    }
}

pub struct FormatDialogModelRust {
    locals: Vec<LocaleInformation>,

}

impl Default for FormatDialogModelRust {
    fn default() -> Self {
        FormatDialogModelRust {
            locals: get_locale_information(OutputFor::AllLocales),
        }
    }
}


use qobject::*;

impl qobject::FormatDialogModel {
    fn column_count(self: &FormatDialogModel, parent: &QModelIndex) -> i32 {
        1
    }

    fn row_count(self: &FormatDialogModel, parent: &QModelIndex) -> i32 {
        self.rust().locals.len() as i32
    }

    fn role_names(self: &FormatDialogModel) -> QHash_i32_QByteArray {
        let mut hash = QHash_i32_QByteArray::default();
        hash.insert(ItemDataRole::DisplayRole.repr, QByteArray::from("locale_display_name"));
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