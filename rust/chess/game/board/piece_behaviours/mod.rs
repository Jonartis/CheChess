
pub mod bishop_behaviour;
pub mod king_behaviour;
pub mod knight_behaviour;
pub mod pawn_behaviour;
pub mod queen_behaviour;
pub mod rook_behaviour;

use super::piece::{LocatedPiece, PieceOwnerType};
use super::Board;

pub trait PieceBehaviour
{
    fn can_move(&self, from : LocatedPiece, to : LocatedPiece, board: &Board) -> bool;
    fn board_display(&self, owner : PieceOwnerType) -> &'static str;
}
