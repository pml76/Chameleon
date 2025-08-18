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

    set(Python_BASE ${CMAKE_BINARY_DIR}/Python-${VCPKG_TARGET_TRIPLET})
    message(STATUS "Creating venv in: ${Python_BASE}")
    execute_process(COMMAND ${Python_EXECUTABLE} -m venv ${Python_BASE} WORKING_DIRECTORY ${CMAKE_BINARY_DIR})
    set(Python_EXECUTABLE ${Python_BASE}/Scripts/python${EXECUTABLE_EXTENSION})

    if(NOT ${CMAKE_SYSTEM_NAME} STREQUAL "Windows" )
        message(FATAL_ERROR "Not running on windows. Don't know how to handle python here... aborting!")
    endif ()


    if(${CMAKE_SYSTEM_NAME} STREQUAL "Windows")
        if (NOT EXISTS ${Python_BASE}/Scripts/pip${EXECUTABLE_EXTENSION})
            message(STATUS "Installing pip...")
            FetchContent_Declare(
                    get_pip
                    GIT_REPOSITORY https://github.com/pypa/get-pip
                    GIT_TAG        25.2
            )
            FetchContent_MakeAvailable(get_pip)
            message(STATUS "put get_pip in ${get_pip_SOURCE_DIR}")

            execute_process(COMMAND ${Python_EXECUTABLE} ${get_pip_SOURCE_DIR}/public/get-pip.py --no-warn-script-location
                            WORKING_DIRECTORY ${Python_BASE})
        endif ()
        set(Pip_EXECUTABLE ${Python_BASE}/Scripts/pip${EXECUTABLE_EXTENSION})
        message(STATUS "Found pip in: ${Pip_EXECUTABLE}")

        message(STATUS "Writing sitecustomize.py")
        file(WRITE "${Python_BASE}/Lib/site-packages/sitecustomize.py"
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

        set(VCPKG_EXECUTABLE vcpkg.exe env)
        message(STATUS "Executing pip using working directory: ${Python_BASE}")
        message(STATUS "Pip-Executable: ${Pip_EXECUTABLE}")

        message(STATUS "Installing pandas 3.2.1")
        execute_process(COMMAND ${Pip_EXECUTABLE} install "pandas==2.3.1"
                        WORKING_DIRECTORY ${Python_BASE})

#        message(STATUS "Installing polars 1.32.2")
#        execute_process(COMMAND ${Pip_EXECUTABLE} install "polars==1.32.2"
#                        WORKING_DIRECTORY ${Python_BASE})

        message(STATUS "CMAKE_PREFIX_PATH=${CMAKE_PREFIX_PATH}")

        # set PYARROW_BUNDLE_ARROW_CPP=1
        set( ENV{CMAKE_PREFIX_PATH} "${CMAKE_PREFIX_PATH}" )
        set( ENV{PYARROW_CMAKE_GENERATOR} Ninja)
        message(STATUS "Installing arrow 21.0.0")
        execute_process(COMMAND ${Pip_EXECUTABLE} install -v ${CMAKE_SOURCE_DIR}/extern/apache-arrow/python
                WORKING_DIRECTORY ${Python_BASE})

        # message(STATUS "Generating 'arrow.cmake'")
        # execute_process( COMMAND ${Python_EXECUTABLE} FindArrowLibraryDirectories.py ${CMAKE_BINARY_DIR} WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}/CMake )


    endif ()

endfunction()

get_python()


# include(${CMAKE_BINARY_DIR}/arrow.cmake)
# target_link_libraries(HelloWorld PRIVATE arrow::arrow_shared arrow::arrow_compute_shared arrow::pyarrow_shared)

find_package(Arrow REQUIRED)
target_link_libraries(HelloWorld PRIVATE Arrow::arrow_shared)


