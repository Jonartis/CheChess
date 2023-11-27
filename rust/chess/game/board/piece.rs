

use core::fmt;

use super::movement::*;
use super::board::Board;
use super::error::MovementError;
use super::piece_behaviours::*;

#[derive(Copy, Clone, PartialEq)]
pub enum PieceOwnerType
{
    White,
    Black
}

pub struct Piece
{
    behaviour : Box<dyn PieceBehaviour>,
    owner : PieceOwnerType,
}


impl Piece
{
    pub fn make_bishop(owner : PieceOwnerType) -> Piece
    {
        Piece { behaviour: Box::new(bishop_behaviour::BishopBehaviour {}), owner }
    }

    pub fn make_king(owner : PieceOwnerType) -> Piece
    {
        Piece { behaviour: Box::new(king_behaviour::KingBehaviour {}), owner }
    }

    pub fn make_knight(owner : PieceOwnerType) -> Piece
    {
        Piece { behaviour: Box::new(knight_behaviour::KnightBehaviour {}), owner }
    }

    pub fn make_pawn(owner : PieceOwnerType) -> Piece
    {
        Piece { behaviour: Box::new(pawn_behaviour::PawnBehaviour {}), owner }
    }

    pub fn make_queen(owner : PieceOwnerType) -> Piece
    {
        Piece { behaviour: Box::new(queen_behaviour::QueenBehaviour {}), owner }
    }

    pub fn make_rook(owner : PieceOwnerType) -> Piece
    {
        Piece { behaviour: Box::new(rook_behaviour::RookBehaviour {}), owner }
    }

    pub fn can_move(&self, from : Location, to : Location, board : &Board) -> Result<bool, MovementError>
    {
        //TODO: the from must be a valid piece always as we are getting selected, can we pass the piece through? (maybe just the owner)
        let from_piece = LocatedPiece { location: from, opt_piece: board.get_piece(from)? };
        let to_piece = LocatedPiece { location: to, opt_piece: board.get_piece(to)? };
        Ok(self.behaviour.can_move(from_piece, to_piece))
    }

    pub fn get_owner(&self) -> PieceOwnerType
    {
        self.owner
    }
    
}

impl fmt::Display for Piece
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", self.behaviour)
    }
}