

use core::fmt;

use super::movement::*;
use super::board::Board;
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

pub struct LocatedPiece<'a>
{
    pub location : BoardLocation,
    pub opt_piece : Option<&'a Piece>
}

impl Piece
{
    pub fn make_pawn(owner : PieceOwnerType) -> Piece
    {
        Piece { behaviour: Box::new(pawn_behaviour::PawnBehaviour {}), owner }
    }

    pub fn make_rook(owner : PieceOwnerType) -> Piece
    {
        Piece { behaviour: Box::new(rook_behaviour::RookBehaviour {}), owner }
    }

    pub fn make_knight(owner : PieceOwnerType) -> Piece
    {
        Piece { behaviour: Box::new(knight_behaviour::KnightBehaviour {}), owner }
    }

    pub fn make_bishop(owner : PieceOwnerType) -> Piece
    {
        Piece { behaviour: Box::new(bishop_behaviour::BishopBehaviour{}), owner }
    }

    pub fn make_queen(owner : PieceOwnerType) -> Piece
    {
        Piece { behaviour: Box::new(queen_behaviour::QueenBehaviour {}), owner }
    }

    pub fn make_king(owner : PieceOwnerType) -> Piece
    {
        Piece { behaviour: Box::new(king_behaviour::KingBehaviour {}), owner }
    }

    pub fn can_move(&self, from : BoardLocation, to : BoardLocation, board : &Board) -> bool
    {
        let from_piece = LocatedPiece { location: from, opt_piece: Some(self) };
        let to_piece = LocatedPiece { location: to, opt_piece: board.get_piece(to) };

        let can_move = match to_piece.opt_piece
        {
            Some(piece) => piece.get_owner() != self.get_owner(),
            None => true
        };
        
        return can_move && self.behaviour.can_move(from_piece, to_piece, board);
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
        write!(f, "{}", self.behaviour.board_display(self.owner))
    }
}