#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <time.h>
#include "consoleLib.h"
#include "consoleMenu.h"
#include "gameField.h"
#include "helpMenu.h"

#define VERSION "1.1.0"

void resetGame(void);

void init(void);

void update(void);
void updateKey(int);
void updateMouse(void);

int moveBox(int posX, int posY, int moveX, int moveY);

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

//Set funcPtr to drawField after START_MENU
static void (*draw)(void);
static void (*drawOld)(void);

//Maps
#define MAX_LEVEL_PACK_COUNT 64

#define TUTORIAL_MAP "tutorial.lvl"
#define MAIN_MAP "main.lvl"
#define DEMON_MAP "demon.lvl"

static int mapCount;
static int currentMapIndex = 0;
static FILE *map;
static FILE *mapSave;
static char pathMapData[MAX_LEVEL_PACK_COUNT][512];
static char pathMapSaveData[512 + 4];

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
static time_t timeStartInMenu = -1;
static time_t timeStart = -1;
static time_t timeSec = 0;
static time_t timeMin = 0;
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

static int levelBestTime[99];
static int levelBestMoves[99];

static int levelPackAllLevelsBeaten[MAX_LEVEL_PACK_COUNT] = {0};
static int levelPackBestTimeSum = -1;
static int levelPackBestMovesSum = -1;

static const int gameMinWidth = 74;
static const int gameMinHeight = 23;

static enum {
    START_MENU, SELECT_LEVEL_PACK, SELECT_LEVEL, IN_GAME, GAME_OVER
}screen;

