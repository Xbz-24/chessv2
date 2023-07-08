# Chess Game in Rust
This is a detailed, multiplayer chess game implemented in Rust, with a graphical user interface, client-server communication, and artificial intelligence for automated play.
## Project Structure
The project is organized into several modules, each representing a key part of the game. Here is a brief overview of each module:

1. src/communication/: This module handles all the client-server communication tasks, which includes setting up and managing connections, handling incoming and outgoing data, etc.
2. src/game/: This module includes all the game logic, from managing the board state, validating and executing moves, to enforcing game rules and maintaining the game history.
3. src/gui/: This module is responsible for the graphical user interface of the game, including rendering the game board, highlighting valid moves, and showing game end popups.
4. src/player/: This module manages the player logic. It includes both human and AI players, and defines their behaviours and capabilities.
5. src/utils/: This module provides various utility functions and constants that are used throughout the project.
## Key components
Here is a more detailed look at some key components of the project:
- Client and Server: The Client initiates communication with the Server, and vice versa. They handle establishing a connection, sending/receiving moves, and maintaining the game state.
- Board: The Board holds the state of the game at any given moment. It is a 2D array of Tile objects.
- Game: The Game class manages the game session. It is responsible for updating the game state, checking the game status and enforcing the game rules.
- Pieces: hese classes represent the various types of chess pieces. Each piece knows its valid moves.
- Player and AIPlayer: These classes represent human and AI players. They decide which move to make and when.
- Utils: This module contains utility functions and constants. It includes a logger, a timer, and a coordinate converter, among other utilities.

For a more detailed breakdown of each component and module, refer to the individual mod.rs files in each directory.

## How to Run
Todo: lorem ipsum