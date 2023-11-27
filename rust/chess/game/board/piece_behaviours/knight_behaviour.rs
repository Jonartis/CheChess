

use super::PieceBehaviour;
use super::LocatedPiece;

pub struct KnightBehaviour
{}

impl PieceBehaviour for KnightBehaviour
{
    fn can_move(&self, _from : LocatedPiece, _to : LocatedPiece) -> bool
    {
        false
    }

    fn board_display(&self) -> &'static str
    {
        "k"
    }
    
}