chess? in rust? *rusty* chess? yup! 

built on bitboards. generates valid moves. *makes* moves. has *special* moves like en passant and castling. yum!
## Note
To read more in-depth documentation, see `target/doc/chess/index.html`.

## Modules
- `piece` - piece types (pawn, rook, knight, bishop, queen, king) and colors (white, black)
- `bitboard` - the 64 bits and nothing else
- `position` - tracks positions of all pieces on board 
- `game` - tracks game state; including GameResult (OnGoing, Checkmate, Stalemate), as well as information about the game (Position, turn, selected *piece*)
- `moves` - generates valid moves via `valid_moves` function, and defined Move struct (from, to, piece).
- `special_moves` - castling, and promotion logic (en passant is stored in **moves**)
- `make_move` - applies Move to Game and its Position
- `helper` - utility functions like `initialize_board`, `index_to_square`, `square_to_index` and `print_debug_board`

## Using the Library
To import the library, use:
```rust
use chess::*
```
The basic flow of usage is:
1.  Initialize the **position** of all the pieces using `initialize_board()`, and create a new instance of **Game** with this position;
```rust
let position = initialize_board();
let mut game = Game::new(position);
```
2. Select a piece given a location on the board by indicating its position on the board: `select_piece(square: u8)`. 
*In the first round of the game, the starting color is White, and accordingly, only a White Piece can be selected.* The function will return a Piece enum if there is a piece on the square.

> Note: The current player can be checked with `player_tracker()` within the game module.
3. Pass the selected piece (`Piece`) returned by the function into `valid_moves(from: u8, piece: Piece, position: &Position)` where *from* is the square from which you selected the piece
It will return possible moves for the chosen piece as a `Vec<Move>`. 
```rust
let moves = valid_moves(from_square, piece, &game.position);
```

> *Note*! from_square only accepts bit indexes (0...63). To change chess notation (A1...H8) to bits, use the `square_to_index(square: &str)`helper function.

> Note: Valid moves returns from/to squares as bits. To convert them to chess notation, use `index_to_square(index: u8)`
4.  Execute one of the moves in the vector by calling `make_move(m: Move, game: &mut Game)`. The function will return Ok() or Err(). If successful, the board will be updated, including game statuses. 
> *Note*: It is recommended to fetch your chosen move by taking its index in the Vec<Move>, for example `let chosen_move = moves[idx]`
5. Whether the game has ended can be checked with the boolean function `is_over()` from the game module, which returns *False* if it is not over, and *True* if it is over. If the game ends, the result can be seen via `result` in Game, which will return `Checkmate(Color)`, where `Color` is the *checked color*, or `Stalemate`. 
6. If the game has not ended, the game will continue onward onto the next turn. The current player will change to the opposite color.  
## Credits
- The entirety of the [Chess programming wiki](https://www.chessprogramming.org/Main_Page) but especially the page on [board representatin](https://www.chessprogramming.org/Board_Representation) for introducing me to BitBoards, and [Bitboard Serialization](https://www.chessprogramming.org/Bitboard_Serialization) to create `make_move`, and the page on [En passant](https://www.chessprogramming.org/En_passant) for introducing me to the idea of "the en passant square".
- [Writing a BitBoard in Rust Pt. 1: The Basics](https://nereuxofficial.github.io/posts/bitboard-rust/) for writing the BitBoard structure I used.
- The entirety of [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html) for teaching me Rust
- Deepseek for giving me the idea to use fold() when combining my BitBoards. (Question used: `How could I get the reduced version of multiple bitboards in rust, using an OR operation? (i.e. a combined version) 
Don't give me the code, but give me pointers as to what to look at / research`)

- Wikipedia for various diagrams and formulations to help me understand chess. 
