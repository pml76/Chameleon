cmake_minimum_required(VERSION 3.10)

project(HelloWorld)


add_executable(HelloWorld helloworld.cpp)


# add_subdirectory(./3rd-party/apache-arrow-21.0.0/cpp)
#find_package(Arrow CONFIG REQUIRED)
#target_link_libraries(HelloWorld PRIVATE Arrow::arrow_shared)


find_package(Python COMPONENTS Interpreter Development)

if(${Python_Interpreter_FOUND})

    execute_process( COMMAND ${Python_EXECUTABLE} FindArrowLibraryDirectories.py ${CMAKE_BINARY_DIR} WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}/CMake/ OUTPUT_VARIABLE Arrow_Library_Dirs)

    include(${CMAKE_BINARY_DIR}/arrow.cmake)

endif()

target_link_libraries(HelloWorld PRIVATE Python::Python arrow::arrow_shared arrow::pyarrow_shared)



