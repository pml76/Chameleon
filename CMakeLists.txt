cmake_minimum_required(VERSION 3.10)

project(HelloWorld)


add_executable(HelloWorld helloworld.cpp)


# add_subdirectory(./3rd-party/apache-arrow-21.0.0/cpp)
find_package(Arrow CONFIG REQUIRED)
target_link_libraries(HelloWorld PRIVATE Arrow::arrow_shared)


find_package(Python COMPONENTS Interpreter Development)
target_link_libraries(HelloWorld PRIVATE Python::Python)



