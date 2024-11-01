# ConsoleSokoban

A sokoban game for Console (Linux, Windows)

Download on itch.io: [Console Sokoban](https://jddev0.itch.io/console-sokoban)

## Gameplay
![image](https://github.com/user-attachments/assets/e8215abb-7437-47c4-a971-4d613405cef6)

## Compile & Run
### Linux
- Required packages: `cmake`, `make`, `gcc`, `libncurses-dev`

1. `cmake -DCMAKE_BUILD_TYPE=Release -S . -B release`
2. `cmake --build release`
3. `release/Sokoban`
