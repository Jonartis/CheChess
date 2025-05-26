

use self::{bishop_behaviour::BishopBehaviour, rook_behaviour::RookBehaviour};

use super::*;

pub struct QueenBehaviour
{}

impl QueenBehaviour
{
    pub fn can_move_as_queen(from : &LocatedPiece, to : &LocatedPiece, board: &Board) -> bool
    {
        let mut can_move = RookBehaviour::can_move_as_rook(&from, &to, board);
        if !can_move
        {
            can_move = BishopBehaviour::can_move_as_bishop(&from, &to, board);
        }
        can_move
    }
}

impl PieceBehaviour for QueenBehaviour
{
    fn can_move(&self, from : &LocatedPiece, to : &LocatedPiece, board: &Board) -> bool
    {
        QueenBehaviour::can_move_as_queen(from, to, board)
    }

    fn get_type(&self) -> PieceType
    {
        PieceType::Queen
    }

    fn to_board_string(&self, owner : PieceOwnerType) -> &'static str
    {
        if owner == PieceOwnerType::Black { "q" } else { "Q" }
    }
    
}