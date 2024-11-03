#ifndef GAME_H
#define GAME_H

int startGame(int argc, char *argv[]);

void updateKey(int);

#define VERSION "1.3.0-dev"

extern int escCheck;

extern const int gameMinWidth;
extern const int gameMinHeight;

extern void (*draw)(void);

enum {
    START_MENU, SELECT_LEVEL_PACK, SELECT_LEVEL, IN_GAME, GAME_OVER
}screen;

#endif
