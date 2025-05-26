use super::*;

pub struct RookBehaviour
{}

impl RookBehaviour
{
    pub fn can_move_as_rook(from: &LocatedPiece, to: &LocatedPiece, board : &Board) -> bool
    {
        let mut can_move = false;
        //If the rook is moving in a straight line...
        if from.location.get_col() == to.location.get_col() || from.location.get_row() == to.location.get_row()
        {
            //...and there are no pieces on the way.
            can_move = !board.has_piece_straight(from.location, to.location);
        }
        can_move
    }
}

impl PieceBehaviour for RookBehaviour
{
    fn can_move(&self, from : &LocatedPiece, to : &LocatedPiece, board : &Board) -> bool
    {
        RookBehaviour::can_move_as_rook(from, to, board)
    }

    fn get_type(&self) -> PieceType
    {
        PieceType::Rook
    }

    fn to_board_string(&self, owner : PieceOwnerType) -> &'static str
    {
        if owner == PieceOwnerType::Black { "r" } else { "R" }
    }
    
}