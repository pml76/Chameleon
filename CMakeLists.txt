cmake_minimum_required(VERSION 3.24)

project(chameleon_bin)

if (CMAKE_CXX_COMPILER_ID STREQUAL "MSVC")
    set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreadedDLL")
endif()

if(BUILD_WASM)
    # Ensure Rust build for the correct target
    set(Rust_CARGO_TARGET wasm32-unknown-emscripten)
    set(THREADS_PREFER_PTHREAD_FLAG ON)
    find_package(Threads REQUIRED)
endif()

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CXXQT_QTCOMPONENTS Core Gui Qml QuickControls2 QuickTest Test)
if(NOT BUILD_WASM)
    set(CXXQT_QTCOMPONENTS ${CXXQT_QTCOMPONENTS} QmlImportScanner)
endif()

find_package(Qt6 COMPONENTS ${CXXQT_QTCOMPONENTS})
set(Qt "Qt6")

if(MSVC)
    # Qt also needs to link against the non-debug version of the MSVC Runtime libraries, see the previous comment.
    # Note: The Qt:: targets are ALIAS targets that do not support setting properties directly.
    # We therefore need to resolve the target names to either Qt5 or Qt6 directly.
    set_property(
            TARGET ${Qt}::Core ${Qt}::Gui ${Qt}::Qml ${Qt}::QuickControls2 ${Qt}::QuickTest ${Qt}::Test ${Qt}::Network
            PROPERTY MAP_IMPORTED_CONFIG_DEBUG "RELEASE")
endif()

find_package(CxxQt QUIET)
if(NOT CxxQt_FOUND)
    include(FetchContent)
    FetchContent_Declare(
            CxxQt
            GIT_REPOSITORY https://github.com/kdab/cxx-qt-cmake.git
            GIT_TAG main
    )

    FetchContent_MakeAvailable(CxxQt)
endif()


cxx_qt_import_crate(MANIFEST_PATH rust/crates/chameleon/Cargo.toml
        CRATES chameleon
        CRATE_TYPES staticlib
        #LOCKED
        NO_LINKER_OVERRIDE
        QT_MODULES Qt::Core Qt::Gui Qt::Qml Qt::QuickControls2 Qt::Network
)

cxx_qt_import_qml_module(chameleon_main
        URI "chameleon.main"
        SOURCE_CRATE chameleon)

cxx_qt_import_qml_module(chameleon_dialog
        URI "chameleon.dialogs.format"
        SOURCE_CRATE chameleon)

# Define the executable with the C++ source
if(BUILD_WASM)
    # Currently need to use qt_add_executable
    # for WASM builds, otherwise there is no
    # HTML output.
    #
    # TODO: Figure out how to configure such that
    #       we can use add_executable for WASM
    qt_add_executable(chameleon_bin cpp/main.cpp)
else()
    add_executable(chameleon_bin cpp/main.cpp)
endif()


set(Python_ROOT_DIR "c:/GitHub/Chameleon/.venv")
find_package(Python REQUIRED COMPONENTS Development.Module Interpreter)

find_package(ICU REQUIRED COMPONENTS uc data tu io i18n)

# Link to the qml module, which in turn links to the Rust qml_multi_crates library
target_link_libraries(chameleon_bin PRIVATE
        Python::Module ICU::i18n ICU::uc ICU::data ICU::tu ICU::io
        Qt::Core Qt::Gui Qt::Qml chameleon_main chameleon_dialog)

# If we are using a statically linked Qt then we need to import any qml plugins
qt_import_qml_plugins(chameleon_bin)
