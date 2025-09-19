chess? in rust? *rusty* chess? yup! 

built on bitboards. generates valid moves. *makes* moves. has *special* moves like en passant and castling. yum!

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
1. Initalizing the board;
```rust
let position = initialize_board();
let mut game = Game::new(position);
```
2. Select a piece given a location on the board using `select_piece(square)`. 
In the first round of the game, the starting color is White, and accordingly, only a White Piece can be selected.
3. Pass the selected piece returned by the function into `valid_moves(from,piece,position)` where *from* is the square from which you selected the piece, and *position* is the current state of the board, given as a egenskap (???) of game. 
It will return possible moves for the chosen piece as a Vec<Move>. For example:
```rust
let moves = valid_moves(from_square, piece, &game.position);
```
> *Note*! from_square only accepts bit indexes (0...63). To change chess notation (A1...H8) to bits, use the `square_to_index(square)`helper function.

> Note: Valid moves returns from/to squares as bits. To convert them to chess notation, use `index_to_square(index: u8)`
4.  Execute one of the moves in the vector by calling `make_move(m, game)`, where m is the given move. The function will return Ok() or Err(). If successful, the board will be updated, including game statuses. 
> *Note*: It is recommended to fetch your chosen move by taking its index in the Vec<Move>, for example `let chosen_move = moves[idx]`
5. Whether the game has ended can be checked with the `is_over()` function from Game, which returns False if it is not over, and True if it is over. If the game ends, the result can be seen via `result` in Game, which will return Checkmate(Color), where Color is the checked color, or Stalemate. 
6. If the game has not ended, it is the opposite color's turn, to select a piece and make its move.
## Credits
- The entirety of the [Chess programming wiki](https://www.chessprogramming.org/Main_Page) but especially the page on [board representatin](https://www.chessprogramming.org/Board_Representation) for introducing me to BitBoards, and [Bitboard Serialization](https://www.chessprogramming.org/Bitboard_Serialization) to create `make_move`, and the page on [En passant](https://www.chessprogramming.org/En_passant) for introducing me to the idea of "the en passant square".
- [Writing a BitBoard in Rust Pt. 1: The Basics](https://nereuxofficial.github.io/posts/bitboard-rust/) for writing the BitBoard structure I used.
- The entirety of [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html) for teaching me Rust
- Deepseek for giving me the idea to use fold() when combining my BitBoards. (Question used: `How could I get the reduced version of multiple bitboards in rust, using an OR operation? (i.e. a combined version) 
Don't give me the code, but give me pointers as to what to look at / research`)

- Wikipedia for various diagrams and formulations to help me understand chess. 
