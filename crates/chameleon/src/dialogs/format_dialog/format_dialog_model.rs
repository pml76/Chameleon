
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
    }
}

pub struct FormatDialogModelRust {
    i: i32,
}

impl Default for FormatDialogModelRust {
    fn default() -> Self {
        FormatDialogModelRust {
            i: 0,
        }
    }
}


