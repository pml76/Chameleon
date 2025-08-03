set(ARROW_INCLUDE_DIRS C:/Users/U439644/Projects/Chameleon/.venv/Lib/site-packages/pyarrow/include)
set(ARROW_LIBRARY_DIRS C:/Users/U439644/Projects/Chameleon/.venv/Lib/site-packages/pyarrow C:/Users/U439644/Projects/Chameleon/.venv/Lib/site-packages/pyarrow.libs)
set(ARROW_LIBRARIES arrow_python arrow)

add_library(arrow SHARED IMPORTED)
set_target_properties(arrow PROPERTIES
                      INTERFACE_COMPILE_FEATURES "cxx_std_17"
                      INTERFACE_INCLUDE_DIRECTORIES "${ARROW_INCLUDE_DIRS}"
                      INTERFACE_LINK_DIRECTORIES "${ARROW_LIBRARY_DIRS}"
                      IMPORTED_IMPLIB "C:/Users/U439644/Projects/Chameleon/.venv/Lib/site-packages/pyarrow/arrow.lib"
                      IMPORTED_LOCATION "C:/Users/U439644/Projects/Chameleon/.venv/Lib/site-packages/pyarrow/arrow.dll"
                      INTERFACE_LINK_LIBRARIES "${ARROW_LIBRARIES} ws2_32"
                      )
