# Chameleon

This repository is a Rust + Qt (Qt Quick) application.

Answer to the issue: Yes — you can build a Qt Widgets application using Rust.

Below is a concise guide showing multiple approaches and a minimal working example.

---

Creating a Qt Widgets application in Rust

There are three practical approaches, depending on your needs:

1) Use the Rust Qt bindings (ritual crates: qt_core, qt_gui, qt_widgets)
- Best when you want to write the user interface directly with Qt Widgets from Rust.
- Crates: qt_core, qt_gui, qt_widgets (and more, e.g., qt_3d, etc.).
- Status: Community-maintained. Requires a matching Qt installation and MSVC toolchain on Windows.

2) Use cxx-qt and keep Widgets in C++
- You build the Qt Widgets UI in C++ (e.g., QMainWindow, QWidget, dialogs) and expose business logic written in Rust via cxx-qt bridges.
- This is great if you’re already comfortable with C++/Qt Widgets and you want Rust for core logic, models, and algorithms.

3) Use qmetaobject-rs (primarily for QML/Qt Quick)
- qmetaobject-rs shines with QML/Qt Quick. While you can integrate a QQuickWidget inside a Widgets-based app, it’s not the typical path if you want a classic Widgets-only UI.

Minimal example using qt_widgets

Below is a simple “Hello, Widgets” app using the ritual Qt crates. It shows a push button. This is a sketch that reflects the crate’s typical API layout.

Cargo.toml

[package]
name = "hello_qt_widgets"
version = "0.1.0"
edition = "2021"

[dependencies]
qt_core = "0.5"
qt_gui = "0.5"
qt_widgets = "0.5"

src/main.rs

use qt_core::qs;
use qt_widgets::application::Application;
use qt_widgets::push_button::PushButton;

fn main() {
    // Application::init takes a closure; return value is used as exit code
    Application::init(|_| {
        let mut button = PushButton::new();
        button.set_text(qs("Hello from Rust + Qt Widgets"));
        button.show();
        Application::exec() // enter the Qt event loop
    });
}

Notes and setup (Windows + Qt)

- Install Qt (same compiler as your Rust toolchain; on Windows, that’s typically MSVC). Using the Qt Maintenance Tool or an online installer is fine.
- Ensure the Qt bin directory is on PATH at runtime (so Qt DLLs are found). Example: C:\Qt\6.x.x\msvcYYYY_64\bin.
- The ritual crates look for Qt via environment variables and common install locations. If detection fails, set QTDIR to your Qt root or use documented variables (see links below).
- Use a recent stable Rust toolchain (rustup toolchain list). For MSVC: rustup default stable-x86_64-pc-windows-msvc.
- If you see linker errors, verify that the Qt version matches the expected ABI (e.g., MSVC vs MinGW) and that you’re building for the same architecture (x64 vs x86).

Alternative: Widgets in C++ with cxx-qt + Rust logic

- Keep the UI in C++, e.g., QMainWindow, dialogs, and custom widgets.
- Expose Rust types and methods via cxx-qt to call into Rust for data models, formatting, heavy computation, etc.
- This repository already uses cxx-qt for QML; the same bridging can be applied when your C++ UI is Widgets instead of QML.

Useful links

- qt_widgets crate on crates.io: https://crates.io/crates/qt_widgets
- Ritual project (Qt Rust bindings): https://github.com/rust-qt/ritual
- cxx-qt (KDAB): https://github.com/KDAB/cxx-qt
- qmetaobject-rs: https://github.com/woboq/qmetaobject-rs
- Qt Downloads: https://www.qt.io/download

FAQ

- Can I mix Widgets and QML?
  Yes. You can embed QML via QQuickWidget in a Widgets app or embed native widgets into a QML/Qt Quick scene with controls. Consider complexity and styling differences.

- Which approach should I choose?
  - Widgets-only UI and no C++: Use the ritual qt_widgets bindings.
  - Existing C++ Widgets codebase or you prefer designing UI with Qt Designer/C++ and keep Rust for logic: Use cxx-qt.
  - Full QML/Qt Quick app with Rust: qmetaobject-rs or cxx-qt are strong options.


Using cxx-qt to compile custom C++ files (Yes)

You can compile your own C++ sources alongside cxx-qt bridges. This repo already does it. Here’s how you can add more files.

Where to put your C++ files
- Place them under crates/chameleon/cpp (or any subfolder you prefer). Example: cpp/my_algorithm.cpp and cpp/my_algorithm.h.

Approach A: Use cxx_build next to CxxQtBuilder (what this repo does)
- Add your Rust bridge modules to cxx_build::bridges([...]) if you expose functions via #[cxx::bridge] in Rust files.
- Add your extra C++ sources with .files(["cpp/format.cpp", "cpp/locale.cpp", "cpp/my_algorithm.cpp"]).
- Optionally add include paths, flags, and defines.
- Keep the existing CxxQtBuilder for QML/Qt linking.

Minimal example (excerpt from crates/chameleon/build.rs):

// Extra C/C++ compilation using cxx_build
let mut binding = cxx_build::bridges(["src/format.rs", "src/locale.rs"]);
let mut cxx_bridge = binding.files(["cpp/format.cpp", "cpp/locale.cpp", "cpp/my_algorithm.cpp"]);

// Add include paths gathered from vcpkg/Qt/etc as needed
cxx_bridge = cxx_bridge
    .flag("/utf-8")
    .flag("/std:c++17");

cxx_bridge.compile("cxx_extra");

println!("cargo:rerun-if-changed=cpp/format.cpp");
println!("cargo:rerun-if-changed=cpp/locale.cpp");
println!("cargo:rerun-if-changed=cpp/my_algorithm.cpp");
println!("cargo:rerun-if-changed=cpp/my_algorithm.h");
println!("cargo:rerun-if-changed=src/format.rs");
println!("cargo:rerun-if-changed=src/locale.rs");

// Keep using CxxQtBuilder to set up Qt modules and QML modules
CxxQtBuilder::new()
    .qt_module("Network")
    .qml_module(QmlModule { /* ... as in this repo ... */ })
    .build();

Approach B: Use CxxQtBuilder’s cc builder directly
- If you prefer to configure everything via CxxQtBuilder, you can access its underlying cc::Build to add files, include paths, or flags.
- Pattern (check your cxx-qt-build version for exact API):

let mut builder = CxxQtBuilder::new();
// Configure Qt/QML modules on builder (qt_module, qml_module, etc.)
// Add extra C++ sources via the cc builder
builder.cc_builder()
    .file("cpp/my_algorithm.cpp")
    .include("cpp")
    .flag("/std:c++17");

builder.build();

Notes and tips
- Headers/includes: Add include("path") for any directories containing your headers.
- Windows/MSVC flags: /std:c++17 and /utf-8 are commonly useful; avoid -std=c++17 on MSVC.
- Rerun triggers: Use cargo:rerun-if-changed for every source/header so Cargo rebuilds when you edit them.
- Linking extra libraries: If your custom code needs additional non-Qt libs, emit cargo:rustc-link-lib=name and cargo:rustc-link-search=native=... from build.rs accordingly. When using vcpkg on Windows, prefer using the vcpkg crate to discover include and link settings.
- Calling C++ from Rust: Expose functions/types in a #[cxx::bridge] mod or through cxx-qt’s bridges. Remember to keep types FFI-safe or wrap them with unique_ptr/shared_ptr where appropriate.

References
- cxx-qt-build docs: https://docs.rs/cxx-qt-build/
- cxx crate build docs (cxx_build): https://docs.rs/cxx-build/
- KDAB cxx-qt repo (examples): https://github.com/KDAB/cxx-qt
