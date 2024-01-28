

use super::*;

pub struct BishopBehaviour
{}

impl BishopBehaviour
{
    pub fn can_move_as_bishop(from: &LocatedPiece, to: &LocatedPiece, board: &Board) -> Result<bool, MovementError>
    {
        let mut can_move = false;
        let diffx = from.location.col.abs_diff(to.location.col);
        let diffy = from.location.row.abs_diff(to.location.row);
        //If we are moving diagonally...
        if diffx == diffy
        {
            can_move = !board.has_piece_diagonal(from.location, to.location)?;
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