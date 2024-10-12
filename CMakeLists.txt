cmake_minimum_required(VERSION 3.29)
project(Sokoban C)

set(CMAKE_C_STANDARD 11)

include_directories(.)

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
elseif(WIN32)
    # TODO windows
endif()
