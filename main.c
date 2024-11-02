#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <time.h>
#include <sys/time.h>

#include "consoleLib.h"
#include "consoleMenu.h"
#include "gameField.h"
#include "helpMenu.h"

#include "map_tutorial.h"
#include "map_main.h"
#include "map_special.h"
#include "map_demon.h"

#define VERSION "1.3.0"

void resetGame(void);

void init(void);

void update(void);
void updateKey(int);
void updateMouse(void);

int moveBoxOrKey(int posX, int posY, int moveX, int moveY);

void drawField(void);
void drawSelectLevelPack(void);
void drawSelectLevel(void);
void drawStartMenu(void);

void initVars(void);

void setLevel(int lvl);
void readLevelData(void);
void saveLevelData(void);

void updateLevelPackStats(int levelPackIndex);

int min(int, int);
long min_l(long, long);

size_t timediff_milisec(struct timeval, struct timeval);

//Set funcPtr to drawField after START_MENU
static void (*draw)(void);
static void (*drawOld)(void);

//Maps & Levels
#define MAX_LEVEL_PACK_COUNT 64
#define MAX_LEVEL_COUNT_PER_PACK 192

const char build_in_map_prefix[] = "build-in:";

static int mapCount;
static int currentMapIndex = 0;
static FILE *mapSave;
static char pathMapData[MAX_LEVEL_PACK_COUNT][512];
static char pathMapSaveData[2048];

//Help
static int isHelp;

static int escCheck;
static int continueFlag;
static int continueLevelAddFlag;

//Draw player background
static int isPlayerBackground = 0;
static const int playerBackgroundDelay = 12;
static int playerBackgroundTmp = 0;

//Level
static int moves = 0;
static int oldMoves = 0;
static int hasTimeStartInMenu = 0;
static struct timeval timeStartInMenu;
static int hasTimeStart = 0;
static struct timeval timeStart;
static size_t timeMilliSec = 0;
static size_t timeSec = 0;
static size_t timeMin = 0;
static int playerPosX;
static int playerPosY;
static int playerPosXOld;
static int playerPosYOld;
static int level, minLevelNotCompleted = 0;
static int selectedLevel;
static struct field levelNow;
static struct field levelNowLastStep;
static struct field levelNowTmpStep;
static int levelCount = 0;
static struct field *levels;

static long levelBestTime[MAX_LEVEL_COUNT_PER_PACK];
static int levelBestMoves[MAX_LEVEL_COUNT_PER_PACK];

static int levelPackAllLevelsBeaten[MAX_LEVEL_PACK_COUNT] = {0};
static long levelPackBestTimeSum = -1;
static int levelPackBestMovesSum = -1;

static const int gameMinWidth = 74;
static const int gameMinHeight = 23;

static enum {
    START_MENU, SELECT_LEVEL_PACK, SELECT_LEVEL, IN_GAME, GAME_OVER
}screen;

void resetGame(void) {
    if(mapSave != NULL) {
        fclose(mapSave);
        mapSave = NULL;
    }

    reset();
    freeTableOfContents(&content);

    removeField(&levelNow);
    removeField(&levelNowLastStep);
    removeField(&levelNowTmpStep);
    for(int i = 0;i < levelCount;i++)
        removeField(levels + i);
    free(levels);
}

int main(int argc, char *argv[]) {
    //Default level packs
    int i = 0;

    memcpy(pathMapData[i++], tutorial_map_id, min((int)strlen(tutorial_map_id) + 1, 512));
    memcpy(pathMapData[i++], main_map_id, min((int)strlen(main_map_id) + 1, 512));
    memcpy(pathMapData[i++], special_map_id, min((int)strlen(special_map_id) + 1, 512));
    memcpy(pathMapData[i++], demon_map_id, min((int)strlen(demon_map_id) + 1, 512));

    for(int j = 1;j < argc && i < MAX_LEVEL_PACK_COUNT;j++) //Additional level packs
        memcpy(pathMapData[i++], argv[j], min((int)strlen(argv[j]) + 1, 512));

    mapCount = i;

    init();

    while(1) {
        update();
        sleepMS(40);
    }

    return EXIT_SUCCESS;
}

