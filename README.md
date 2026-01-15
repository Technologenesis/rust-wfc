# Welcome to Modus

Modus is a text-based, multiplayer, roguelike RPG.  Its modular structure is intended to allow users to customize the game experience
to the greatest possible extent.

## Playing the Game

Clone the git repository and use `cargo run` to launch the game.  You will be prompted to enter basic character information;
once this is done, you can host or join a game.

In-game, you take actions by typing commands.  The parser is still very primitive; right now, the supported actions are:

- `circumspect`: this action will provide you with a list of objects in the world and a short description for each.
- `attack <handle>`: this action will cause your character to punch the object with the given handle, applying your character's `punch_force`.
- `collect <handle>`: this action will cause the object with the given handle to be removed from the world, converted into an inventory object,
and added to your inventory.
- `interact <handle>`: this action will cause your character to interact with the given object in a way that depends on the object being interacted with.
