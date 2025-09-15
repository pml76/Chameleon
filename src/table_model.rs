use cxx_qt_lib::QVariant;

#[cxx_qt::bridge]
mod qobject {

    unsafe extern "C++" {
        include!(<QAbstractTableModel>);
        type QAbstractTableModel;

        include!("cxx_qt_lib/qmodelindex.h");
        type QModelIndex = cxx_qt_lib::QModelIndex;
        
        include!("cxx_qt_lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[base = "QAbstractTableModel"]
        type TableModel = super::TableModelRust;
    }
}

struct TableModelRust {
    contents: Vec<Vec<QVariant>>
}