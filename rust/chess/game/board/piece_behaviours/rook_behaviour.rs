

use super::PieceBehaviour;
use super::LocatedPiece;

pub struct RookBehaviour
{}

impl PieceBehaviour for RookBehaviour
{
    fn can_move(&self, _from : LocatedPiece, _to : LocatedPiece) -> bool
    {
        false
    }

    fn board_display(&self) -> &'static str
    {
        "r"
    }
    
}