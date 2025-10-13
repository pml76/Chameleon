use cxx_qt::CxxQtType;
use cxx_qt_lib_additions::ItemDataRole;
#[cxx_qt::bridge]
pub mod qobject {
    #[repr(i32)]
    #[derive(Debug)]
    enum ENumberSignDisplay {
        Auto = 0,
        Always = 1,
        Never = 2,
        Accounting = 3,
        AccountingAlways = 4,
        ExceptZero = 5,
        SignNegative = 6,
        AccountingNegative = 7,
        AccountingExceptZero = 8,
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
        type NumberSignDisplaySelectorModel = super::NumberSignDisplaySelectorModelRust;

        #[cxx_override]
        #[rust_name = "column_count"]
        fn columnCount(self: &NumberSignDisplaySelectorModel, parent: &QModelIndex) -> i32;

        #[cxx_override]
        #[rust_name = "row_count"]
        fn rowCount(self: &NumberSignDisplaySelectorModel, parent: &QModelIndex) -> i32;

        #[cxx_override]
        #[rust_name = "data"]
        fn data(self: &NumberSignDisplaySelectorModel, index: &QModelIndex, role: i32) -> QVariant;

        #[cxx_override]
        #[rust_name = "role_names"]
        fn roleNames(self: &NumberSignDisplaySelectorModel) -> QHash_i32_QByteArray;

    }

}

pub struct NumberSignDisplaySelectorModelRust {
    sign_conventions: Vec<ENumberSignDisplay>,
    strings: Vec<String>,
}

impl NumberSignDisplaySelectorModelRust {
    pub(crate) fn get_number_sign_displays(&self) -> &Vec<ENumberSignDisplay> {
        &self.sign_conventions
    }
}

impl Default for NumberSignDisplaySelectorModelRust {
    fn default() -> Self {
        Self {
            sign_conventions: vec![ENumberSignDisplay::Auto, ENumberSignDisplay::Always, ENumberSignDisplay::Never, ENumberSignDisplay::Accounting, ENumberSignDisplay::AccountingAlways, ENumberSignDisplay::ExceptZero, ENumberSignDisplay::SignNegative, ENumberSignDisplay::AccountingNegative],
            strings: vec!["Auto".to_string(), "Always".to_string(), "Never".to_string(), "Accounting".to_string(), "Accounting Always".to_string(), "Except Zero".to_string(), "Sign Negative".to_string(), "Accounting Negative".to_string()],
        }
    }
}

pub use qobject::*;

impl qobject::NumberSignDisplaySelectorModel {

    fn role_names(self: &NumberSignDisplaySelectorModel) -> QHash_i32_QByteArray {
        let mut role_names = QHash_i32_QByteArray::default();
        role_names.insert(ItemDataRole::DisplayRole.repr, "text".into());
        role_names.insert(ItemDataRole::EditRole.repr, "value".into());
        role_names
    }

    fn column_count(self: &NumberSignDisplaySelectorModel, _parent: &QModelIndex) -> i32 {
        1
    }

    fn row_count(self: &NumberSignDisplaySelectorModel, _parent: &QModelIndex) -> i32 {
        self.rust().sign_conventions.len() as i32
    }

    fn data(self: &NumberSignDisplaySelectorModel, index: &QModelIndex, role: i32) -> QVariant {
        let role = ItemDataRole{repr: role};
        match role {
            ItemDataRole::DisplayRole => {
                let index = index.row();
                let string = self.rust().strings[index as usize].clone();
                QVariant::from(&QString::from(string))
            },
            ItemDataRole::EditRole => {
                let index = index.row() as usize;
                QVariant::from(&self.rust().sign_conventions[index].repr)
            },
            _ => {
                QVariant::default()
            }
        }
    }
}