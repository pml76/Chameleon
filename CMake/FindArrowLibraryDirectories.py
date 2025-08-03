import pyarrow as pa
import sys
import os

lib_dirs = pa.get_library_dirs()
include_dirs = pa.get_include()
libs = pa.get_libraries()




def convert(lst):
    lst2 = []
    for i in lst:
        lst2.append(i.replace("\\", "/"))

    return "".join([char for char in str(lst2) if char not in "[],\'"])

with open(sys.argv[1] + "\\arrow.cmake", "w") as file:
    file.write("set(ARROW_INCLUDE_DIRS " + convert([include_dirs]) + ")\n")
    file.write("set(ARROW_LIBRARY_DIRS " + convert(lib_dirs) + ")\n")
    file.write("set(ARROW_LIBRARIES " + convert(libs) + ")\n")
    file.write("\n")
    file.write("add_library(arrow::arrow_shared SHARED IMPORTED)\n")
    file.write("set_target_properties(arrow::arrow_shared PROPERTIES\n")
    file.write("                      INTERFACE_COMPILE_FEATURES \"cxx_std_17\"\n")
    file.write("                      IMPORTED_LINK_INTERFACE_LANGUAGES \"C;CXX\"")
    file.write("                      INTERFACE_INCLUDE_DIRECTORIES \"${ARROW_INCLUDE_DIRS}\"\n")
    file.write("                      INTERFACE_LINK_DIRECTORIES \"${ARROW_LIBRARY_DIRS}\"\n")
    for p in lib_dirs:
        if os.path.exists( p + "\\arrow.lib" ):
            file.write("                      IMPORTED_IMPLIB \"" + convert([p]) + "/arrow.lib\"\n")
            break

    for p in lib_dirs:
        if os.path.exists( p + "\\arrow.dll" ):
            file.write("                      IMPORTED_LOCATION \"" + convert([p]) + "/arrow.dll\"\n")
#    file.write("                      INTERFACE_LINK_LIBRARIES \"${ARROW_LIBRARIES} ws2_32\"\n")
    file.write("                      )\n")
    file.write("\n")
    file.write("add_library(arrow::pyarrow_shared SHARED IMPORTED)\n")
    file.write("set_target_properties(arrow::pyarrow_shared PROPERTIES\n")
    file.write("                      INTERFACE_COMPILE_FEATURES \"cxx_std_17\"\n")
    file.write("                      IMPORTED_LINK_INTERFACE_LANGUAGES \"C;CXX\"\n")
    file.write("                      INTERFACE_INCLUDE_DIRECTORIES \"${ARROW_INCLUDE_DIRS}\"\n")
    file.write("                      INTERFACE_LINK_DIRECTORIES \"${ARROW_LIBRARY_DIRS}\"\n")
    for p in lib_dirs:
        if os.path.exists( p + "\\arrow_python.lib" ):
            file.write("                      IMPORTED_IMPLIB \"" + convert([p]) + "/arrow_python.lib\"\n")
            break

    for p in lib_dirs:
        if os.path.exists( p + "\\arrow_python.dll" ):
            file.write("                      IMPORTED_LOCATION \"" + convert([p]) + "/arrow_python.dll\"\n")
    #    file.write("                      INTERFACE_LINK_LIBRARIES \"${ARROW_LIBRARIES} ws2_32\"\n")
    file.write("                      )\n")
