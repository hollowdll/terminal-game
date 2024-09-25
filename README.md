# Overview

A terminal based classic RPG game where you fight enemies in randomly generated dungeons. No mouse required. You play with your keyboard in a fully interactive TUI mode.

The game is programmed in the Rust language and requires only the game binary to play.

The goal is to build your character as strong as possible and get as far in the dungeon as you can. You can get equippable items as you progress that make your character stronger. At the end of a dungeon, there is a boss fight that you need to win to advance to the next dungeon floor. The enemies get stronger on each floor. Enemies, items, drops and dungeon rooms are all randomized.

# Save file

The game saves your progress to a save file so you can continue playing the next time you open the game.

The file is located in a directory `terminal-rpg-game` in the user's config directory. The location is different depending on your operating system. E.g. on Linux it is `$XDG_CONFIG_HOME` or `$HOME/.config` and on Windows Roaming AppData (C:/Users/<user>/AppData/Roaming).

# Run the game

Open your terminal and go to the directory where the game binary is.
```sh
cd path/to/your/directory
```

Run and start the game
```sh
./terminal_rpg
```
Make sure the file has execute permission.

# Build from source

You need the Rust compiler to build the game binary from source. Cargo package manager is also useful.

Clone this repo with Git and go to the game crate directory:
```sh
git clone https://github.com/hollowdll/terminal-game.git
cd path/to/your/clone/location/terminal-game/game
```

Build in debug mode and run
```sh
cargo run
```

Build release version
```sh
cargo build --release
```
The output binary will be placed in the target directory (target/release).

# Configuration

Build time environment variables:

- `TERM_RPG_GAME_MODE` - Sets the mode the binary will be built in. When set to "development", enables some features useful in development.
