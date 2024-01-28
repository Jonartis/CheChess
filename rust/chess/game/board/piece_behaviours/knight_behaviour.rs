

use super::*;

pub struct KnightBehaviour
{}

impl PieceBehaviour for KnightBehaviour
{
    fn can_move(&self, from : LocatedPiece, to : LocatedPiece, _board: &Board) -> Result<bool, MovementError>
    {
        let col_diff = from.location.col.abs_diff(to.location.col);
        let row_diff = from.location.row.abs_diff(to.location.row);

        let valid_move = col_diff <= 2 && row_diff <= 2 && col_diff + row_diff == 3;

        Ok(valid_move)
    }

    fn board_display(&self, owner : PieceOwnerType) -> &'static str
    {
        //H for Horse as k is taken by King
        if owner == PieceOwnerType::Black { "h" } else { "H" }
    }
    
}