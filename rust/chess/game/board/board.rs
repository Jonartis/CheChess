
//Always 8x8
pub const BOARD_WIDTH:usize = 8;
pub const BOARD_HEIGTH:usize = 8;

use crate::game::board::piece::PieceOwnerType;

use super::error::MovementError;

use super::piece::Piece;
use super::movement::Location;
use core::fmt;

type RowType = [Option<Piece>; BOARD_WIDTH];

pub struct Board
{
    table : [RowType; BOARD_HEIGTH] //2D array
}

impl Board
{
    fn create_main_row(owner : PieceOwnerType) -> RowType
    {
        [
            Some(Piece::make_rook(owner)),
            Some(Piece::make_knight(owner)),
            Some(Piece::make_bishop(owner)),
            Some(Piece::make_queen(owner)),
            Some(Piece::make_king(owner)),
            Some(Piece::make_bishop(owner)),
            Some(Piece::make_knight(owner)),
            Some(Piece::make_rook(owner))
        ]
    }

    fn create_pawn_row(owner : PieceOwnerType) -> RowType
    {
        const EMPTY:Option<Piece> = None;
        const EMPTY_LINE:RowType = [EMPTY;BOARD_WIDTH];

        let mut pawn_row:RowType = EMPTY_LINE;
        
        for i in 0..BOARD_WIDTH
        {
            pawn_row[i] = Some(Piece::make_pawn(owner))
        }
        
        return pawn_row;
    }

    pub fn default() -> Board
    {
        const BLACK:PieceOwnerType = PieceOwnerType::Black;
        const WHITE:PieceOwnerType = PieceOwnerType::White;
        const EMPTY:Option<Piece> = None;
        const EMPTY_LINE:RowType = [EMPTY;BOARD_WIDTH];

        Board{ 
            table: [
                Board::create_main_row(BLACK),
                Board::create_pawn_row(BLACK),
                EMPTY_LINE,
                EMPTY_LINE,
                EMPTY_LINE,
                EMPTY_LINE,
                Board::create_pawn_row(WHITE),
                Board::create_main_row(WHITE)]
        }
    }

    pub fn is_inside(loc :Location) -> bool
    {
        loc.row < BOARD_HEIGTH && loc.col < BOARD_WIDTH
    }

    pub fn try_move(&mut self, from :Location, to :Location) -> Result<(), MovementError>
    {
        if Board::is_inside(from) 
        {
            if Board::is_inside(to)
            {
                let opt_piece = self.table[from.row][from.col].as_ref();

                let piece = match opt_piece {
                    Some(piece) => piece,
                    None => return Err(MovementError::SourcePieceNotFound)
                };
                if piece.can_move(from, to, self)?
                {
                    self.table[to.row][to.col] = self.table[from.row][from.col].take();
                }
            }
            else
            {
                return Err(MovementError::DestinationOutOfBounds);
            }
        }
        else 
        {    
            return Err(MovementError::SourceOutOfBounds);
        }
        Ok(())
    }

    pub fn get_piece(&self, loc : Location) -> Result<Option<&Piece>, MovementError>
    {
        if Board::is_inside(loc) 
        {
            let piece = self.table[loc.row][loc.col].as_ref();
            return Ok(piece);
        }
        else 
        {    
            return Err(MovementError::SourceOutOfBounds);
        }
    }

    pub fn has_piece_straight(&self, start : Location, dest : Location) -> Result<bool, MovementError>
    {
        let mut found_piece = false;
        if start.col == dest.col
        {
            let row_range = if start.row > dest.row { dest.row+1..start.row } else { start.row+1..dest.row };
            for row in row_range
            {
                if self.get_piece(Location { row, col: start.col })?.is_some()
                {
                    found_piece = true;
                    break;
                }
            }
        }
        else
        {
            let col_range = if start.col > dest.col { dest.col+1..start.col } else { start.col+1..dest.col };
            for col in col_range
            {
                if self.get_piece(Location { row: start.row, col })?.is_some()
                {
                    found_piece = true;
                    break;
                }
            }
        }
        Ok(found_piece)
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
                match cell {
                    Some(piece) => write!(f, "{} ", piece)?,
                    None => write!(f, "* ")?
                };
                
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