

use super::PieceBehaviour;
use super::LocatedPiece;

pub struct BishopBehaviour
{}


impl PieceBehaviour for BishopBehaviour
{
    fn can_move(&self, _from : LocatedPiece, _to : LocatedPiece) -> bool
    {
        false
    }

    fn board_display(&self) -> &'static str
    {
        "b"
    }
    
}