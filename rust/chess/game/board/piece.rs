

use core::fmt;
use std::cmp::PartialEq;
use super::movement::*;
use super::board::Board;
use super::piece_behaviours::*;

#[derive(strum_macros::Display, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PieceOwnerType
{
    White,
    Black
}

impl PieceOwnerType
{
    pub fn flip(&mut self)
    {
        if *self == PieceOwnerType::Black { *self = PieceOwnerType::White } else { *self = PieceOwnerType::Black }
    }
}

#[derive(strum_macros::Display, PartialEq, Eq, Hash, Copy, Clone)]
pub enum PieceType
{
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

impl PieceType
{
    pub fn create_behaviour(&self) -> Box<dyn PieceBehaviour>
    {
        match *self
        {
            PieceType::Pawn => Box::new(pawn_behaviour::PawnBehaviour{}),
            PieceType::Rook => Box::new(rook_behaviour::RookBehaviour{}),
            PieceType::Knight => Box::new(knight_behaviour::KnightBehaviour{}),
            PieceType::Bishop => Box::new(bishop_behaviour::BishopBehaviour{}),
            PieceType::Queen => Box::new(queen_behaviour::QueenBehaviour{}),
            PieceType::King => Box::new(king_behaviour::KingBehaviour{})
        }
    }
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
    pub fn make(piece_type : PieceType, owner : PieceOwnerType) -> Piece
    {
        Piece { behaviour: piece_type.create_behaviour(), owner }
    }

    pub fn can_move(&self, from : &LocatedPiece, to : &LocatedPiece, board : &Board) -> bool
    {
        let can_move = match to.opt_piece
        {
            Some(piece) => piece.get_owner() != self.get_owner(),
            None => true
        };
        
        return can_move && self.behaviour.can_move(from, to, board);
    }

    pub fn get_owner(&self) -> PieceOwnerType
    {
        self.owner
    }

    pub fn is_game_ending_piece(&self) -> bool
    {
        self.get_type() == PieceType::King
    }

    pub fn is_upgradeable_piece(&self) -> bool
    {
        self.get_type() == PieceType::Pawn
    }

    pub fn get_type(&self) -> PieceType
    {
        self.behaviour.get_type()
    }
    
}

impl fmt::Display for Piece
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", self.behaviour.to_board_string(self.owner))
    }
}