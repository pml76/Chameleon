cmake_minimum_required(VERSION 3.10)

project(HelloWorld)


add_executable(HelloWorld helloworld.cpp)

find_package(Python3 COMPONENTS Development REQUIRED)
target_link_libraries(HelloWorld PRIVATE Python3::Python)

include(CMake/GetInstalledPythonExecutable.cmake)

get_installed_python_executable(Python_EXECUTABLE)

message(INFO "${Python_EXECUTABLE}")

if(${Python_Interpreter_FOUND})

    execute_process( COMMAND ${Python_EXECUTABLE} FindArrowLibraryDirectories.py ${CMAKE_BINARY_DIR} WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}/CMake/ OUTPUT_VARIABLE Arrow_Library_Dirs)

    include(${CMAKE_BINARY_DIR}/arrow.cmake)

endif()

# target_link_libraries(HelloWorld PRIVATE arrow::arrow_shared arrow::arrow_compute_shared arrow::pyarrow_shared)



