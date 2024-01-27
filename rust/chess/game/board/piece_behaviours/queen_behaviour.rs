

use super::*;

pub struct QueenBehaviour
{}

impl PieceBehaviour for QueenBehaviour
{
    fn can_move(&self, _from : LocatedPiece, _to : LocatedPiece, _board: &Board) -> Result<bool, MovementError>
    {
        Ok(false)
    }

    fn board_display(&self, owner : PieceOwnerType) -> &'static str
    {
        if owner == PieceOwnerType::Black { "q" } else { "Q" }
    }
    
}