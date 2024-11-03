#ifndef LEVEL_H
#define LEVEL_H

#include <stdio.h>
#include <sys/time.h>

void freeLevelData(void);

void initLevelData(int argc, char *argv[]);

void updateKeySelectLevelPack(int key);
void updateKeySelectLevel(int key);
void updateKeyInGame(int key);

void updateMouseSelectLevelPack(int column, int row);
void updateMouseSelectLevel(int column, int row);

void drawSelectLevelPack(void);
void drawSelectLevel(void);

void readLevelData(void);

void updateLevelPackStats(int levelPackIndex);

//Maps & Levels
#define MAX_LEVEL_PACK_COUNT 64
#define MAX_LEVEL_COUNT_PER_PACK 192

extern int currentMapIndex;

extern int continueFlag;

//Draw player background
extern int isPlayerBackground;
extern const int playerBackgroundDelay;
extern int playerBackgroundTmp;

//Level
extern int hasTimeStartInMenu;
extern struct timeval timeStartInMenu;
extern int hasTimeStart;
extern struct timeval timeStart;
extern size_t timeMilliSec;
extern size_t timeSec;
extern size_t timeMin;
extern int minLevelNotCompleted;
extern int selectedLevel;
extern int levelCount;

#endif