void init(void) {
    //Load level pack stats
    for(int i = 0;i < mapCount;i++)
        updateLevelPackStats(i);

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
                drawOld = drawSelectLevelPack;
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
                switch(key) {
                    case CL_KEY_LEFT:
                        if(currentMapIndex == 0)
                            break;

                        currentMapIndex--;

                        updateLevelPackStats(currentMapIndex);

                        break;
                    case CL_KEY_UP:
                        currentMapIndex -= 24;
                        if(currentMapIndex < 0)
                            currentMapIndex += 24;

                        updateLevelPackStats(currentMapIndex);

                        break;
                    case CL_KEY_RIGHT:
                        if(currentMapIndex == mapCount-1)
                            break;

                        currentMapIndex++;

                        updateLevelPackStats(currentMapIndex);

                        break;
                    case CL_KEY_DOWN:
                        currentMapIndex += 24;
                        if(currentMapIndex >= mapCount)
                            currentMapIndex -= 24;

                        updateLevelPackStats(currentMapIndex);

                        break;
                }

                if(key == CL_KEY_ENTER) {
                    screen = SELECT_LEVEL;

                    readLevelData();

                    //Set new draw function
                    draw = drawSelectLevel;
                }

                break;
            case SELECT_LEVEL:
                switch(key) {
                    case CL_KEY_LEFT:
                        if(selectedLevel == 0)
                            break;

                        selectedLevel--;

                        break;
                    case CL_KEY_UP:
                        selectedLevel -= 24;
                        if(selectedLevel < 0)
                            selectedLevel += 24;

                        break;
                    case CL_KEY_RIGHT:
                        if(selectedLevel == levelCount-1)
                            break;

                        selectedLevel++;

                        break;
                    case CL_KEY_DOWN:
                        selectedLevel += 24;
                        if(selectedLevel >= levelCount)
                            selectedLevel -= 24;

                        break;
                }

                if(key == CL_KEY_ENTER && selectedLevel <= minLevelNotCompleted) {
                    setLevel(selectedLevel);
                    selectedLevel = 0;

                    screen = IN_GAME;

                    //Set new draw function
                    draw = drawField;
                }

                break;
            case IN_GAME:
                //Level end
                if(continueFlag) {
                    if(continueLevelAddFlag) {
                        if(level >= minLevelNotCompleted)
                            minLevelNotCompleted = level+1;

                        //Update best scores
                        long bestTime = (long)(timeMilliSec + 1000 * timeSec + 60000 * timeMin);
                        if(levelBestTime[level] == -1 || levelBestTime[level] > bestTime)
                            levelBestTime[level] = bestTime;

                        if(levelBestMoves[level] == -1 || levelBestMoves[level] > moves)
                            levelBestMoves[level] = moves;

                        //Save level
                        saveLevelData();

                        continueLevelAddFlag = 0;
                    }

                    if(key == CL_KEY_ENTER) {
                        continueFlag = 0;
                        level++;

                        //All levels completed
                        if(level == levelCount) {
                            level--;
                            screen = GAME_OVER;

                            break;
                        }

                        setLevel(level);
                    }else if(key == 'r') {
                        continueFlag = 0;

                        setLevel(level);
                    }

                    break;
                }

                //One step back
                if(key == 'z') {
                    for(int i = 0;i < levelNow.width;i++) {
                        for(int j = 0;j < levelNow.height;j++) {
                            enum fieldIDs tmp = levelNow.field[i][j];
                            levelNow.field[i][j] = levelNowLastStep.field[i][j];
                            levelNowLastStep.field[i][j] = tmp;
                        }
                    }

                    //Reset move count
                    int tmp = moves;
                    moves = oldMoves;
                    oldMoves = tmp;

                    //Reset player pos
                    tmp = playerPosX;
                    playerPosX = playerPosXOld;
                    playerPosXOld = tmp;
                    tmp = playerPosY;
                    playerPosY = playerPosYOld;
                    playerPosYOld = tmp;
                }

                //Reset
                if(key == 'r')
                    setLevel(level);

                if(isArrowKey(key)) {
                    //For check change
                    int playerPosXTmp = playerPosX;
                    int playerPosYTmp = playerPosY;
                    for(int i = 0;i < levelNow.width;i++)
                        for(int j = 0;j < levelNow.height;j++)
                            levelNowTmpStep.field[i][j] = levelNow.field[i][j];

                    //Set players old position to old level data
                    enum fieldIDs tmp = levels[level].field[playerPosX][playerPosY];
                    if(tmp == PLAYER || tmp == BOX || tmp == KEY || tmp == LOCKED_DOOR)
                        tmp = EMPTY;
                    else if(tmp == BOX_IN_GOAL || tmp == KEY_IN_GOAL)
                        tmp = GOAL;
                    levelNow.field[playerPosX][playerPosY] = tmp;

                    if(!hasTimeStart) {
                        gettimeofday(&timeStart, NULL);
                        hasTimeStart = 1;
                    }

                    switch(key) {
                        case CL_KEY_LEFT:
                            switch(levelNow.field[playerPosX - 1][playerPosY]) {
                                case EMPTY:
                                case GOAL:
                                case ONE_WAY_LEFT:
                                    playerPosX--;
                                    break;
                                case BOX:
                                case BOX_IN_GOAL:
                                case KEY:
                                case KEY_IN_GOAL:
                                    if(moveBoxOrKey(playerPosX - 1, playerPosY, -1, 0))
                                        playerPosX--;
                                    break;
                                default:
                                    break;
                            }

                            break;
                        case CL_KEY_UP:
                            switch(levelNow.field[playerPosX][playerPosY - 1]) {
                                case EMPTY:
                                case GOAL:
                                case ONE_WAY_UP:
                                    playerPosY--;
                                    break;
                                case BOX:
                                case BOX_IN_GOAL:
                                case KEY:
                                case KEY_IN_GOAL:
                                    if(moveBoxOrKey(playerPosX, playerPosY - 1, 0, -1))
                                        playerPosY--;
                                    break;
                                default:
                                    break;
                            }

                            break;
                        case CL_KEY_RIGHT:
                            switch(levelNow.field[playerPosX + 1][playerPosY]) {
                                case EMPTY:
                                case GOAL:
                                case ONE_WAY_RIGHT:
                                    playerPosX++;
                                    break;
                                case BOX:
                                case BOX_IN_GOAL:
                                case KEY:
                                case KEY_IN_GOAL:
                                    if(moveBoxOrKey(playerPosX + 1, playerPosY, 1, 0))
                                        playerPosX++;
                                    break;
                                default:
                                    break;
                            }

                            break;
                        case CL_KEY_DOWN:
                            switch(levelNow.field[playerPosX][playerPosY + 1]) {
                                case EMPTY:
                                case GOAL:
                                case ONE_WAY_DOWN:
                                    playerPosY++;
                                    break;
                                case BOX:
                                case BOX_IN_GOAL:
                                case KEY:
                                case KEY_IN_GOAL:
                                    if(moveBoxOrKey(playerPosX, playerPosY + 1, 0, 1))
                                        playerPosY++;
                                    break;
                                default:
                                    break;
                            }

                            break;
                    }

                    //Set player to new position
                    levelNow.field[playerPosX][playerPosY] = PLAYER;

                    //Copy level to last step if change
                    if(playerPosX != playerPosXTmp || playerPosY != playerPosYTmp) {
                        oldMoves = moves++; //Set oldMoves later moves++

                        playerPosXOld = playerPosXTmp;
                        playerPosYOld = playerPosYTmp;
                        for(int i = 0;i < levelNow.width;i++)
                            for(int j = 0;j < levelNow.height;j++)
                                levelNowLastStep.field[i][j] =
                                levelNowTmpStep.field[i][j];
                    }
                }

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
                if(row == 0)
                    break;

                int mapIndex = column/3 + (row - 1)/2*24;
                if(mapIndex < mapCount) {
                    currentMapIndex = mapIndex;
                    updateKey(CL_KEY_ENTER);
                }

                break;
            case SELECT_LEVEL:
                if(row == 0)
                    break;

                int level = column/3 + (row - 1)/2*24;
                if(level <= minLevelNotCompleted) {
                    updateKey(CL_KEY_ENTER);
                    setLevel(level);
                }

                break;
            case IN_GAME:
            case GAME_OVER:
                break;
        }
    }
}

