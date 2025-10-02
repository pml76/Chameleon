cmake_minimum_required(VERSION 4.0)
project(test)

set(CMAKE_CXX_STANDARD 20)

add_library(icu INTERFACE)
find_package(ICU COMPONENTS data uc i18n io REQUIRED)
# link system, cross or host libraries to interface target
target_link_libraries(icu INTERFACE ${ICU_LIBRARIES})
target_include_directories(icu INTERFACE ${ICU_INCLUDE_DIRS})



add_library(test STATIC enums.h format.h icu_includes.h locale.h format.cpp locale.cpp)
# target_include_directories()
target_link_libraries(test icu)


