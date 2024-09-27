# Dungeon generation algorithm

The game generates dungeon floors with randomly generated rooms. Rooms are connected together in 2D grid style. Each room has X and Y coordinates. The start room has coordinates (0,0). Each room can have at most 2 adjacent rooms. Directions are up, down, right and left. Rooms are always generated in up, right or left direction from the previous room.

Here is how the dungeon floor generation works:

- Create start room
- Create the second room
- Connect the start room and the second room
- Create a random possible room
- Connect the current room and the created room
- Repeat until threshold for the boss room is reached
- Boss entrance room becomes a possible room to be randomized
- Create random rooms until boss entrance room is randomized
- Stop creating rooms
- randomize treasure chest to one random room
- randomize enemies to random rooms (1 enemy per room)
- randomize boss enemy
- randomize shop items