void resetGame(void) {
    if(map != NULL) {
        fclose(map);
        map = NULL;
    }
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

    memcpy(pathMapData[i++], TUTORIAL_MAP, min((int)strlen(TUTORIAL_MAP) + 1, 512));
    memcpy(pathMapData[i++], MAIN_MAP, min((int)strlen(MAIN_MAP) + 1, 512));
    memcpy(pathMapData[i++], DEMON_MAP, min((int)strlen(DEMON_MAP) + 1, 512));

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

    srand((unsigned)time(NULL));

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
    if(hasInput()) { //If has key input: call updateKey
        updateKey(getKey());
    }
    updateMouse();

    //Time
    if(screen == IN_GAME && !continueFlag) {
        if(timeStart < 0) {
            timeSec = 0;
            timeMin = 0;
        }else {
            time_t timeNow = time(NULL);

            timeSec = (time_t)(difftime(timeNow, timeStart));
            if(timeStartInMenu > -1)
                timeSec -= (time_t)(difftime(timeNow, timeStartInMenu));

            timeMin = timeSec/60;
            timeSec %= 60;
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

        if(timeStartInMenu > -1 && timeStart > -1)
            timeStart += (time_t)(difftime(time(NULL), timeStartInMenu));

        timeStartInMenu = -1;

        return;
    }
    //Block other inputs if [y]es/[n]o
    if(escCheck) {
        if(key == 'y') {
            if(screen == START_MENU) {
                //Exit game
                exit(EXIT_SUCCESS);
            }else {
                //Reset game (to start menu)
                initVars();
            }
        }else if(key == 'n') {
            escCheck = 0;

            if(timeStartInMenu > -1 && timeStart > -1)
                timeStart += (time_t)(difftime(time(NULL), timeStartInMenu));

            timeStartInMenu = -1;
        }
    }else {
        //Help
        if(key == CL_KEY_F1) {
            isHelp = !isHelp;
            if(isHelp) {
                drawOld = draw;
                draw = drawHelp;

                timeStartInMenu = time(NULL);
            }else {
                draw = drawOld;

                if(timeStartInMenu >-1 && timeStart > -1)
                    timeStart += (time_t)(difftime(time(NULL), timeStartInMenu));

                timeStartInMenu = -1;
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
                        int bestTime = (int)(timeSec + 60 * timeMin);
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
                    if(tmp == PLAYER || tmp == BOX)
                        tmp = EMPTY;
                    if(tmp == BOX_IN_GOAL)
                        tmp = GOAL;
                    levelNow.field[playerPosX][playerPosY] = tmp;

                    if(timeStart < 0)
                        timeStart = time(NULL);

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
                                    if(moveBox(playerPosX - 1, playerPosY, -1, 0))
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
                                    if(moveBox(playerPosX, playerPosY - 1, 0, -1))
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
                                    if(moveBox(playerPosX + 1, playerPosY, 1, 0))
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
                                    if(moveBox(playerPosX, playerPosY + 1, 0, 1))
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

                timeStartInMenu = time(NULL);
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

int moveBox(int posX, int posY, int moveX, int moveY) {
    enum fieldIDs *tmpIDOld = &levelNow.field[posX][posY];
    enum fieldIDs *tmpIDNew = &levelNow.field[posX + moveX][posY + moveY];
    if(*tmpIDNew == EMPTY || *tmpIDNew == GOAL) {
        if(*tmpIDNew == GOAL) {
            *tmpIDNew = BOX_IN_GOAL;

            continueFlag = 1;
            continueLevelAddFlag = 1;
            int breakFlag = 0;
            for(int i = 0;i < levelNow.width;i++) {
                for(int j = 0;j < levelNow.height;j++) {
                    if(levelNow.field[i][j] == GOAL) {
                        continueFlag = 0;
                        continueLevelAddFlag = 0;

                        break;
                    }

                    //Is player at GOAL -> check level field
                    if(i == posX && j == posY && (levels[level].
                    field[i][j] == GOAL || levels[level].
                    field[i][j] == BOX_IN_GOAL)) {
                        continueFlag = 0;
                        continueLevelAddFlag = 0;

                        breakFlag = 1;
                        break;
                    }
                }

                if(breakFlag)
                    break;
            }
        }else {
            *tmpIDNew = BOX;
        }

        if(*tmpIDOld == BOX)
            *tmpIDOld = EMPTY;
        else
            *tmpIDOld = GOAL;

        return 1;
    }

    return 0;
}

void drawField(void) {
    resetColor();
    drawf("Level: %02d", level + 1);

    setCursorPos((int)((gameMinWidth - 11) * .25), 0);
    drawf("Moves: %04d", moves);

    setCursorPos(gameMinWidth - 11, 0);
    drawf("Time: %02d:%02d", timeMin, timeSec);

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

    //Exit
    if(escCheck) {
        setColor(CL_COLOR_BLACK, CL_COLOR_YELLOW);
        setCursorPos(27, 10);
        drawf("Back to start menu?");
        setCursorPos(27, 11);
        drawf("-------------------");
        setCursorPos(27, 12);
        drawf("                   ");
        setCursorPos(27, 13);
        drawf("[y]es          [n]o");

        //Draw border
        setColor(CL_COLOR_LIGHT_BLACK, CL_COLOR_RED);
        setCursorPos(26, 9);
        drawf("                     ");
        setCursorPos(26, 14);
        drawf("                     ");
        for(int i = 10;i < 14;i++) {
            setCursorPos(26, i);
            drawf(" ");
            setCursorPos(46, i);
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
    drawf(".-------------------------------.");
    for(int i = 1;i < 4;i++) {
        setCursorPos(0, y + i);
        drawf("|                               |");
    }
    setCursorPos(0, y + 4);
    drawf("\'-------------------------------\'");

    //Draw sum of best time and sum of best moves
    resetColor();
    setCursorPos(1, y + 1);
    drawf("Selected level pack:         %02d", currentMapIndex + 1);
    setCursorPos(1, y + 2);
    drawf("Best time sum      : ");
    if(levelPackBestTimeSum < 0) {
        drawf("X:XX:XX:XX");
    }else {
        drawf("%01d:%02d:%02d:%02d", levelPackBestTimeSum/86400, (levelPackBestTimeSum/3600)%24, (levelPackBestTimeSum/60)%60, levelPackBestTimeSum%60);
    }
    setCursorPos(1, y + 3);
    drawf("Best moves sum     :     ");
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
        drawf("%2d", i + 1);

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
    y = 4 + (levelCount/24)*2;

    setCursorPos(0, y);
    setColor(CL_COLOR_CYAN, CL_COLOR_NO_COLOR);
    drawf(".---------------------.");
    for(int i = 1;i < 4;i++) {
        setCursorPos(0, y + i);
        drawf("|                     |");
    }
    setCursorPos(0, y + 4);
    drawf("\'---------------------\'");

    //Draw best time and best moves
    resetColor();
    setCursorPos(1, y + 1);
    drawf("Selected level:    %02d", selectedLevel + 1);
    setCursorPos(1, y + 2);
    drawf("Best time     : ");
    if(levelBestTime[selectedLevel] < 0) {
        drawf("XX:XX");
    }else {
        drawf("%02d:%02d", levelBestTime[selectedLevel]/60, levelBestTime[selectedLevel]%60);
    }
    setCursorPos(1, y + 3);
    drawf("Best moves    :  ");
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
    timeSec = timeMin = 0;
    timeStart = -1;
    timeStartInMenu = -1;

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

    map = fopen(pathMapData[currentMapIndex], "r");
    char buf[4096];
    if(map == NULL) {
        reset();
        printf("Can't read map data file \"%s\"!\n", pathMapData[currentMapIndex]);

        exit(EXIT_FAILURE);
    }

    fscanf(map, "Levels: %d\n\n", &levelCount);
    if(levelCount > 99) {
        reset();
        printf("To many levels (Max: 99) (In file: %d)!\n", levelCount);

        exit(EXIT_FAILURE);
    }

    strcpy(pathMapSaveData, pathMapData[currentMapIndex]);
    strcat(pathMapSaveData, ".sav");

    mapSave = fopen(pathMapSaveData, "r+");
    if(mapSave == NULL) {
        //File does not yet exist
        mapSave = fopen(pathMapSaveData, "w+");
    }

    if(mapSave == NULL) {
        fclose(map);
        map = NULL;

        reset();
        printf("Can't read or create map save file \"%s\"!\n", pathMapSaveData);

        exit(EXIT_FAILURE);
    }
    minLevelNotCompleted = 0;
    fscanf(mapSave, "%d\n", &minLevelNotCompleted);

    if(minLevelNotCompleted > levelCount) //If mLNC == lC -> all levels completed
        minLevelNotCompleted = 0;

    for(int i = 0;i < 99;i++) {
        levelBestTime[i] = -1;
        levelBestMoves[i] = -1;
        fscanf(mapSave, "%d,%d\n", levelBestTime + i, levelBestMoves + i);
    }

    levels = malloc((size_t)levelCount * sizeof(struct field));

    int width, height;
    for(int i = 0;i < levelCount;i++) {
        fscanf(map, "w: %d, h: %d\n", &width, &height);

        //"height >=", 1st line: infos
        if(width > gameMinWidth || height >= gameMinHeight) {
            reset();
            printf("Level is too large (Max: %d x %d) (Level: %d x %d)!\n", gameMinWidth,
            gameMinHeight - 1, width, height);

            exit(EXIT_FAILURE);
        }

        for(int j = 0;j < height;j++)
            fscanf(map, "%s", buf + j*width);
        fscanf(map, "\n");

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

        for(int i = 0;i < 99;i++)
            fprintf(mapSave, "%d,%d\n", levelBestTime[i], levelBestMoves[i]);

        fflush(mapSave);
    }else {
        printf("Can't read or create map save file \"%s\"!\n", pathMapSaveData);
    }
}

void updateLevelPackStats(int levelPackIndex) {
    if(map != NULL) {
        fclose(map);
        map = NULL;
    }
    if(mapSave != NULL) {
        fclose(mapSave);
        mapSave = NULL;
    }

    levelPackAllLevelsBeaten[levelPackIndex] = 0;
    levelPackBestTimeSum = -1;
    levelPackBestMovesSum = -1;

    map = fopen(pathMapData[levelPackIndex], "r");
    if(map == NULL)
        return;

    int levelCountTmp = 100;

    fscanf(map, "Levels: %d\n\n", &levelCountTmp);
    fclose(map);
    map = NULL;
    if(levelCountTmp > 99)
        return;

    strcpy(pathMapSaveData, pathMapData[levelPackIndex]);
    strcat(pathMapSaveData, ".sav");

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
        int bestTimeTmp = -1;
        int bestMovesTmp = -1;

        fscanf(mapSave, "%d,%d\n", &bestTimeTmp, &bestMovesTmp);

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
