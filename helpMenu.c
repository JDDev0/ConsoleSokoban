#include "helpMenu.h"
#include "consoleLib.h"
#include "consoleMenu.h"
#include "gameField.h"

struct tableOfContents content;
int helpPage;
const int maxHelpPage = 5;

void initHelpMenu(int width, int height) {
    initTableOfContents(&content, width, height);

    addSection(&content, "Control", 2);
    addSubSection(&content, "Keyboard", 2);
    addSubSubSection(&content, "Help menu", 2);
    addSubSubSection(&content, "Exit window", 2);
    addSubSubSection(&content, "Start menu", 2);
    addSubSubSection(&content, "Game control", 2);
    addSubSection(&content, "Mouse input", 3);
    addSubSubSection(&content, "Help menu", 3);
    addSubSubSection(&content, "Exit window", 3);
    addSubSubSection(&content, "Start menu", 3);
    addSection(&content, "Console arguments", 4);
    addSection(&content, "Menus", 5);
    addSubSection(&content, "Help menu", 5);
    addSubSection(&content, "Exit window", 5);
    addSubSection(&content, "Game screen", 5);
    addSection(&content, "Gameplay", 6);
    addSubSection(&content, "Play", 6);
    addSubSection(&content, "Game over", 6);
}

void drawHelp(void) {
    setColor(CL_COLOR_YELLOW, CL_COLOR_NO_COLOR);
    setUnderline(1);
    drawf("Help menu");

    setCursorPos(0, 2);
    switch(helpPage) {
        case 0:
            setUnderline(0);
            drawContent(&content, 0);

            break;
        case 1:
            setColor(CL_COLOR_BLUE, CL_COLOR_NO_COLOR);
            drawf("1 Control\n");

            setColor(CL_COLOR_GREEN, CL_COLOR_NO_COLOR);
            drawf("1.1 Keyboard\n");

            setUnderline(0);
            setColor(CL_COLOR_LIGHT_RED, CL_COLOR_NO_COLOR);
            drawf("F1");
            resetColor();
            drawf(": Open help menu");

            setUnderline(1);
            setColor(CL_COLOR_CYAN, CL_COLOR_NO_COLOR);
            setCursorPos(0, 6);
            drawf("1.1.1 Help menu\n");

            setUnderline(0);
            setColor(CL_COLOR_LIGHT_RED, CL_COLOR_NO_COLOR);
            drawf("ESC");
            resetColor();
            drawf("/");
            setColor(CL_COLOR_LIGHT_RED, CL_COLOR_NO_COLOR);
            drawf("F1");
            resetColor();
            drawf(": Exit help menu\n");

            setColor(CL_COLOR_LIGHT_RED, CL_COLOR_NO_COLOR);
            drawf("UP");
            resetColor();
            drawf("/");
            setColor(CL_COLOR_LIGHT_RED, CL_COLOR_NO_COLOR);
            drawf("DOWN");
            resetColor();
            drawf(": Switch page");

            setUnderline(1);
            setColor(CL_COLOR_CYAN, CL_COLOR_NO_COLOR);
            setCursorPos(0, 10);
            drawf("1.1.2 Exit window\n");

            setUnderline(0);
            setColor(CL_COLOR_LIGHT_RED, CL_COLOR_NO_COLOR);
            drawf("y");
            resetColor();
            drawf("/");
            setColor(CL_COLOR_LIGHT_RED, CL_COLOR_NO_COLOR);
            drawf("n");
            resetColor();
            drawf(": Yes (Exit)/No (Not exit)");

            setUnderline(1);
            setColor(CL_COLOR_CYAN, CL_COLOR_NO_COLOR);
            setCursorPos(0, 13);
            drawf("1.1.3 Start menu\n");

            setUnderline(0);
            setColor(CL_COLOR_LIGHT_RED, CL_COLOR_NO_COLOR);
            drawf("ENTER");
            resetColor();
            drawf(": Start game\n");

            setColor(CL_COLOR_LIGHT_RED, CL_COLOR_NO_COLOR);
            drawf("ESC");
            resetColor();
            drawf(": Exit window");

            setUnderline(1);
            setColor(CL_COLOR_CYAN, CL_COLOR_NO_COLOR);
            setCursorPos(0, 17);
            drawf("1.1.4 Game control\n");

            setUnderline(0);
            setColor(CL_COLOR_LIGHT_RED, CL_COLOR_NO_COLOR);
            drawf("Arrow keys");
            resetColor();
            drawf(": Move position\n");
            setColor(CL_COLOR_LIGHT_RED, CL_COLOR_NO_COLOR);
            drawf("r");
            resetColor();
            drawf(": Reset level\n");
            setUnderline(0);
            setColor(CL_COLOR_LIGHT_RED, CL_COLOR_NO_COLOR);
            drawf("z");
            resetColor();
            drawf(": One step back/forward");

            break;
        case 2:
            setColor(CL_COLOR_GREEN, CL_COLOR_NO_COLOR);
            drawf("1.2 Mouse input\n");

            setUnderline(0);
            resetColor();
            drawf("Left click: [");
            setColor(CL_COLOR_NO_COLOR, CL_COLOR_YELLOW);
            drawf("L");
            resetColor();
            drawf("] \"Position\"\n");
            drawf("Right click: [");
            setColor(CL_COLOR_NO_COLOR, CL_COLOR_YELLOW);
            drawf("R");
            resetColor();
            drawf("] \"Position\"\n");
            drawf("Middle click: [");
            setColor(CL_COLOR_NO_COLOR, CL_COLOR_YELLOW);
            drawf("M");
            resetColor();
            drawf("] \"Position\"");

            setUnderline(1);
            setColor(CL_COLOR_CYAN, CL_COLOR_NO_COLOR);
            setCursorPos(0, 7);
            drawf("1.2.1 Help menu\n");

            setUnderline(0);
            resetColor();
            drawf("[");
            setColor(CL_COLOR_NO_COLOR, CL_COLOR_YELLOW);
            drawf("L");
            resetColor();
            drawf("] \"Page: 00\": Switch page (The same as ");
            setColor(CL_COLOR_LIGHT_RED, CL_COLOR_NO_COLOR);
            drawf("DOWN");
            resetColor();
            drawf(")\n[");
            setColor(CL_COLOR_NO_COLOR, CL_COLOR_YELLOW);
            drawf("L");
            resetColor();
            drawf("] Chapter at first pages: Goto page");

            setUnderline(1);
            setColor(CL_COLOR_CYAN, CL_COLOR_NO_COLOR);
            setCursorPos(0, 11);
            drawf("1.2.2 Exit window\n");

            setUnderline(0);
            resetColor();
            drawf("[");
            setColor(CL_COLOR_NO_COLOR, CL_COLOR_YELLOW);
            drawf("L");
            resetColor();
            drawf("] \"[y]es\": Yes (The same as ");
            setColor(CL_COLOR_LIGHT_RED, CL_COLOR_NO_COLOR);
            drawf("y");
            resetColor();
            drawf(")\n[");
            setColor(CL_COLOR_NO_COLOR, CL_COLOR_YELLOW);
            drawf("L");
            resetColor();
            drawf("] \"[n]o\": No (The same as ");
            setColor(CL_COLOR_LIGHT_RED, CL_COLOR_NO_COLOR);
            drawf("n");
            resetColor();
            drawf(")");

            setUnderline(1);
            setColor(CL_COLOR_CYAN, CL_COLOR_NO_COLOR);
            setCursorPos(0, 15);
            drawf("1.2.3 Start menu\n");

            setUnderline(0);
            resetColor();
            drawf("[");
            setColor(CL_COLOR_NO_COLOR, CL_COLOR_YELLOW);
            drawf("L");
            resetColor();
            drawf("] \"ENTER\": Start game (The same as ");
            setColor(CL_COLOR_LIGHT_RED, CL_COLOR_NO_COLOR);
            drawf("ENTER");
            resetColor();
            drawf(")\n[");
            setColor(CL_COLOR_NO_COLOR, CL_COLOR_YELLOW);
            drawf("L");
            resetColor();
            drawf("] \"Help: F1\": Open help menu (The same as ");
            setColor(CL_COLOR_LIGHT_RED, CL_COLOR_NO_COLOR);
            drawf("F1");
            resetColor();
            drawf(")");

            break;
        case 3:
            setColor(CL_COLOR_BLUE, CL_COLOR_NO_COLOR);
            drawf("2 Console arguments\n");

            setUnderline(0);
            resetColor();
            drawf("1) No arguments\n2) \"Path to level pack 1\" \"Path to level pack 2\" ...");

            break;
        case 4:
            setColor(CL_COLOR_BLUE, CL_COLOR_NO_COLOR);
            drawf("3 Menus\n");
            setColor(CL_COLOR_GREEN, CL_COLOR_NO_COLOR);
            drawf("3.1 Help menu\n");

            setUnderline(0);
            resetColor();
            drawf("\"Page: x of y\": x: (Current page), y: (Last page)\n\"");
            setColor(CL_COLOR_BLUE, CL_COLOR_NO_COLOR);
            setUnderline(1);
            drawf("x Title");
            setUnderline(0);
            resetColor();
            drawf("\":     Heading 1 (Chapter Name)\n\"");
            setColor(CL_COLOR_GREEN, CL_COLOR_NO_COLOR);
            setUnderline(1);
            drawf("x.x Title");
            setUnderline(0);
            resetColor();
            drawf("\":   Heading 2 (Chapter.Chapter Name)\n\"");
            setColor(CL_COLOR_CYAN, CL_COLOR_NO_COLOR);
            setUnderline(1);
            drawf("x.x.x Title");
            setUnderline(0);
            resetColor();
            drawf("\": Heading 3 (Chapter.Chapter.Chapter Name)\n");

            setUnderline(1);
            setColor(CL_COLOR_GREEN, CL_COLOR_NO_COLOR);
            setCursorPos(0, 9);
            drawf("3.2 Exit window\n");

            setUnderline(0);
            resetColor();
            drawf("Confirm exit\n");

            setUnderline(1);
            setColor(CL_COLOR_GREEN, CL_COLOR_NO_COLOR);
            setCursorPos(0, 12);
            drawf("3.3 Game screen\n");

            setUnderline(0);
            setColor(CL_COLOR_PINK, CL_COLOR_NO_COLOR);
            drawf("Game field:");

            resetColor();
            setCursorPos(1, 14);
            drawf(": Empty\n       : One way doors\n : Wall\n : Player\n   : BOX\n "
            ": Goal");

            setColor(CL_COLOR_LIGHT_BLUE, CL_COLOR_NO_COLOR);
            setCursorPos(0, 14);
            drawf("-\n< ^ > v");
            setColor(CL_COLOR_LIGHT_GREEN, CL_COLOR_NO_COLOR);
            setCursorPos(0, 16);
            drawf("#");
            setColor(CL_COLOR_YELLOW, CL_COLOR_NO_COLOR);
            setCursorPos(0, 17);
            drawf("P");
            setColor(CL_COLOR_LIGHT_CYAN, CL_COLOR_NO_COLOR);
            setCursorPos(0, 18);
            drawf("@");
            setColor(CL_COLOR_PINK, CL_COLOR_NO_COLOR);
            setCursorPos(2, 18);
            drawf("@");
            setColor(CL_COLOR_LIGHT_RED, CL_COLOR_NO_COLOR);
            setCursorPos(0, 19);
            drawf("x");

            break;
        case 5:
            setColor(CL_COLOR_BLUE, CL_COLOR_NO_COLOR);
            drawf("4 Gameplay\n");
            setColor(CL_COLOR_GREEN, CL_COLOR_NO_COLOR);
            drawf("4.1 Game\n");

            setUnderline(0);
            resetColor();
            drawf("Move all boxes to the goals.");

            setUnderline(1);
            setColor(CL_COLOR_GREEN, CL_COLOR_NO_COLOR);
            setCursorPos(0, 6);
            drawf("4.2 Game over\n");

            setUnderline(0);
            resetColor();
            drawf("Press ");
            setColor(CL_COLOR_LIGHT_RED, CL_COLOR_NO_COLOR);
            drawf("ESC");
            resetColor();
            drawf(" to left the game (See: ");
            setColor(CL_COLOR_GREEN, CL_COLOR_NO_COLOR);
            drawf("1.1 Keyboard");
            resetColor();
            drawf(").");

            break;
    }

    setCursorPos(0, 22);
    resetColor();
    drawf("Page: ");
    setColor(CL_COLOR_CYAN, CL_COLOR_NO_COLOR);
    drawf("%d", helpPage + 1);
    resetColor();
    drawf(" of ");
    setColor(CL_COLOR_CYAN, CL_COLOR_NO_COLOR);
    drawf("%d", maxHelpPage + 1);
}
