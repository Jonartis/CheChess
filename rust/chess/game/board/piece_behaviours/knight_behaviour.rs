

use super::*;

pub struct KnightBehaviour
{}

impl PieceBehaviour for KnightBehaviour
{
    fn can_move(&self, _from : LocatedPiece, _to : LocatedPiece, _board: &Board) -> Result<bool, MovementError>
    {
        Ok(false)
    }

    fn board_display(&self, owner : PieceOwnerType) -> &'static str
    {
        //H for Horse as k is taken by King
        if owner == PieceOwnerType::Black { "h" } else { "H" }
    }
    
}