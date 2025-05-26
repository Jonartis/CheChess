

use super::*;

pub struct BishopBehaviour
{}

impl BishopBehaviour
{
    pub fn can_move_as_bishop(from: &LocatedPiece, to: &LocatedPiece, board: &Board) -> bool
    {
        let mut can_move = false;
        let diffx = from.location.get_col().abs_diff(to.location.get_col());
        let diffy = from.location.get_row().abs_diff(to.location.get_row());
        //If we are moving diagonally...
        if diffx == diffy
        {
            can_move = !board.has_piece_diagonal(from.location, to.location);
        }
        can_move
    }
}

impl PieceBehaviour for BishopBehaviour
{
    fn can_move(&self, from : &LocatedPiece, to : &LocatedPiece, board: &Board) -> bool
    {
        BishopBehaviour::can_move_as_bishop(from, to, board)
    }

    fn get_type(&self) -> PieceType
    {
        PieceType::Bishop
    }

    fn to_board_string(&self, owner : PieceOwnerType) -> &'static str
    {
        if owner == PieceOwnerType::Black { "b" } else { "B" }
    }
    
}