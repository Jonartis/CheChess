

use self::queen_behaviour::QueenBehaviour;

use super::*;

pub struct KingBehaviour
{}

impl PieceBehaviour for KingBehaviour
{
    fn can_move(&self, from: LocatedPiece, to: LocatedPiece, board: &Board) -> Result<bool, MovementError>
    {
        let col_diff = from.location.col.abs_diff(to.location.col);
        let row_diff = from.location.row.abs_diff(to.location.row);
        
        let mut can_move = false;
        if col_diff <= 1 && row_diff <= 1
        {
            can_move = QueenBehaviour::can_move_as_queen(from, to, board)?;
        }
        Ok(can_move)
    }

    fn board_display(&self, owner : PieceOwnerType) -> &'static str
    {
        if owner == PieceOwnerType::Black { "k" } else { "K" }
    }
    
}