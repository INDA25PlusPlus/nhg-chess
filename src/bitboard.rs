// bitboard.rs

// bitboard courtesy of https://nereuxofficial.github.io/posts/bitboard-rust/
// note in deriving: https://doc.rust-lang.org/rust-by-example/trait/derive.html
#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct BitBoard(pub u64);
