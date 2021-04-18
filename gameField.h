#ifndef GAME_FIELD_H
#define GAME_FIELD_H

    enum fieldIDs {
        EMPTY, ONE_WAY_LEFT, ONE_WAY_UP, ONE_WAY_RIGHT, ONE_WAY_DOWN, WALL, PLAYER,
        BOX, GOAL, BOX_IN_GOAL
    };

    struct field {
        int width;
        int height;
        enum fieldIDs **field;
    };

    void initField(struct field *field, int width, int height);
    void removeField(struct field *field);

    char getCharFromField(struct field *field, int x, int y, int isPlayerBackground);
#endif
