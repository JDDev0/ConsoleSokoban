#include <stdlib.h>
#include "gameField.h"
#include "consoleLib.h"

void initField(struct field *field, int width, int height) {
    field->width = width;
    field->height = height;
    field->field = malloc((size_t)width*sizeof(enum fieldIDs *));
    for(int i = 0;i < (int)width;i++)
        field->field[i] = malloc((size_t)height*sizeof(enum fieldIDs));

    for(int i = 0;i < width;i++)
        for(int j = 0;j < height;j++)
            field->field[i][j] = EMPTY;
}
void removeField(struct field *field) {
    for(int i = 0;i < field->width;i++)
        free(field->field[i]);
    free(field->field);
}

char getCharFromField(struct field *field, int x, int y, int isPlayerBackground) {
    switch(field->field[x][y]) {
        case EMPTY:
            setColor(CL_COLOR_LIGHT_BLUE, CL_COLOR_NO_COLOR);
            return '-';
        case ONE_WAY_LEFT:
            setColor(CL_COLOR_LIGHT_BLUE, CL_COLOR_NO_COLOR);
            return '<';
        case ONE_WAY_UP:
            setColor(CL_COLOR_LIGHT_BLUE, CL_COLOR_NO_COLOR);
            return '^';
        case ONE_WAY_RIGHT:
            setColor(CL_COLOR_LIGHT_BLUE, CL_COLOR_NO_COLOR);
            return '>';
        case ONE_WAY_DOWN:
            setColor(CL_COLOR_LIGHT_BLUE, CL_COLOR_NO_COLOR);
            return 'v';
        case WALL:
            setColor(CL_COLOR_LIGHT_GREEN, CL_COLOR_NO_COLOR);
            return '#';
        case PLAYER:
            if(isPlayerBackground)
                setColor(CL_COLOR_NO_COLOR, CL_COLOR_YELLOW);
            else
                setColor(CL_COLOR_YELLOW, CL_COLOR_NO_COLOR);
            return 'P';
        case BOX:
            setColor(CL_COLOR_LIGHT_CYAN, CL_COLOR_NO_COLOR);
            return '@';
        case GOAL:
            setColor(CL_COLOR_LIGHT_RED, CL_COLOR_NO_COLOR);
            return 'x';
        case BOX_IN_GOAL:
            setColor(CL_COLOR_PINK, CL_COLOR_NO_COLOR);
            return '@';
    }

    return ' ';
}

