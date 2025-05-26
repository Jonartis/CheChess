
pub mod bishop_behaviour;
pub mod king_behaviour;
pub mod knight_behaviour;
pub mod pawn_behaviour;
pub mod queen_behaviour;
pub mod rook_behaviour;

use super::piece::{LocatedPiece, PieceOwnerType, PieceType};
use super::Board;

pub trait PieceBehaviour
{
    fn can_move(&self, from : &LocatedPiece, to : &LocatedPiece, board: &Board) -> bool;
    fn to_board_string(&self, owner : PieceOwnerType) -> &'static str;
    fn get_type(&self) -> PieceType;
}
