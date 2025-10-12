use cxx_qt_build::{CxxQtBuilder};

fn main() {
    CxxQtBuilder::new()
        .file("src/qt_enums.rs")
        .build();
}