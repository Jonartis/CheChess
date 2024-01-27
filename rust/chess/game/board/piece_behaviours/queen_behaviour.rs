

use self::{bishop_behaviour::BishopBehaviour, rook_behaviour::RookBehaviour};

use super::*;

pub struct QueenBehaviour
{}

impl PieceBehaviour for QueenBehaviour
{
    fn can_move(&self, from : LocatedPiece, to : LocatedPiece, board: &Board) -> Result<bool, MovementError>
    {
        let mut can_move = RookBehaviour::can_move_as_rook(&from, &to, board)?;
        if !can_move
        {
            can_move = BishopBehaviour::can_move_as_bishop(&from, &to, board)?;
        }
        Ok(can_move)
    }

    fn board_display(&self, owner : PieceOwnerType) -> &'static str
    {
        if owner == PieceOwnerType::Black { "q" } else { "Q" }
    }
    
}