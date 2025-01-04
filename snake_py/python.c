#include <curses.h>
#include <stdlib.h>
#include <unistd.h>

typedef struct {
    int x;
    int y;
} vec;

void main() {
    WINDOW* win = initscr();
    keypad(win, true);
    nodelay(win, true);

    int width = 40;
    int height = 20;

    vec pos = {0, 0};
    vec food = {rand() % 20, rand() % 20};
    vec dir = {1, 0};
    char *faceDir = ">";
    while (true) {
        int pressed = wgetch(win);
        if (pressed == KEY_LEFT) {
            if (dir.x == 1) continue;
            dir.x = -1;
            dir.y = 0;
            faceDir = "<";
        }
        if (pressed == KEY_RIGHT) {
            if (dir.x == -1) continue;
            dir.x = 1;
            dir.y = 0;
            faceDir = ">";
        }
        if (pressed == KEY_UP) {
            if (dir.y == 1) continue;
            dir.x = 0;
            dir.y = -1;
            faceDir = "^";
        }
        if (pressed == KEY_DOWN) {
            if (dir.y == -1) continue;
            dir.x = 0;
            dir.y = 1;
            faceDir = "v";
        }
        pos.x += dir.x;
        pos.y += dir.y;

        erase();
        mvaddstr(pos.y, pos.x * 2, faceDir);
        mvaddstr(food.y, food.x, "X");

        if (food.x == pos.x && food.y == pos.y) {
            food.x = rand() % 20;
            food.y = rand() % 20;
        }
        usleep(100000);
    }

    endwin();
}

void render_scr() {

}