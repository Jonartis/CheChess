

use super::*;

pub struct PawnBehaviour
{}

impl PieceBehaviour for PawnBehaviour
{
    fn can_move(&self, from : LocatedPiece, to : LocatedPiece, _board : &Board) -> Result<bool, MovementError>
    {
        let mut can_move : bool = false;

        //Can move if the movement is a single step forward... 
        if from.opt_piece.is_some() && to.location.row == from.location.row.wrapping_sub(1)
        {
            //...and in the same column...
            if from.location.col == to.location.col
            {
                //...and there is no piece ahead
                can_move = to.opt_piece.is_none();
            }
            //Or if it is a single diagonal movement... 
            else if to.location.col == from.location.col.wrapping_sub(1) || to.location.col == from.location.col.wrapping_add(1)
            {
                //...and there is an enemy piece to eat
                can_move = match to.opt_piece
                {
                    Some(piece) => piece.get_owner() != from.opt_piece.unwrap().get_owner(),
                    None => false
                }
            }
        }
        Ok(can_move)
    }

    fn board_display(&self, owner : PieceOwnerType) -> &'static str
    {
        if owner == PieceOwnerType::Black { "p" } else { "P" }
    }
    
}