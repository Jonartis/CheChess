

use super::*;

pub struct BishopBehaviour
{}


impl PieceBehaviour for BishopBehaviour
{
    fn can_move(&self, _from : LocatedPiece, _to : LocatedPiece, _board: &Board) -> Result<bool, MovementError>
    {
        Ok(false)
    }

    fn board_display(&self) -> &'static str
    {
        "b"
    }
    
}