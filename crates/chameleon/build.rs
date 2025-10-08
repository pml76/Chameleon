use cxx_qt_build::{CxxQtBuilder, QmlModule};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use chameleon_settings::*;

pub fn copy_dir_recursive<S: AsRef<Path>, D: AsRef<Path>>(src: S, dst: D) -> io::Result<()> {
    fn copy_dir(src: &Path, dst: &Path) -> io::Result<()> {
        if !src.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Source '{}' is not a directory", src.display()),
            ));
        }
        fs::create_dir_all(dst)?;

        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if file_type.is_dir() {
                copy_dir(&src_path, &dst_path)?;
            } else if file_type.is_file() {
                // Overwrites if it exists
                fs::create_dir_all(dst_path.parent().unwrap())?;
                fs::copy(&src_path, &dst_path)?;
            } else if file_type.is_symlink() {
                // Follow the symlink: copy target contents.
                // If the link points to a directory, recurse; otherwise copy the file.
                let meta = fs::metadata(&src_path)?;
                if meta.is_dir() {
                    copy_dir(&src_path, &dst_path)?;
                } else {
                    fs::copy(&src_path, &dst_path)?;
                }
            }
        }
        Ok(())
    }

    copy_dir(src.as_ref(), dst.as_ref())
}

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
        .qml_module(QmlModule {
            uri: "chameleon.main",
            rust_files: &[
                "src/python_dataframe_model.rs",
            ],
            qml_files: &["../../qml/main.qml"],
            ..Default::default()
        })
        .qml_module(QmlModule::<&str, &str> {
            uri: "chameleon.dialogs.format",
            rust_files: &[
                "src/dialogs/format_dialog/locale_selector_model.rs",
                "src/dialogs/format_dialog/format.rs",
                "src/dialogs/format_dialog/locale.rs",
                "src/dialogs/format_dialog/number_sign_display_selector_model.rs",
                "src/dialogs/format_dialog/notion_selector_model.rs",
            ],
            qml_files: &["../../qml/FormatDialog.qml"],
            ..Default::default()
        })
        .cc_builder(|cc| {
            cc.includes(paths.clone());
            cc.include(cargo_manifest_dir.clone().join(".."));
            cc.flag("/utf-8");
            cc.flag("/std:c++17");
            cc.files(["cpp/dialogs/format_dialog/format.cpp", "cpp/dialogs/format_dialog/locale.cpp"]);

        })
        .build();



    let mut python_dir = PathBuf::from(cargo_manifest_dir.clone());
    python_dir.pop();
    python_dir.pop();
    python_dir = python_dir.join(".venv");

    if !python_dir.exists() {
        panic!("Python virtual environment not found. Please create it first by running `python -m venv .venv` in the project root directory.");
    }

    copy_dir_recursive(
        format!("{}/qt-build-utils/qml_modules", out_dir),
        format!("{}/../../../qml_modules", out_dir),
    )
    .unwrap(); 


    copy_dir_recursive(
        format!("{}/cxxqtbuild/", out_dir),
        format!("{}/../../../cxxqtbuild", out_dir),
    )
    .unwrap();

    let mut qml_dir = PathBuf::from(cargo_manifest_dir.clone());
    qml_dir.pop();
    qml_dir.pop();
    qml_dir.push("qml");

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
