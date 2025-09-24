use cxx_qt_build::{CxxQtBuilder, QmlModule};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use chameleon_settings::*;
use conan2::ConanInstall;

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

    ConanInstall::new()
        .build("missing")
        .run()
        .parse()
        .emit();

    let fmt = vcpkg::Config::new().emit_includes(true).find_package("fmt");
    let mut paths : Vec<String> = Vec::new();
    
    match fmt {
        Ok(fmt) => {
            if !fmt.include_paths.is_empty() {
                paths = fmt
                    .include_paths
                    .iter()
                    .map(|s| s.display().to_string())
                    .collect::<Vec<_>>();
                println!("cargo:include={}", paths.join(","));
            }
        }
        Err(e) => {
            println!("note, vcpkg did not find zlib: {}", e);
        }
    }

    let mut binding = cxx_build::bridge("src/format.rs");
    let mut cxx_bridge = binding.file("cpp/format.cpp");

    for path in paths {
        cxx_bridge = cxx_bridge.include(path);
    }
    cxx_bridge = cxx_bridge.flag("/utf-8");
    cxx_bridge.compile("cxx_format");

    println!("cargo:rerun-if-changed=cpp/format.cpp");
    println!("cargo:rerun-if-changed=cpp/format.h");
    println!("cargo:rerun-if-changed=src/format.rs");

    CxxQtBuilder::new()
        // Link Qt's Network library
        // - Qt Core is always linked
        // - Qt Gui is linked by enabling the qt_gui Cargo feature of cxx-qt-lib.
        // - Qt Qml is linked by enabling the qt_qml Cargo feature of cxx-qt-lib.
        // - Qt Qml requires linking Qt Network on macOS
        .qt_module("Network")
        .qml_module(QmlModule {
            uri: "com.kdab.cxx_qt.demo",
            rust_files: &[
                "src/cxxqt_object.rs",
                "src/table_model.rs",
                "src/python_dataframe_model.rs",
            ],
            qml_files: &["../../qml/main.qml"],
            ..Default::default()
        })
        .build();

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let mut cargo_manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap().as_str());
    cargo_manifest_dir.pop();
    cargo_manifest_dir.pop();
    let python_dir = cargo_manifest_dir.join(".venv");

    if !python_dir.exists() {
        panic!("Python virtual environment not found. Please create it first by running `python -m venv .venv` in the project root directory.");
    }

    copy_dir_recursive(
        format!("{}/qt-build-utils/qml_modules", out_dir),
        format!("{}/../../../qml_modules", out_dir),
    )
    .unwrap();

    let mut loc_dir = PathBuf::from(out_dir.clone());
    loc_dir.pop();
    loc_dir.pop();
    loc_dir.pop();
    loc_dir.pop();
    loc_dir.pop();
    loc_dir.push("cldr-json");
    loc_dir.push("cldr-json");

    let settings = GlobalSettings {
        locales_directory: PathBuf::from(loc_dir.to_str().unwrap()),
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
