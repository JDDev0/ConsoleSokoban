# ConsoleSokoban

A sokoban game for Console (Linux, Windows)

Download on itch.io: [Console Sokoban](https://jddev0.itch.io/console-sokoban)

## Gameplay
![image](https://github.com/user-attachments/assets/e8215abb-7437-47c4-a971-4d613405cef6)

## Compile & Run

### Linux
- Required packages: `cmake`, `make`, `gcc`, `libncurses-dev`

Compile & Run
1. `cmake -DCMAKE_BUILD_TYPE=Release -S . -B release`
2. `cmake --build release`
3. `release/Sokoban`

### Windows
Required programs:
1. Install cmake and add it to $PATH
2. Install MinGW and add it to $PATH

Compile & Run
1. `cmake -G "MinGW Makefiles" -DCMAKE_BUILD_TYPE=Release -S . -B release`
2. `cmake --build release`
3. Go into the `release` folder and double click `Sokoban.exe`
