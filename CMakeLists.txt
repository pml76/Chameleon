cmake_minimum_required(VERSION 3.20)
project(test)

set(CMAKE_CXX_STANDARD 20)

add_library(icu INTERFACE)
find_package(ICU COMPONENTS data uc i18n io REQUIRED)
# link system, cross or host libraries to interface target
target_link_libraries(icu INTERFACE ${ICU_LIBRARIES})
target_include_directories(icu INTERFACE ${ICU_INCLUDE_DIRS})

find_package(Qt6 REQUIRED COMPONENTS Core)


add_library(test STATIC
        rust/crates/chameleon-format-dialog/cpp/includes/enums.h
        rust/crates/chameleon-format-dialog/cpp/includes/icu_includes.h
        rust/crates/chameleon-format-dialog/cpp/locale.h
        rust/crates/chameleon-format-dialog/cpp/locale.cpp
        rust/crates/chameleon-format-dialog/cpp/format.h
        rust/crates/chameleon-format-dialog/cpp/format.cpp
        rust/crates/chameleon-format-dialog/cpp/units.cpp
        rust/crates/chameleon-format-dialog/cpp/units.h
         )
message(${CMAKE_SOURCE_DIR})
target_include_directories( test PUBLIC "${CMAKE_SOURCE_DIR}/target/debug/cxxqtbuild/include" )
set_property(TARGET test PROPERTY MSVC_RUNTIME_LIBRARY "MultiThreadedDLL")
target_link_libraries(test icu)
target_link_libraries(test Qt6::Core)


