// note in deriving: https://doc.rust-lang.org/rust-by-example/trait/derive.html

/// BitBoard courtesy of: <https://nereuxofficial.github.io/posts/bitboard-rust/>
/// The BitBoard is a 64-bit number with 1 bit for every square of a chess board. 
/// Each piece-color combination has one associated BitBoard, such that there is one for White Queens, one for Black Queens, and so on.
#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct BitBoard(pub u64);