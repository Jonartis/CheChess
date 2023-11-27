
pub mod bishop_behaviour;
pub mod king_behaviour;
pub mod knight_behaviour;
pub mod pawn_behaviour;
pub mod queen_behaviour;
pub mod rook_behaviour;

use core::fmt;
use super::movement::LocatedPiece;

pub trait PieceBehaviour
{
    fn can_move(&self, from : LocatedPiece, to : LocatedPiece) -> bool;
    fn board_display(&self) -> &'static str;
}

impl fmt::Display for dyn PieceBehaviour
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", self.board_display())
    }
}