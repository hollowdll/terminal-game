# Overview

A terminal based RPG game where you fight enemies in randomly generated dungeons. No mouse required. You play with your keyboard in a fully interactive TUI mode.

The game is programmed in the Rust language and requires only the game binary to play.

The goal is to build your character as strong as possible and get as far in the dungeon as you can. You can get equippable items of different rarities as you progress that make your character stronger. At the end of a dungeon, there is a boss fight that you need to win to advance to the next dungeon floor. The enemies get stronger on each floor. Enemies, items, drops and dungeon rooms are all randomized.

# Demo

Below is a short gameplay video of the game

![gameplay video](https://imgur.com/a/NkTp335)

<iframe class="imgur-embed" width="560" height="315" frameborder="0" src="https://imgur.com/whLAJ9m/embed"></iframe>

Below is a short gameplay video of a boss fight

![bossfight video](https://imgur.com/a/iDIw5bI)

# Install

Precompiled binaries for Linux and Windows can be downloaded [here](https://github.com/hollowdll/terminal-game/releases).

SHA256 checksums are available for all releases. You can verify the downloaded files against the SHA256 hashes in the checksums.txt found in every release.

Extract tar archive on Linux
```sh
tar xzvf terminal_rpg_<version>_linux.tar.gz
```

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

# Save file

The game saves your progress to a save file `terminal_rpg_game_data` so you can continue playing the next time you open the game. The game creates this file if it doesn't exist when the game is saved.

The file is located in a directory `terminal-rpg-game` in the user's config directory. The location is different depending on your operating system. E.g. on Linux it is `$XDG_CONFIG_HOME` or `$HOME/.config` and on Windows Roaming AppData (C:/Users/youruser/AppData/Roaming).

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

# Run with Docker

You can also run the game with Docker. With this you need to remember that the save file won't persist by default if the container is removed. To fix this, you need to use a Docker volume.

Alternatively map the savefile's path from your host machine to the container so you can play with the same save file both on the host machine and in containers.

Clone this repo with Git if not done yet and go to the repo's root directory:
```sh
git clone https://github.com/hollowdll/terminal-game.git
cd path/to/your/clone/location/terminal-game
```

Build the image
```sh
docker build -t terminal-rpg .
```

Create a container and run the game (Persists save file to a volume)
```sh
docker run --name terminal_rpg -v terminal_rpg_game_data:/root/.config/terminal-rpg-game --rm -it terminal-rpg
```

Create a container and run the game (Mount your host machine save file to the container)
```sh
docker run --name terminal_rpg -v <your_savefile_path>:/root/.config/terminal-rpg-game/terminal_rpg_game_data --rm -it terminal-rpg
```

# Configuration

Build time environment variables:

- `TERM_RPG_GAME_MODE` - Sets the mode the binary will be built in. When set to "development", enables some features useful in development.

# Releases

This section is for the developer.

On Debian the following package is needed to cross-compile to Windows
```sh
# Install
sudo apt install gcc-mingw-w64-x64-64

# Verify installation
x86_64-w64-mingw32-gcc --version
```
Other Linux distros not tested.

Run the script `release.sh` in the project root to build new release files.
```sh
./release.sh 0.1.0
```
This will output a new version directory to `releases` directory in the game crate directory. It contains compressed archives of the game binary for supported platforms (Linux .tar.gz and Windows .zip).

It also creates a checksums.txt file that contains the sha256 checksums of the archives. With this users can verify that they install the right file.

