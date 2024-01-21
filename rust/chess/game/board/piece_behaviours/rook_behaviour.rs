use super::*;

pub struct RookBehaviour
{}

impl PieceBehaviour for RookBehaviour
{
    fn can_move(&self, from : LocatedPiece, to : LocatedPiece, board : &Board) -> Result<bool, MovementError>
    {
        let mut can_move = false;
        //If the rook is moving in a straight line...
        if from.location.col == to.location.col || from.location.row == to.location.row
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
                can_move = !board.has_piece_straight(from.location, to.location)?;
            }

        }
        Ok(can_move)
    }

    fn board_display(&self) -> &'static str
    {
        "r"
    }
    
}