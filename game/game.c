#include "game.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <sys/time.h>

#include "consoleLib.h"
#include "consoleMenu.h"
#include "helpMenu.h"
#include "level/level.h"

int escCheck;

const int gameMinWidth = 74;
const int gameMinHeight = 23;

void (*draw)(void);
void (*drawOld)(void);

//Help
static int isHelp;

void resetGame(void);

void init(void);
void initVars(void);

void update(void);
void updateMouse(void);

void drawStartMenu(void);

size_t timediff_milisec(struct timeval, struct timeval);

int startGame(int argc, char *argv[]) {
    init();

    initLevelData(argc, argv);

    while(1) {
        update();
        sleepMS(40);
    }

    return EXIT_SUCCESS;
}

void resetGame(void) {
    reset();
    freeTableOfContents(&content);

    freeLevelData();
}

void init(void) {
    initConsole();

    int widthCon, heightCon;
    getConsoleSize(&widthCon, &heightCon);
    if(widthCon < gameMinWidth || heightCon < gameMinHeight) {
        reset();
        printf("Console is to small (Min: %d x %d)!\n", gameMinWidth, gameMinHeight);

        exit(EXIT_FAILURE);
    }

    //Add reset after console size check
    atexit(resetGame);

    //Help menu
    initHelpMenu(gameMinWidth, gameMinHeight - 4);

    initVars();
}

void initVars(void) {
    //Help
    isHelp = 0;

    //Function pointer
    drawOld = draw = drawStartMenu;
    screen = START_MENU;

    escCheck = 0;
    continueFlag = 0;
}


void update(void) {
    //Input
    if(hasInput()) {
        updateKey(getKey());
    }
    updateMouse();

    //Time
    if(screen == IN_GAME && !continueFlag && !hasTimeStartInMenu) {
        if(hasTimeStart) {
            struct timeval timeNow;
            gettimeofday(&timeNow, NULL);

            timeMilliSec = timediff_milisec(timeNow, timeStart);

            timeSec = timeMilliSec/1000;
            timeMilliSec %= 1000;

            timeMin = timeSec/60;
            timeSec %= 60;
        }else {
            timeMilliSec = 0;
            timeSec = 0;
            timeMin = 0;
        }
    }

    //Graphics
    clrscr();
    //Player background
    if(++playerBackgroundTmp >= playerBackgroundDelay + isPlayerBackground) {
        //If isPlayerBackground: wait an additional update (25 updates per second, every half
        //second: switch background/foreground colors [12 updates, 13 updates])
        playerBackgroundTmp = 0;
        isPlayerBackground = !isPlayerBackground;
    }
    draw();
}

void updateKey(int key) {
    //Help
    if(isHelp && key == CL_KEY_ESC) {
        isHelp = 0;
        draw = drawOld;

        if(hasTimeStartInMenu && hasTimeStart) {
            struct timeval timeNow;
            gettimeofday(&timeNow, NULL);

            timeStart.tv_sec += timeNow.tv_sec - timeStartInMenu.tv_sec;
            timeStart.tv_usec += timeNow.tv_usec - timeStartInMenu.tv_usec;
        }

        hasTimeStartInMenu = 0;

        return;
    }
    //Block other inputs if [y]es/[n]o (Exit game immediately if GAME_OVER and ESC was pressed)
    if(escCheck || (screen == GAME_OVER && key == CL_KEY_ESC)) {
        if(key == 'y' || (screen == GAME_OVER && key == CL_KEY_ESC)) {
            if(screen == START_MENU) {
                //Exit game
                exit(EXIT_SUCCESS);
            }else {
                //Go to level selection
                screen = SELECT_LEVEL;

                escCheck = 0;
                continueFlag = 0;

                updateLevelPackStats(currentMapIndex);

                readLevelData();

                //Set new draw function
                draw = drawSelectLevel;
            }
        }else if(key == 'n') {
            escCheck = 0;

            if(hasTimeStartInMenu && hasTimeStart) {
                struct timeval timeNow;
                gettimeofday(&timeNow, NULL);

                timeStart.tv_sec += timeNow.tv_sec - timeStartInMenu.tv_sec;
                timeStart.tv_usec += timeNow.tv_usec - timeStartInMenu.tv_usec;
            }

            hasTimeStartInMenu = 0;
        }
    }else {
        //Help
        if(key == CL_KEY_F1) {
            isHelp = !isHelp;
            if(isHelp) {
                drawOld = draw;
                draw = drawHelp;

                gettimeofday(&timeStartInMenu, NULL);
                hasTimeStartInMenu = 1;
            }else {
                draw = drawOld;

                if(hasTimeStartInMenu && hasTimeStart) {
                    struct timeval timeNow;
                    gettimeofday(&timeNow, NULL);

                    timeStart.tv_sec += timeNow.tv_sec - timeStartInMenu.tv_sec;
                    timeStart.tv_usec += timeNow.tv_usec - timeStartInMenu.tv_usec;
                }

                hasTimeStartInMenu = 0;
            }

            return;
        }
        if(isHelp) {
            if(key == CL_KEY_UP) {
                helpPage--;
                if(helpPage < 0) {
                    helpPage = maxHelpPage;
                }
            }else if(key == CL_KEY_DOWN) {
                helpPage++;
                if(helpPage > maxHelpPage) {
                    helpPage = 0;
                }
            }

            return;
        }

        switch(screen) {
            case START_MENU:
                if(key == CL_KEY_ENTER) {
                    initVars();

                    //Set selected level
                    if(minLevelNotCompleted == levelCount)
                        selectedLevel = 0;
                    else
                        selectedLevel = minLevelNotCompleted;

                    screen = SELECT_LEVEL_PACK;

                    //Set new draw function
                    draw = drawSelectLevelPack;

                    updateLevelPackStats(currentMapIndex);
                }

                break;
            case SELECT_LEVEL_PACK:
                updateKeySelectLevelPack(key);

                break;
            case SELECT_LEVEL:
                updateKeySelectLevel(key);

                break;
            case IN_GAME:
                updateKeyInGame(key);

                break;
            case GAME_OVER:
                break;
        }

        //Exit game
        if(key == CL_KEY_ESC) {
            if(screen == SELECT_LEVEL_PACK) {
                screen = START_MENU;

                //Set new draw function
                draw = drawStartMenu;
            }else if(screen == SELECT_LEVEL) {
                screen = SELECT_LEVEL_PACK;

                //Set new draw function
                draw = drawSelectLevelPack;

                updateLevelPackStats(currentMapIndex);
            }else {
                escCheck = 1;

                gettimeofday(&timeStartInMenu, NULL);
                hasTimeStartInMenu = 1;
            }
        }
    }
}
void updateMouse(void) {
    int column, row;
    getMousePosClicked(&column, &row);

    if(column == -1) //No input
        return;

    //Help
    if(isHelp) {
        //"row - 2": Content start at row 2
        int tmpPage = getPageMouseClicked(&content, helpPage, row - 2);
        if(tmpPage != -1)
            helpPage = tmpPage;

        if(row == 22 && column < 8)
            updateKey(CL_KEY_DOWN);

        return;
    }

    if(escCheck) {
        if(row == 13) {
            if(screen == START_MENU) {
                if(column > 28 && column < 34)
                    updateKey('y');
                else if(column > 40 && column < 45)
                    updateKey('n');
            }else {
                if(column > 26 && column < 32)
                    updateKey('y');
                else if(column > 41 && column < 46)
                    updateKey('n');
            }
        }
    }else {
        switch(screen) {
            case START_MENU:
                if(row == 16 && column > 26 && column < 32)
                    updateKey(CL_KEY_ENTER);
                if(row == 21 && column > 64 && column < 73)
                    updateKey(CL_KEY_F1);

                break;
            case SELECT_LEVEL_PACK:
                updateMouseSelectLevelPack(column, row);

                break;
            case SELECT_LEVEL:
                updateMouseSelectLevel(column, row);

                break;
            case IN_GAME:
            case GAME_OVER:
                break;
        }
    }
}

