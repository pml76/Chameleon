use cxx_qt_build::{CxxQtBuilder, QmlModule};

use std::path::PathBuf;
use chameleon_settings::*;
use build_utils::copy_dir_recursive;



fn main() {

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let cargo_manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap().as_str());



    CxxQtBuilder::new()
        // Link Qt's Network library
        // - Qt Core is always linked
        // - Qt Gui is linked by enabling the qt_gui Cargo feature of cxx-qt-lib.
        // - Qt Qml is linked by enabling the qt_qml Cargo feature of cxx-qt-lib.
        // - Qt Qml requires linking Qt Network on macOS
        .qt_module("Network")
        .qml_module(QmlModule::<&str,&str> {
            uri: "chameleon.main",
            qml_files: &["qml/main.qml"],
            ..Default::default()
        })
        .files(["src/python_dataframe_model.rs"])
        .build();



    let mut python_dir = PathBuf::from(cargo_manifest_dir.clone());
    python_dir.pop();
    python_dir.pop();
    python_dir.pop();
    python_dir = python_dir.join(".venv");

    if !python_dir.exists() {
        panic!(
            "Python virtual environment not found in {}. Please create it first by running `python -m venv .venv` in the project root directory.", python_dir.display());
    }

    copy_dir_recursive(
        format!("{}/qt-build-utils/qml_modules", out_dir),
        format!("{}/../../../qml_modules", out_dir),
    )
    .expect("Failed to copy qml modules");

    copy_dir_recursive(
        format!("{}/cxxqtbuild/", out_dir),
        format!("{}/../../../cxxqtbuild", out_dir),
    )
    .expect("Failed to copy cxxqtbuild");


    let mut qml_dir = PathBuf::from(cargo_manifest_dir.clone());
    qml_dir.pop();
    qml_dir.pop();
    qml_dir.push("../../../qml");

    let settings = GlobalSettings {
        qml_directory: qml_dir,
        default_locale: "en".to_string(),
        python_base_directory: PathBuf::from(python_dir.clone()),
    };

    let mut path = PathBuf::from(out_dir);
    path.pop();
    path.pop();
    path.pop();
    path.pop();
    path.pop();
    path = path.join(SETTINGS_FILE_NAME);
    let file = std::fs::File::create(path).unwrap();
    serde_json::to_writer(
        file,
        &settings,
    ).expect("Failed to write global settings");

}
