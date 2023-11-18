
//Always 8x8
const BOARD_WIDTH:usize = 8;
const BOARD_HEIGTH:usize = 8;

mod piece_type;
use piece_type::PieceType;
use core::fmt;
extern crate utils;

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

        let brd = Board{ 
            table: [
                MAIN_LINE,
                PAWN_LINE,
                EMPTY_LINE,
                EMPTY_LINE,
                EMPTY_LINE,
                EMPTY_LINE,
                PAWN_LINE,
                MAIN_LINE]
        };
        brd
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