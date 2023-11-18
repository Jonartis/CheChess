

use core::fmt;

#[derive(Copy, Clone)]
pub enum PieceType
{
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
    None
}

impl fmt::Display for PieceType
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let as_str = match self {
            PieceType::Pawn => "p",
            PieceType::Bishop => "b",
            PieceType::Knight => "k",
            PieceType::Rook => "r",
            PieceType::Queen => "q",
            PieceType::King => "K",
            PieceType::None => "*"
        };
        write!(f, "{}", as_str)
    }
}