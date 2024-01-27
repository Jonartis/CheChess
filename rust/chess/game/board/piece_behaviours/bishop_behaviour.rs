

use super::*;

pub struct BishopBehaviour
{}

impl BishopBehaviour
{
    pub fn can_move_as_bishop(from: &LocatedPiece, to: &LocatedPiece, board: &Board) -> Result<bool, MovementError>
    {
        let mut can_move = false;
        let diffx = (from.location.col - to.location.col).abs();
        let diffy = (from.location.row - to.location.row).abs();
        //If we are moving diagonally...
        if diffx == diffy
        {
            //...and the target is empty or is an enemy piece...
            let valid_destination = match to.opt_piece
            {
                Some(piece) => piece.get_owner() != from.opt_piece.unwrap().get_owner(),
                None => true
            };
            //...and there are no pieces on the way.
            if valid_destination
            {
                can_move = !board.has_piece_diagonal(from.location, to.location)?;
            }

        }
        Ok(can_move)
    }
}

impl PieceBehaviour for BishopBehaviour
{
    fn can_move(&self, from : LocatedPiece, to : LocatedPiece, board: &Board) -> Result<bool, MovementError>
    {
        BishopBehaviour::can_move_as_bishop(&from, &to, board)
    }

    fn board_display(&self, owner : PieceOwnerType) -> &'static str
    {
        if owner == PieceOwnerType::Black { "b" } else { "B" }
    }
    
}