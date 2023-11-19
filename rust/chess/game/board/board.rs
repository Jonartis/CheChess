
//Always 8x8
pub const BOARD_WIDTH:usize = 8;
pub const BOARD_HEIGTH:usize = 8;

use super::piece_type::PieceType;
use super::movement::Location;
use core::fmt;
use crate::error;

type RowType = [PieceType; BOARD_WIDTH];

pub struct Board
{
    table : [RowType; BOARD_HEIGTH] //2D array
}

impl Board
{
    pub fn default() -> Board
    {
        const MAIN_LINE:RowType =
        [PieceType::Rook, PieceType::Knight, PieceType::Bishop, PieceType::Queen, PieceType::King, PieceType::Bishop, PieceType::Knight, PieceType::Rook];

        const PAWN_LINE:RowType = [PieceType::Pawn; BOARD_WIDTH];
        const EMPTY_LINE:RowType = [PieceType::None; BOARD_WIDTH];

        Board{ 
            table: [
                MAIN_LINE,
                PAWN_LINE,
                EMPTY_LINE,
                EMPTY_LINE,
                EMPTY_LINE,
                EMPTY_LINE,
                PAWN_LINE,
                MAIN_LINE]
        }
    }

    pub fn is_inside(loc :Location) -> bool
    {
        loc.row < BOARD_HEIGTH && loc.col < BOARD_WIDTH
    }

    pub fn try_move(&mut self, from :Location, to :Location) -> Result<(), error::MovementError>
    {
        if Board::is_inside(from) 
        {
            if Board::is_inside(to)
            {
                //TODO: Check ownership of each piece and whoever is moving them
                //TODO: Check rules of chess for each pieces allowed movement
                let piece = self.table[from.row][from.col];
                self.table[to.row][to.col] = piece;
                self.table[from.row][from.col] = PieceType::None;
            }
            else
            {
                return Err(error::MovementError::DestinationOutOfBounds);
            }
        }
        else 
        {    
            return Err(error::MovementError::SourceOutOfBounds);
        }
        Ok(())
    }

}

impl fmt::Display for Board
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        for (row_idx, row) in self.table.iter().enumerate()
        {
            write!(f, "{} ", BOARD_HEIGTH - row_idx)?;
            for cell in row
            {
                write!(f, "{} ", cell)?;
            }
            writeln!(f, "")?;
        }
        write!(f, "  ")?;
        for it in 0..BOARD_WIDTH
        {
            write!(f, "{} ", utils::ASCII_LOWER[it])?;
        }
        Ok(())
    }
}