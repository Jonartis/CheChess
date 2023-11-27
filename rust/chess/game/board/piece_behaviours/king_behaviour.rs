

use super::PieceBehaviour;
use super::LocatedPiece;

pub struct KingBehaviour
{}

impl PieceBehaviour for KingBehaviour
{
    fn can_move(&self, _from : LocatedPiece, _to : LocatedPiece) -> bool
    {
        false
    }

    fn board_display(&self) -> &'static str
    {
        "K"
    }
    
}