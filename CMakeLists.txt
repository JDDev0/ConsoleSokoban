cmake_minimum_required(VERSION 3.0)
project(Sokoban C)

set(CMAKE_C_STANDARD 11)

include_directories(.)

function(include_map map_name)
    file(READ "${map_name}.lvl" hex_content HEX)

    string(REPEAT "[0-9a-f]" 32 column_pattern)
    string(REGEX REPLACE "(${column_pattern})" "\\1\n" content "${hex_content}")

    string(REGEX REPLACE "([0-9a-f][0-9a-f])" "0x\\1, " content "${content}")

    string(REGEX REPLACE ", $" "" content "${content}")

    STRING(TOUPPER "MAP_${map_name}_H" include_guard)

    set(array_definition "#ifndef ${include_guard}\n#define ${include_guard}\nstatic const char ${map_name}_map_id[] = \"build-in:${map_name}\";\nstatic const char ${map_name}_map_data[] = {\n${content}\n};\n#endif\n")

    set(source "//Auto generated file (DO NOT EDIT)\n${array_definition}\n")

    file(WRITE "map_${map_name}.h" "${source}")
endfunction()

include_map(tutorial)
include_map(main)
include_map(demon)

add_executable(Sokoban
    consoleLib.c
    consoleLib.h
    consoleMenu.c
    consoleMenu.h
    gameField.c
    gameField.h
    helpMenu.c
    helpMenu.h
    main.c)

if(UNIX)
    find_package(Curses REQUIRED)
    include_directories(${CURSES_INCLUDE_DIR})

    target_link_libraries(Sokoban ${CURSES_LIBRARIES})

    target_link_libraries(Sokoban m)
endif()
