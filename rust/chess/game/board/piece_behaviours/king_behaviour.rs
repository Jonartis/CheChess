

use super::*;

pub struct KingBehaviour
{}

impl PieceBehaviour for KingBehaviour
{
    fn can_move(&self, _from : LocatedPiece, _to : LocatedPiece, _board: &Board) -> Result<bool, MovementError>
    {
        Ok(false)
    }

    fn board_display(&self) -> &'static str
    {
        "K"
    }
    
}