void drawStartMenu(void) {
    //Draw border (top)
    setColor(CL_COLOR_WHITE, CL_COLOR_BLUE);
    drawf("/--------------------------------------------------------------------"
          "----\\\n");

    //Draw text
    setColor(CL_COLOR_LIGHT_YELLOW, CL_COLOR_NO_COLOR);
    drawf("                -----------------------------------------\n          "
          "      .---- .---. |  ./ .---. .--.  .---. .   .\n                |   "
          "  |   | | /'  |   | |   : |   | |\\  |\n                '---. |   | :"
          "{    |   | +---+ +---+ | \\ |\n                    | |   | | \\.  |  "
          " | |   : |   | |  \\|\n                ----' '---' |  '\\ '---' '--' "
          " |   | '   '\n                ---------------------------------------"
          "--\n\n\n\n\n\n-------------------------------------------------------"
          "------------------");

    //Draw infos
    resetColor();
    char verStr[70]; //69 chars + '\0'
    sprintf(verStr, "Version: " VERSION);
    setCursorPos(71 - (int)strlen(verStr), 14); //69 chars, (3 chars empty)
    drawf("%s", verStr);
    setCursorPos(21, 16);
    drawf("Press ");
    setColor(CL_COLOR_LIGHT_RED, CL_COLOR_NO_COLOR);
    drawf("ENTER");
    resetColor();
    drawf(" to start the game!");
    setCursorPos(1, 21);
    drawf("By ");
    setColor(CL_COLOR_NO_COLOR, CL_COLOR_YELLOW);
    drawf("JDDev0");

    resetColor();
    setCursorPos(65, 21);
    drawf("Help: ");
    setColor(CL_COLOR_LIGHT_RED, CL_COLOR_NO_COLOR);
    drawf("F1");

    //Draw border
    setColor(CL_COLOR_WHITE, CL_COLOR_BLUE);
    for(int i = 1;i < 22;i++) {
        setCursorPos(0, i);
        drawf("|");
        setCursorPos(73, i);
        drawf("|");
    }
    drawf("\n\\-----------------------------------------------------------------"
          "-------/");

    //Exit
    if(escCheck) {
        setColor(CL_COLOR_BLACK, CL_COLOR_YELLOW);
        setCursorPos(29, 10);
        drawf("   Exit game?   ");
        setCursorPos(29, 11);
        drawf("   ----------   ");
        setCursorPos(29, 12);
        drawf("                ");
        setCursorPos(29, 13);
        drawf("[y]es       [n]o");

        //Draw border
        setColor(CL_COLOR_LIGHT_BLACK, CL_COLOR_RED);
        setCursorPos(28, 9);
        drawf("                  ");
        setCursorPos(28, 14);
        drawf("                  ");
        for(int i = 10;i < 14;i++) {
            setCursorPos(28, i);
            drawf(" ");
            setCursorPos(45, i);
            drawf(" ");
        }
    }
}

inline size_t timediff_milisec(struct timeval start, struct timeval end) {
    return ((start.tv_sec - end.tv_sec) * 1000000 + start.tv_usec - end.tv_usec) / 1000;
}
