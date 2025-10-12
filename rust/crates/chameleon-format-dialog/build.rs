use std::path::PathBuf;
use cxx_qt_build::{CxxQtBuilder, QmlModule};
use build_utils::copy_dir_recursive;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let cargo_manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap().as_str());

    let lib_icu = vcpkg::Config::new().emit_includes(true).find_package("icu");
    let mut paths : Vec<String> = Vec::new();

    match lib_icu {
        Ok(lib_icu) => {
            if !lib_icu.include_paths.is_empty() {
                paths = lib_icu
                    .include_paths
                    .iter()
                    .map(|s| s.display().to_string())
                    .collect::<Vec<_>>();
                println!("cargo:include={}", paths.join(","));
            }
        }
        Err(e) => {
            println!("note, vcpkg did not find icu: {}", e);
        }
    }


    CxxQtBuilder::new()
        // Link Qt's Network library
        // - Qt Core is always linked
        // - Qt Gui is linked by enabling the qt_gui Cargo feature of cxx-qt-lib.
        // - Qt Qml is linked by enabling the qt_qml Cargo feature of cxx-qt-lib.
        // - Qt Qml requires linking Qt Network on macOS
        .qt_module("Network")
        .qml_module(QmlModule::<&str, &str> {
            uri: "chameleon.dialogs.format",
            qml_files: &["qml/FormatDialog.qml"],
            ..Default::default()
        })
        .files([
                "src/locale_selector_model.rs",
                "src/format.rs",
                "src/locale.rs",
                "src/number_sign_display_selector_model.rs",
                "src/notion_selector_model.rs",
                "src/model.rs"
               ])
        .cc_builder(|cc| {
            cc.includes(paths.clone());
            cc.include(cargo_manifest_dir.clone().join("../../../.."));
            cc.flag("/utf-8");
            cc.flag("/std:c++17");
            cc.files(["cpp/format.cpp", "cpp/locale.cpp", "cpp/units.cpp"]);

        })
        .build();


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



}