int moveBoxOrKey(int posX, int posY, int moveX, int moveY) {
    enum fieldIDs *tmpIDOld = &levelNow.field[posX][posY];
    enum fieldIDs *tmpIDNew = &levelNow.field[posX + moveX][posY + moveY];

    int isBox = *tmpIDOld == BOX || *tmpIDOld == BOX_IN_GOAL;

    if(*tmpIDNew == EMPTY || *tmpIDNew == GOAL || (!isBox && *tmpIDNew == LOCKED_DOOR)) {
        if(isBox && *tmpIDNew == GOAL) {
            *tmpIDNew = BOX_IN_GOAL;

            continueFlag = 1;
            continueLevelAddFlag = 1;
            int breakFlag = 0;
            for(int i = 0;i < levelNow.width;i++) {
                for(int j = 0;j < levelNow.height;j++) {
                    if(levelNow.field[i][j] == GOAL || levelNow.field[i][j] == KEY_IN_GOAL) {
                        continueFlag = 0;
                        continueLevelAddFlag = 0;

                        breakFlag = 1;
                        break;
                    }

                    //Is player at GOAL -> check level field
                    if(i == posX && j == posY && levels[level].field[i][j] == GOAL) {
                        continueFlag = 0;
                        continueLevelAddFlag = 0;

                        breakFlag = 1;
                        break;
                    }
                }

                if(breakFlag)
                    break;
            }
        }else if(!isBox && *tmpIDNew == GOAL) {
            *tmpIDNew = KEY_IN_GOAL;
        }else if(isBox) {
            *tmpIDNew = BOX;
        }else if(*tmpIDNew == LOCKED_DOOR) {
            //Open door and destroy key
            *tmpIDNew = EMPTY;
        }else {
            *tmpIDNew = KEY;
        }

        if(*tmpIDOld == BOX || *tmpIDOld == KEY)
            *tmpIDOld = EMPTY;
        else
            *tmpIDOld = GOAL;

        return 1;
    }

    return 0;
}

