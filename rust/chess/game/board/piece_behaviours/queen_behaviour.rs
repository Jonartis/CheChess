

use super::PieceBehaviour;
use super::LocatedPiece;

pub struct QueenBehaviour
{}

impl PieceBehaviour for QueenBehaviour
{
    fn can_move(&self, _from : LocatedPiece, _to : LocatedPiece) -> bool
    {
        false
    }

    fn board_display(&self) -> &'static str
    {
        "q"
    }
    
}