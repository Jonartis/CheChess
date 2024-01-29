

use super::*;

pub struct KnightBehaviour
{}

impl PieceBehaviour for KnightBehaviour
{
    fn can_move(&self, from : LocatedPiece, to : LocatedPiece, _board: &Board) -> bool
    {
        let col_diff = from.location.get_col().abs_diff(to.location.get_col());
        let row_diff = from.location.get_row().abs_diff(to.location.get_row());

        let valid_move = col_diff <= 2 && row_diff <= 2 && col_diff + row_diff == 3;

        valid_move
    }

    fn board_display(&self, owner : PieceOwnerType) -> &'static str
    {
        //H for Horse as k is taken by King
        if owner == PieceOwnerType::Black { "h" } else { "H" }
    }
    
}