void drawField(void) {
    resetColor();
    drawf("Pack: %02d", currentMapIndex + 1);

    setCursorPos((int)((gameMinWidth - 9) * .25), 0);
    drawf("Level: ");
    if(level + 1 < 100) {
        drawf("%02d", level + 1);
    }else {
        drawf("%c", 'A' + (level + 1 - 100) / 10);
        drawf("%d", (level + 1) % 10);
    }

    setCursorPos((int)((gameMinWidth - 11) * .75), 0);
    drawf("Moves: %04d", moves);

    setCursorPos(gameMinWidth - 15, 0);
    drawf("Time: %02d:%02d.%03d", timeMin, timeSec, timeMilliSec);

    if(continueFlag) {
        setCursorPos((int)((gameMinWidth - 16) * .5), 0);
        drawf("Level completed!");
    }
    if(screen == GAME_OVER) {
        setCursorPos((int)((gameMinWidth - 13) * .5), 0);
        drawf("You have won!");
    }

    int startX = (int)((gameMinWidth - levelNow.width) * .5);
    for(int i = 0;i < levelNow.height;i++) {
        setCursorPos(startX, i+1);
        for(int j = 0;j < levelNow.width;j++) {
            drawf("%c", getCharFromField(&levelNow, j, i, isPlayerBackground));
        }
        drawf("\n");
    }

    //Draw special help text for tutorial levels (tutorial pack and tutorial levels in special pack)
    if(currentMapIndex == 0) { //Tutorial pack
        resetColor();
        switch(level) {
            case 0:
                if(continueFlag) {
                    setCursorPos(18, 8);
                    drawf("Press ");

                    setColor(CL_COLOR_RED, CL_COLOR_NO_COLOR);
                    drawf("ENTER");

                    resetColor();
                    drawf(" to go to the next level...");
                }else {
                    setCursorPos(17, 8);
                    drawf("Use the arrow keys (< ^ > v) to move...");
                }

                break;
            case 1:
                setCursorPos(16, 8);
                drawf("Boxes (");

                setColor(CL_COLOR_LIGHT_CYAN, CL_COLOR_NO_COLOR);
                drawf("@");

                resetColor();
                drawf(") must be placed on ");

                setColor(CL_COLOR_RED, CL_COLOR_NO_COLOR);
                drawf("all");

                resetColor();
                drawf(" goals (");

                setColor(CL_COLOR_RED, CL_COLOR_NO_COLOR);
                drawf("x");

                resetColor();
                drawf(")");

                break;

            case 2:
                setCursorPos(14, 8);
                drawf("Some boxes (");

                setColor(CL_COLOR_LIGHT_PINK, CL_COLOR_NO_COLOR);
                drawf("@");

                resetColor();
                drawf(") might already be in a goal (");

                setColor(CL_COLOR_RED, CL_COLOR_NO_COLOR);
                drawf("x");

                resetColor();
                drawf(")");

            break;

            case 3:
                setCursorPos(14, 8);
                drawf("Not all boxes (");

                setColor(CL_COLOR_LIGHT_CYAN, CL_COLOR_NO_COLOR);
                drawf("@");

                resetColor();
                drawf(") must be in a goal (");

                setColor(CL_COLOR_RED, CL_COLOR_NO_COLOR);
                drawf("x");

                resetColor();
                drawf(") to win");

                break;

            case 4:
                setCursorPos(5, 8);
                drawf("One-way doors (");

                setColor(CL_COLOR_BLUE, CL_COLOR_NO_COLOR);
                drawf("< ^ > v");

                resetColor();
                drawf(") can only be entered from the opened side");

                break;

            case 5:
                if(screen == GAME_OVER) {
                    setCursorPos(12, 8);
                    drawf("Press ");

                    setColor(CL_COLOR_RED, CL_COLOR_NO_COLOR);
                    drawf("ESC");

                    resetColor();
                    drawf(" to go back to the level selection screen");
                }else {
                    setCursorPos(8, 8);
                    drawf("Boxes (");

                    setColor(CL_COLOR_LIGHT_CYAN, CL_COLOR_NO_COLOR);
                    drawf("@");

                    resetColor();
                    drawf(") can not be moved through one-way doors (");

                    setColor(CL_COLOR_BLUE, CL_COLOR_NO_COLOR);
                    drawf("< ^ > v");

                    resetColor();
                    drawf(")");
                }

                break;
        }
    }else if(currentMapIndex == 2) { //Built-in special pack
        resetColor();
        switch(level) {
            case 0:
                setCursorPos(18, 8);
                drawf("Keys (");

                setColor(CL_COLOR_LIGHT_CYAN, CL_COLOR_NO_COLOR);
                drawf("*");

                resetColor();
                drawf(") can be used to open doors (");

                setColor(CL_COLOR_RED, CL_COLOR_NO_COLOR);
                drawf("=");

                resetColor();
                drawf(")");

                break;
            case 1:
                setCursorPos(19, 8);
                drawf("Every key (");

                setColor(CL_COLOR_LIGHT_CYAN, CL_COLOR_NO_COLOR);
                drawf("*");

                resetColor();
                drawf(") can open any door (");

                setColor(CL_COLOR_RED, CL_COLOR_NO_COLOR);
                drawf("=");

                resetColor();
                drawf(")");

                break;

            case 2:
                setCursorPos(21, 8);
                drawf("Keys (");

                setColor(CL_COLOR_LIGHT_PINK, CL_COLOR_NO_COLOR);
                drawf("*");

                resetColor();
                drawf(") might be in a goal (");

                setColor(CL_COLOR_RED, CL_COLOR_NO_COLOR);
                drawf("x");

                resetColor();
                drawf(")");

                break;
        }
    }

    //Exit
    if(escCheck) {
        setColor(CL_COLOR_BLACK, CL_COLOR_YELLOW);
        setCursorPos(25, 10);
        drawf("Back to level selection?");
        setCursorPos(25, 11);
        drawf("------------------------");
        setCursorPos(25, 12);
        drawf("                        ");
        setCursorPos(25, 13);
        drawf("[y]es               [n]o");

        //Draw border
        setColor(CL_COLOR_LIGHT_BLACK, CL_COLOR_RED);
        setCursorPos(24, 9);
        drawf("                          ");
        setCursorPos(24, 14);
        drawf("                          ");
        for(int i = 10;i < 14;i++) {
            setCursorPos(24, i);
            drawf(" ");
            setCursorPos(49, i);
            drawf(" ");
        }
    }
}
void drawSelectLevelPack(void) {
    resetColor();
    setUnderline(1);
    drawf("Select a level pack:");
    setUnderline(0);

    //Draw first line
    setCursorPos(0, 1);
    drawf("-");
    int max = mapCount%24;
    if(mapCount/24 > 0)
        max = 24;
    for(int i = 0;i < max;i++) {
        int x = 1 + (i%24)*3;

        setCursorPos(x, 1);
        drawf("---");
    }

    for(int i = 0;i < mapCount;i++) {
        int x = 1 + (i%24)*3;
        int y = 2 + (i/24)*2;

        //First box
        if(x == 1) {
            setCursorPos(x - 1, y);
            drawf("|");

            setCursorPos(x - 1, y + 1);
            drawf("-");
        }

        setColor(CL_COLOR_BLACK, levelPackAllLevelsBeaten[i]?CL_COLOR_GREEN:CL_COLOR_YELLOW);
        setCursorPos(x, y);
        drawf("%2d", i + 1);

        resetColor();
        drawf("|");

        setCursorPos(x, y + 1);
        drawf("---");
    }

    //Mark selected level
    int x = (currentMapIndex%24)*3;
    int y = 1 + (currentMapIndex/24)*2;

    setColor(CL_COLOR_CYAN, CL_COLOR_NO_COLOR);
    setCursorPos(x, y);
    drawf("----");
    setCursorPos(x, y + 1);
    drawf("|");
    setCursorPos(x + 3, y + 1);
    drawf("|");
    setCursorPos(x, y + 2);
    drawf("----");

    //Draw border for best time and best moves
    y = 4 + (mapCount/24)*2;

    setCursorPos(0, y);
    setColor(CL_COLOR_CYAN, CL_COLOR_NO_COLOR);
    drawf(".-----------------------------------.");
    for(int i = 1;i < 4;i++) {
        setCursorPos(0, y + i);
        drawf("|                                   |");
    }
    setCursorPos(0, y + 4);
    drawf("\'-----------------------------------\'");

    //Draw sum of best time and sum of best moves
    resetColor();
    setCursorPos(1, y + 1);
    drawf("Selected level pack:             %02d", currentMapIndex + 1);
    setCursorPos(1, y + 2);
    drawf("Sum of best time   : ");
    if(levelPackBestTimeSum < 0) {
        drawf("X:XX:XX:XX.XXX");
    }else {
        drawf("%01d:%02d:%02d:%02d.%03d", levelPackBestTimeSum/86400000, (levelPackBestTimeSum/3600000)%24,
            (levelPackBestTimeSum/60000)%60, (levelPackBestTimeSum/1000)%60, levelPackBestTimeSum%1000);
    }
    setCursorPos(1, y + 3);
    drawf("Sum of best moves  :         ");
    if(levelPackBestMovesSum < 0) {
        drawf("XXXXXX");
    }else {
        drawf("%06d", levelPackBestMovesSum);
    }
}
void drawSelectLevel(void) {
    resetColor();
    setUnderline(1);
    drawf("Select a level (Level pack \"%s\"):", pathMapData[currentMapIndex]);
    setUnderline(0);

    //Draw first line
    setCursorPos(0, 1);
    drawf("-");
    int max = levelCount%24;
    if(levelCount/24 > 0)
        max = 24;
    for(int i = 0;i < max;i++) {
        int x = 1 + (i%24)*3;

        setCursorPos(x, 1);
        drawf("---");
    }

    for(int i = 0;i < levelCount;i++) {
        int x = 1 + (i%24)*3;
        int y = 2 + (i/24)*2;

        //First box
        if(x == 1) {
            setCursorPos(x - 1, y);
            drawf("|");

            setCursorPos(x - 1, y + 1);
            drawf("-");
        }

        setColor(CL_COLOR_BLACK, (i < minLevelNotCompleted)?CL_COLOR_GREEN:(
        (i == minLevelNotCompleted)?CL_COLOR_YELLOW:CL_COLOR_RED));
        setCursorPos(x, y);

        if(i + 1 < 100) {
            drawf("%2d", i + 1);
        }else {
            drawf("%c", 'A' + (i + 1 - 100) / 10);
            drawf("%d", (i + 1) % 10);
        }

        resetColor();
        drawf("|");

        setCursorPos(x, y + 1);
        drawf("---");
    }

    //Mark selected level
    int x = (selectedLevel%24)*3;
    int y = 1 + (selectedLevel/24)*2;

    setColor(CL_COLOR_CYAN, CL_COLOR_NO_COLOR);
    setCursorPos(x, y);
    drawf("----");
    setCursorPos(x, y + 1);
    drawf("|");
    setCursorPos(x + 3, y + 1);
    drawf("|");
    setCursorPos(x, y + 2);
    drawf("----");

    //Draw border for best time and best moves
    y = 4 + ((levelCount - 1)/24)*2;

    setCursorPos(0, y);
    setColor(CL_COLOR_CYAN, CL_COLOR_NO_COLOR);
    drawf(".-------------------------.");
    for(int i = 1;i < 4;i++) {
        setCursorPos(0, y + i);
        drawf("|                         |");
    }
    setCursorPos(0, y + 4);
    drawf("\'-------------------------\'");

    //Draw best time and best moves
    resetColor();
    setCursorPos(1, y + 1);
    drawf("Selected level:        ");
    if(selectedLevel + 1 < 100) {
        drawf("%02d", selectedLevel + 1);
    }else {
        drawf("%c", 'A' + (selectedLevel + 1 - 100) / 10);
        drawf("%d", (selectedLevel + 1) % 10);
    }
    setCursorPos(1, y + 2);
    drawf("Best time     : ");
    if(levelBestTime[selectedLevel] < 0) {
        drawf("XX:XX.XXX");
    }else {
        drawf("%02d:%02d.%03d", levelBestTime[selectedLevel]/60000, (levelBestTime[selectedLevel]%60000)/1000, levelBestTime[selectedLevel]%1000);
    }
    setCursorPos(1, y + 3);
    drawf("Best moves    :      ");
    if(levelBestMoves[selectedLevel] < 0) {
        drawf("XXXX");
    }else {
        drawf("%04d", levelBestMoves[selectedLevel]);
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

void initVars(void) {
    //Help
    isHelp = 0;

    //Function pointer
    drawOld = draw = drawStartMenu;
    screen = START_MENU;

    escCheck = 0;
    continueFlag = 0;
}

void setLevel(int lvl) {
    moves = 0;
    timeMilliSec = timeSec = timeMin = 0;
    hasTimeStart = 0;
    hasTimeStartInMenu = 0;

    level = lvl;
    removeField(&levelNow);
    removeField(&levelNowLastStep);
    removeField(&levelNowTmpStep);

    //Copy level
    initField(&levelNow, levels[lvl].width, levels[lvl].height);
    initField(&levelNowLastStep, levels[lvl].width, levels[lvl].height);
    initField(&levelNowTmpStep, levels[lvl].width, levels[lvl].height);
    for(int i = 0;i < levels[lvl].width;i++) {
        for(int j = 0;j < levels[lvl].height;j++) {
            levelNow.field[i][j] = levels[lvl].field[i][j];
            levelNowLastStep.field[i][j] = levels[lvl].field[i][j];
        }
    }

    int breakFlag = 0;
    for(int i = 0;i < levelNow.width;i++) {
        for(int j = 0;j < levelNow.height;j++) {
            if(levelNow.field[i][j] == PLAYER) {
                playerPosXOld = playerPosX = i;
                playerPosYOld = playerPosY = j;

                breakFlag = 1;
                break;
            }
        }

        if(breakFlag)
            break;
    }
}

void readLevelData(void) {
    if(levels != NULL) {
        for(int i = 0;i < levelCount;i++)
            removeField(levels + i);
        free(levels);

        selectedLevel = 0;
        levelCount = 0;
    }

    getConfigPathPrefix(pathMapSaveData);

    char mapData[65536];
    int mapDataByteOffset = 0;
    int bytesRead = 0;
    if(strlen(build_in_map_prefix) <= strlen(pathMapData[currentMapIndex]) &&
        memcmp(build_in_map_prefix, pathMapData[currentMapIndex], strlen(build_in_map_prefix)) == 0) {
        //build-in map

        if(strlen(tutorial_map_id) <= strlen(pathMapData[currentMapIndex]) &&
            memcmp(tutorial_map_id, pathMapData[currentMapIndex], strlen(tutorial_map_id)) == 0) {
            memcpy(mapData, tutorial_map_data, strlen(tutorial_map_data) + 1);

            strcat(pathMapSaveData, "tutorial.lvl.sav");
        }else if(strlen(main_map_id) <= strlen(pathMapData[currentMapIndex]) &&
            memcmp(main_map_id, pathMapData[currentMapIndex], strlen(main_map_id)) == 0) {
            memcpy(mapData, main_map_data, strlen(main_map_data) + 1);

            strcat(pathMapSaveData, "main.lvl.sav");
        }else if(strlen(special_map_id) <= strlen(pathMapData[currentMapIndex]) &&
            memcmp(special_map_id, pathMapData[currentMapIndex], strlen(special_map_id)) == 0) {
            memcpy(mapData, special_map_data, strlen(special_map_data) + 1);

            strcat(pathMapSaveData, "special.lvl.sav");
        }else if(strlen(demon_map_id) <= strlen(pathMapData[currentMapIndex]) &&
            memcmp(demon_map_id, pathMapData[currentMapIndex], strlen(demon_map_id)) == 0) {
            memcpy(mapData, demon_map_data, strlen(demon_map_data) + 1);

            strcat(pathMapSaveData, "demon.lvl.sav");
        }else {
            reset();
            printf("Can't read build-in map data file \"%s\"!\n", pathMapData[currentMapIndex]);

            exit(EXIT_FAILURE);
        }
    }else {
        FILE *map = fopen(pathMapData[currentMapIndex], "r");
        if(map == NULL) {
            reset();
            printf("Can't read map data file \"%s\"!\n", pathMapData[currentMapIndex]);

            exit(EXIT_FAILURE);
        }

        strcat(pathMapSaveData, pathMapData[currentMapIndex]);
        strcat(pathMapSaveData, ".sav");

        fseek(map, 0, SEEK_END);
        long fileSize = ftell(map);
        fseek(map, 0, SEEK_SET);

        fread(mapData, min_l(fileSize, 65536), 1, map);

        fclose(map);
        map = NULL;
    }

    sscanf(mapData + mapDataByteOffset, "Levels: %d\n\n%n", &levelCount, &bytesRead);
    mapDataByteOffset += bytesRead;
    if(levelCount > MAX_LEVEL_COUNT_PER_PACK) {
        reset();
        printf("To many levels (Max: %d) (In file: %d)!\n", MAX_LEVEL_COUNT_PER_PACK, levelCount);

        exit(EXIT_FAILURE);
    }

    mapSave = fopen(pathMapSaveData, "r+");
    if(mapSave == NULL) {
        //File does not yet exist
        mapSave = fopen(pathMapSaveData, "w+");
    }

    if(mapSave == NULL) {
        reset();
        printf("Can't read or create map save file \"%s\"!\n", pathMapSaveData);

        exit(EXIT_FAILURE);
    }
    minLevelNotCompleted = 0;
    fscanf(mapSave, "%d\n", &minLevelNotCompleted);

    if(minLevelNotCompleted > levelCount) //If mLNC == lC -> all levels completed
        minLevelNotCompleted = 0;

    for(int i = 0;i < MAX_LEVEL_COUNT_PER_PACK;i++) {
        levelBestTime[i] = -1;
        levelBestMoves[i] = -1;

        if(i < minLevelNotCompleted) {
            int hasNewTimeSaveFormat = fscanf(mapSave, "ms%ld,%d\n", levelBestTime + i, levelBestMoves + i);
            if(!hasNewTimeSaveFormat) {
            fscanf(mapSave, "%ld,%d\n", levelBestTime + i, levelBestMoves + i);

                //Old format was saved in seconds (convert to ms)
                levelBestTime[i] *= 1000;
                levelBestTime[i] += 999;
            }
        }
    }

    levels = malloc((size_t)levelCount * sizeof(struct field));

    int width, height;
    for(int i = 0;i < levelCount;i++) {
        char buf[4096];
        sscanf(mapData + mapDataByteOffset, "w: %d, h: %d\n%n", &width, &height, &bytesRead);
        mapDataByteOffset += bytesRead;

        //"height >=", 1st line: infos
        if(width > gameMinWidth || height >= gameMinHeight) {
            reset();
            printf("Level is too large (Max: %d x %d) (Level: %d x %d)!\n", gameMinWidth,
            gameMinHeight - 1, width, height);

            exit(EXIT_FAILURE);
        }

        for(int j = 0;j < height;j++) {
            sscanf(mapData + mapDataByteOffset, "%s%n", buf + j*width, &bytesRead);
            mapDataByteOffset += bytesRead;
        }
        sscanf(mapData + mapDataByteOffset, "\n%n", &bytesRead);
        mapDataByteOffset += bytesRead;

        initField(levels + i, width, height);
        for(int j = 0;j < width;j++) {
            for(int k = 0;k < height;k++) {
                switch(buf[k*width + j]) {
                    case '<':
                        levels[i].field[j][k] = ONE_WAY_LEFT;
                        break;
                    case '^':
                        levels[i].field[j][k] = ONE_WAY_UP;
                        break;
                    case '>':
                        levels[i].field[j][k] = ONE_WAY_RIGHT;
                        break;
                    case 'v':
                        levels[i].field[j][k] = ONE_WAY_DOWN;
                        break;
                    case '#':
                        levels[i].field[j][k] = WALL;
                        break;
                    case 'P':
                        levels[i].field[j][k] = PLAYER;
                        break;
                    case '*':
                        levels[i].field[j][k] = KEY;
                        break;
                    case '~':
                        levels[i].field[j][k] = KEY_IN_GOAL;
                        break;
                    case '=':
                        levels[i].field[j][k] = LOCKED_DOOR;
                        break;
                    case '@':
                        levels[i].field[j][k] = BOX;
                        break;
                    case '+':
                        levels[i].field[j][k] = BOX_IN_GOAL;
                        break;
                    case 'x':
                        levels[i].field[j][k] = GOAL;
                        break;
                }
            }
        }
    }
}
void saveLevelData(void) {
    if(mapSave != NULL) {
        //Go to start of file
        rewind(mapSave);

        fprintf(mapSave, "%d\n", minLevelNotCompleted);

        for(int i = 0;i < minLevelNotCompleted;i++)
            fprintf(mapSave, "ms%ld,%d\n", levelBestTime[i], levelBestMoves[i]);

        fflush(mapSave);
    }else {
        printf("Can't read or create map save file \"%s\"!\n", pathMapSaveData);
    }
}

void updateLevelPackStats(int levelPackIndex) {
    if(mapSave != NULL) {
        fclose(mapSave);
        mapSave = NULL;
    }

    levelPackAllLevelsBeaten[levelPackIndex] = 0;
    levelPackBestTimeSum = -1;
    levelPackBestMovesSum = -1;

    int levelCountTmp = MAX_LEVEL_COUNT_PER_PACK + 1;

    getConfigPathPrefix(pathMapSaveData);

    if(strlen(build_in_map_prefix) <= strlen(pathMapData[levelPackIndex]) &&
        memcmp(build_in_map_prefix, pathMapData[levelPackIndex], strlen(build_in_map_prefix)) == 0) {
        //build-in map

        if(strlen(tutorial_map_id) <= strlen(pathMapData[levelPackIndex]) &&
            memcmp(tutorial_map_id, pathMapData[levelPackIndex], strlen(tutorial_map_id)) == 0) {
            sscanf(tutorial_map_data, "Levels: %d\n\n", &levelCountTmp);

            strcat(pathMapSaveData, "tutorial.lvl.sav");
        }else if(strlen(main_map_id) <= strlen(pathMapData[levelPackIndex]) &&
            memcmp(main_map_id, pathMapData[levelPackIndex], strlen(main_map_id)) == 0) {
            sscanf(main_map_data, "Levels: %d\n\n", &levelCountTmp);

            strcat(pathMapSaveData, "main.lvl.sav");
        }else if(strlen(special_map_id) <= strlen(pathMapData[levelPackIndex]) &&
            memcmp(special_map_id, pathMapData[levelPackIndex], strlen(special_map_id)) == 0) {
            sscanf(special_map_data, "Levels: %d\n\n", &levelCountTmp);

            strcat(pathMapSaveData, "special.lvl.sav");
        }else if(strlen(demon_map_id) <= strlen(pathMapData[levelPackIndex]) &&
            memcmp(demon_map_id, pathMapData[levelPackIndex], strlen(demon_map_id)) == 0) {
            sscanf(demon_map_data, "Levels: %d\n\n", &levelCountTmp);

            strcat(pathMapSaveData, "demon.lvl.sav");
        }else {
            reset();
            printf("Can't read build-in map data file \"%s\"!\n", pathMapData[levelPackIndex]);

            exit(EXIT_FAILURE);
        }
    }else {
        FILE *map = fopen(pathMapData[levelPackIndex], "r");
        if(map == NULL) {
            reset();
            printf("Can't read map data file \"%s\"!\n", pathMapData[levelPackIndex]);

            exit(EXIT_FAILURE);
        }

        strcat(pathMapSaveData, pathMapData[levelPackIndex]);
        strcat(pathMapSaveData, ".sav");

        fscanf(map, "Levels: %d\n\n", &levelCountTmp);

        fclose(map);
        map = NULL;
    }

    if(levelCountTmp > MAX_LEVEL_COUNT_PER_PACK)
        return;

    mapSave = fopen(pathMapSaveData, "r+");
    if(mapSave == NULL)
        return;

    int minLevelNotCompletedTmp = 0;
    fscanf(mapSave, "%d\n", &minLevelNotCompletedTmp);

    if(minLevelNotCompletedTmp > levelCountTmp)
        minLevelNotCompletedTmp = 0;

    levelPackAllLevelsBeaten[levelPackIndex] = minLevelNotCompletedTmp == levelCountTmp;

    levelPackBestTimeSum = 0;
    levelPackBestMovesSum = 0;

    for(int i = 0;i < levelCountTmp;i++) {
        long bestTimeTmp = -1;
        int bestMovesTmp = -1;

        if(i < minLevelNotCompletedTmp) {
            int hasNewTimeSaveFormat = fscanf(mapSave, "ms%ld,%d\n", &bestTimeTmp, &bestMovesTmp);
            if(!hasNewTimeSaveFormat) {
                fscanf(mapSave, "%ld,%d\n", &bestTimeTmp, &bestMovesTmp);

                //Old format was saved in seconds (convert to ms)
                bestTimeTmp *= 1000;
                bestTimeTmp += 999;
            }
        }

        if(levelPackBestTimeSum >= 0)
            levelPackBestTimeSum = bestTimeTmp < 0?-1:(levelPackBestTimeSum + bestTimeTmp);

        if(levelPackBestMovesSum >= 0)
            levelPackBestMovesSum = bestMovesTmp < 0?-1:(levelPackBestMovesSum + bestMovesTmp);
    }
    fclose(mapSave);
    mapSave = NULL;
}

int min(int a, int b) {
    return a < b?a:b;
}

long min_l(long a, long b) {
    return a < b?a:b;
}

inline size_t timediff_milisec(struct timeval start, struct timeval end) {
    return ((start.tv_sec - end.tv_sec) * 1000000 + start.tv_usec - end.tv_usec) / 1000;
}
