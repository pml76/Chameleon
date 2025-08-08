cmake_minimum_required(VERSION 3.10)

project(HelloWorld)


add_executable(HelloWorld helloworld.cpp)

find_package(Python3 COMPONENTS Development REQUIRED)
target_link_libraries(HelloWorld PRIVATE Python3::Python)

message(STATUS "VCPKG_INSTALLED_DIR=${VCPKG_INSTALLED_DIR}")
message(STATUS "VCPKG_TARGET_TRIPLET=${VCPKG_TARGET_TRIPLET}")
message(STATUS "CMAKE_SYSTEM_NAME=${CMAKE_SYSTEM_NAME}")

include(FetchContent)

function(get_python)
    # on windows executables have the extension '.exe'
    if(${CMAKE_SYSTEM_NAME} STREQUAL "Windows")
        SET(EXECUTABLE_EXTENSION ".exe")
    endif ()

    # find an installed python3 interpreter ...
    if(EXISTS ${VCPKG_INSTALLED_DIR}/${VCPKG_TARGET_TRIPLET}/tools/python3/python${EXECUTABLE_EXTENSION})
        SET(Python_EXECUTABLE "${VCPKG_INSTALLED_DIR}/${VCPKG_TARGET_TRIPLET}/tools/python3/python${EXECUTABLE_EXTENSION}")
    else ()
        message(FATAL_ERROR "No python3 interpreter found!")
    endif ()
    message(STATUS "Found python interpreter: ${Python_EXECUTABLE}")

    get_filename_component(Python_ORIGINAL_DIR "${Python_EXECUTABLE}" DIRECTORY)
    set(Python_WORKING_DIR ${CMAKE_BINARY_DIR}/Python3-${VCPKG_TARGET_TRIPLET})

    file(GLOB Python_FILES LIST_DIRECTORIES false "${Python_ORIGINAL_DIR}/*")
    if(EXISTS "${Python_ORIGINAL_DIR}/DLLs")
        file(COPY "${Python_ORIGINAL_DIR}/DLLs/" DESTINATION "${Python_WORKING_DIR}/DLLs")
    endif()

    file(COPY ${Python_FILES} DESTINATION "${Python_WORKING_DIR}/Scripts")
    file(MAKE_DIRECTORY "${Python_WORKING_DIR}/Lib/site-packages")

    file(WRITE "${CMAKE_BINARY_DIR}/full_python_version.py"
        "
import sys
print( str(sys.version_info.major) + \".\" + str(sys.version_info.minor) + \".\" + str(sys.version_info.micro))"
    )

    file(WRITE "${CMAKE_BINARY_DIR}/major_python_version.py"
            "
import sys
print( str(sys.version_info.major) )"
    )

    file(WRITE "${CMAKE_BINARY_DIR}/minor_python_version.py"
            "
import sys
print( str(sys.version_info.minor) )"
    )

    file(WRITE "${CMAKE_BINARY_DIR}/micro_python_version.py"
            "
import sys
print( str(sys.version_info.micro) )"
    )


    execute_process(COMMAND ${Python_EXECUTABLE} ${CMAKE_BINARY_DIR}/full_python_version.py OUTPUT_VARIABLE Python3_VERSION OUTPUT_STRIP_TRAILING_WHITESPACE)
    message(STATUS " Python3_VERSION=${Python3_VERSION}")

    execute_process(COMMAND ${Python_EXECUTABLE} ${CMAKE_BINARY_DIR}/major_python_version.py OUTPUT_VARIABLE Python3_MAJOR_VERSION OUTPUT_STRIP_TRAILING_WHITESPACE)
    message(STATUS " Python3_VERSION=${Python3_MAJOR_VERSION}")

    execute_process(COMMAND ${Python_EXECUTABLE} ${CMAKE_BINARY_DIR}/minor_python_version.py OUTPUT_VARIABLE Python3_MINOR_VERSION OUTPUT_STRIP_TRAILING_WHITESPACE)
    message(STATUS " Python3_VERSION=${Python3_MINOR_VERSION}")

    execute_process(COMMAND ${Python_EXECUTABLE} ${CMAKE_BINARY_DIR}/micro_python_version.py OUTPUT_VARIABLE Python3_MICRO_VERSION OUTPUT_STRIP_TRAILING_WHITESPACE)
    message(STATUS " Python3_VERSION=${Python3_MICRO_VERSION}")


    file(WRITE "${Python_WORKING_DIR}/pyvenv.cfg"
            "
home = ${Python_ORIGINAL_DIR}
include-system-site-packages = false
version = ${Python3_VERSION}
executable = ${Python_EXECUTABLE}
command = ${Python_EXECUTABLE} -m venv ${Python_WORKING_DIR}
            "
    )

    file(WRITE "${Python_WORKING_DIR}/Lib/site-packages/sitecustomize.py"
            "
import os
import sys
from pathlib import Path

import site
#enable direct lookup of installed site-packages without the need to copy them
site.addsitedir(sys.base_prefix + '/Lib/site-packages')

vcpkg_bin_path = Path(sys.base_prefix + '/../../bin')
if vcpkg_bin_path.is_dir():
  os.add_dll_directory(vcpkg_bin_path)
"
    )

    if(${CMAKE_SYSTEM_NAME} STREQUAL "Windows")
        if (NOT EXISTS ${Python_DIR}/Scripts/pip${EXECUTABLE_EXTENSION})

            FetchContent_Declare(
                    get_pip
                    GIT_REPOSITORY https://github.com/pypa/get-pip
                    GIT_TAG        25.2
            )
            FetchContent_MakeAvailable(get_pip)
            message(STATUS "put get_pip in ${get_pip_SOURCE_DIR}")

            execute_process(COMMAND ${Python_EXECUTABLE} ${get_pip_SOURCE_DIR}/public/get-pip.py --no-warn-script-location
                            WORKING_DIRECTORY ${Python_WORKING_DIR})
        endif ()
        set(Pip_EXECUTABLE ${Python_WORKING_DIR}/Scripts/pip${EXECUTABLE_EXTENSION})

        execute_process(COMMAND ${pip_EXECUTABLE} install "'pandas=2.3.1'"
                        WORKING_DIRECTORY ${Python_WORKING_DIR})

    endif ()

endfunction()

get_python()

if(${Python_Interpreter_FOUND})

    execute_process( COMMAND ${Python_EXECUTABLE} FindArrowLibraryDirectories.py ${CMAKE_BINARY_DIR} WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}/CMake/ OUTPUT_VARIABLE Arrow_Library_Dirs)

    include(${CMAKE_BINARY_DIR}/arrow.cmake)

endif()

# target_link_libraries(HelloWorld PRIVATE arrow::arrow_shared arrow::arrow_compute_shared arrow::pyarrow_shared)



