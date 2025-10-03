
/**
  * Holds the Model for the notion of a number format.
  */

#[cxx_qt::bridge]
mod ffi {
    enum ENotion {
        Scientific = 0,
        Engineering = 1,
        CompactShort = 2,
        CompactLong = 3,
        Simplex = 4,
    }

    unsafe extern "C++" {
        type ENotion;

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
}