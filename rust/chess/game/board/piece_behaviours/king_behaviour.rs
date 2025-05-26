

use self::queen_behaviour::QueenBehaviour;

use super::*;

pub struct KingBehaviour
{}

impl PieceBehaviour for KingBehaviour
{
    fn can_move(&self, from: &LocatedPiece, to: &LocatedPiece, board: &Board) -> bool
    {
        let col_diff = from.location.get_col().abs_diff(to.location.get_col());
        let row_diff = from.location.get_row().abs_diff(to.location.get_row());

        let mut can_move = false;
        if col_diff <= 1 && row_diff <= 1
        {
            can_move = QueenBehaviour::can_move_as_queen(from, to, board);
        }
        can_move
    }

    fn get_type(&self) -> PieceType
    {
        PieceType::King
    }

    fn to_board_string(&self, owner : PieceOwnerType) -> &'static str
    {
        if owner == PieceOwnerType::Black { "k" } else { "K" }
    }
    
}