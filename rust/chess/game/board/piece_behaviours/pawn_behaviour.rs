

use super::*;

pub struct PawnBehaviour
{}

impl PieceBehaviour for PawnBehaviour
{
    fn can_move(&self, from: &LocatedPiece, to: &LocatedPiece, board : &Board) -> bool
    {
        let mut can_move : bool;
        let owner = from.opt_piece.unwrap().get_owner();

        if owner == PieceOwnerType::White
        {
            can_move = to.location.get_row() < from.location.get_row();
        }
        else
        {
            can_move = to.location.get_row() > from.location.get_row();
        }
        
        if can_move
        {
            if to.location.get_col() == from.location.get_col() //Straight movement
            {
                let has_extra_move = Board::is_starting_pawn_row(from.location.get_row(), owner);
                let num_allowed_steps = if has_extra_move { 2 } else { 1 };
                can_move = to.location.get_row().abs_diff(from.location.get_row()) <= num_allowed_steps;
                if can_move
                {
                    can_move = to.opt_piece.is_none() && !board.has_piece_straight(from.location, to.location);
                }
            }
            else //Diagonal movement
            {
                let row_diff = to.location.get_row().abs_diff(from.location.get_row());
                let col_diff = to.location.get_col().abs_diff(from.location.get_col());
                //Can move if we are moving a single diagonal and there is an enemy piece
                can_move = col_diff == 1 && row_diff == 1 && to.opt_piece.is_some_and(|other| other.get_owner() != owner);
            }
        }
        can_move
    }
    
    fn get_type(&self) -> PieceType
    {
        PieceType::Pawn
    }

    fn to_board_string(&self, owner : PieceOwnerType) -> &'static str
    {
        if owner == PieceOwnerType::Black { "p" } else { "P" }
    }
    
}