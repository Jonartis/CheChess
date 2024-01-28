use super::*;

pub struct RookBehaviour
{}

impl RookBehaviour
{
    pub fn can_move_as_rook(from: &LocatedPiece, to: &LocatedPiece, board : &Board) -> Result<bool, MovementError>
    {
        let mut can_move = false;
        //If the rook is moving in a straight line...
        if from.location.col == to.location.col || from.location.row == to.location.row
        {
            //...and there are no pieces on the way.
            can_move = !board.has_piece_straight(from.location, to.location)?;
        }
        Ok(can_move)
    }
}

impl PieceBehaviour for RookBehaviour
{
    fn can_move(&self, from : LocatedPiece, to : LocatedPiece, board : &Board) -> Result<bool, MovementError>
    {
        RookBehaviour::can_move_as_rook(&from, &to, board)
    }

    fn board_display(&self, owner : PieceOwnerType) -> &'static str
    {
        if owner == PieceOwnerType::Black { "r" } else { "R" }
    }
    
}