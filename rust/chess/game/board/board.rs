
//Always 8x8
pub const BOARD_SIZE:i32 = 8;
pub const BOARD_SIZE_US:usize = BOARD_SIZE as usize;

use super::error::MovementError;
use super::movement::*;
use super::piece::Piece;
use super::piece::PieceOwnerType;
use core::fmt;

type RowType = [Option<Piece>; BOARD_SIZE_US];

pub struct Board
{
    table : [RowType; BOARD_SIZE_US] //2D array
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
        const EMPTY_LINE:RowType = [EMPTY;BOARD_SIZE_US];

        let mut pawn_row:RowType = EMPTY_LINE;
        
        for i in 0..BOARD_SIZE_US
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
        const EMPTY_LINE:RowType = [EMPTY;BOARD_SIZE_US];

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

    pub fn try_move(&mut self, from: Location, to: Location) -> Result<bool, MovementError>
    {
        let board_from: BoardLocation = match from.try_into() {
            Ok(new_loc) => new_loc,
            Err(_) => { return Err(MovementError::SourceOutOfBounds); }
        };
        
        let board_to: BoardLocation = match to.try_into() {
            Ok(new_loc) => new_loc,
            Err(_) => { return Err(MovementError::DestinationOutOfBounds); }
        };
        
        let opt_piece = self.table[board_from.get_row()][board_from.get_col()].as_ref();

        let piece = match opt_piece {
            Some(piece) => piece,
            None => return Err(MovementError::SourcePieceNotFound)
        };

        let can_move = piece.can_move(from, to, self)?;
        if can_move
        {
            self.table[board_to.get_row()][board_to.get_col()] = self.table[board_from.get_row()][board_from.get_col()].take();
        }
        Ok(can_move)
    }

    pub fn get_piece(&self, loc : BoardLocation) -> Option<&Piece>
    {
        self.table[loc.get_row()][loc.get_col()].as_ref()
    }

    pub fn has_piece_straight(&self, in_from : Location, in_to : Location) -> Result<bool, MovementError>
    {
        let start: BoardLocation = match in_from.try_into() {
            Ok(new_loc) => new_loc,
            Err(_) => { return Err(MovementError::SourceOutOfBounds); }
        };
        
        let dest: BoardLocation = match in_to.try_into() {
            Ok(new_loc) => new_loc,
            Err(_) => { return Err(MovementError::DestinationOutOfBounds); }
        };

        let mut found_piece = false;
        if start.get_col() == dest.get_col()
        {
            let row_range = if start.get_row() > dest.get_row() { dest.get_row()+1..start.get_row() } else { start.get_row()+1..dest.get_row() };
            for row in row_range
            {
                let test_loc = BoardLocation::try_create(row, start.get_col()).unwrap();
                if self.get_piece(test_loc).is_some()
                {
                    found_piece = true;
                    break;
                }
            }
        }
        else
        {
            let col_range = if start.get_col() > dest.get_col() { dest.get_col()+1..start.get_col() } else { start.get_col()+1..dest.get_col() };
            for col in col_range
            {
                let test_loc = BoardLocation::try_create(start.get_row(), col).unwrap();
                if self.get_piece(test_loc).is_some()
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
            write!(f, "{} ", BOARD_SIZE_US - row_idx)?;
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
        for it in 0..BOARD_SIZE_US
        {
            write!(f, "{} ", utils::ASCII_LOWER[it])?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests
{
    use crate::game::board::Location;

    use super::Board;

    struct BoardTester
    {
        board : Board
    }

    impl BoardTester
    {
        pub fn create() -> Self
        {
            Self{board: Board::default()}
        }
        
        pub fn multi_move(&mut self, moves : &[&str])
        {
            let iter = moves.windows(2);
            for pairs in IntoIterator::into_iter(iter)
            {
                let _ = self.try_move(pairs[0], pairs[1]);
            }
        }

        pub fn try_move(&mut self, from_str: &str, to_str: &str) -> Result<bool, super::MovementError>
        {
            let from = Location::try_from(from_str).unwrap();
            let to = Location::try_from(to_str).unwrap();
            self.board.try_move(from, to)
        }

        pub fn has_piece(&self, from_str: &str) -> bool
        {
            let from = Location::try_from(from_str).unwrap();
            self.board.get_piece(from.try_into().unwrap()).is_some()
        }
    }

    #[test]
    fn board_tracks_movement()
    {
        let mut board = BoardTester::create();
        let from = "2a";
        let to = "3a";
        let move_result = board.try_move(from, to);
        assert!(move_result.is_ok_and(|moved| moved), "Failed to move pawn forward");
        assert!(!board.has_piece(from), "Found pawn in original location after moving it");
        assert!(board.has_piece(to), "Not found pawn in new location after moving it");
    }

    #[test]
    fn pawn_movement()
    {
        let mut board = BoardTester::create();
        {
            let move_result = board.try_move("2a", "3a");
            assert!(move_result.is_ok_and(|moved| moved), "Failed to move pawn forward");
        }
        {
            let move_result = board.try_move("3a", "5a");
            assert!(move_result.is_ok_and(|moved| !moved), "We should only be able to move one step at a time");
        }
        {
            let move_result = board.try_move("3a", "4b");
            assert!(move_result.is_ok_and(|moved| !moved), "We should have failed to move diagonally as there is no piece in the diagonal");
        }
        {
            //Move the pawn to a position where it can eat diagonally
            board.multi_move(&["3a","4a","5a","6a"]);
            let move_result = board.try_move("6a", "7b");
            assert!(move_result.is_ok_and(|moved| moved), "We should be able to move diagonally if there is an enemy piece on the way");
        }
    }

    #[test]
    fn rook_movement()
    {
        let mut board = BoardTester::create();
        //Open the way to the left rook!
        board.multi_move(&["2a","3a","4a","5a"]);
        {
            let move_result = board.try_move("1a", "2a");
            assert!(move_result.is_ok_and(|moved| moved), "Failed to move rook forward");
        }
        {
            let move_result = board.try_move("2a", "1a");
            assert!(move_result.is_ok_and(|moved| moved), "Failed to move rook backwards");
        }
        {
            let move_result = board.try_move("1a", "3a");
            assert!(move_result.is_ok_and(|moved| moved), "Failed to move rook forward multiple steps");
        }
        {
            let move_result = board.try_move("3a", "3f");
            assert!(move_result.is_ok_and(|moved| moved), "Failed to move rook sidewards multiple steps");
        }
        {
            let move_result = board.try_move("3f", "7f");
            assert!(move_result.is_ok_and(|moved| moved), "Failed to move consume enemy piece with rook");
        }
        {
            let move_result = board.try_move("7f", "2f");
            assert!(move_result.is_ok_and(|moved| !moved), "We shouldn't be able to eat our own pieces");
        }

    }

    #[ignore] //Ignored until bishop behaviour is implemented
    #[test]
    fn bishop_movement()
    {
        let mut board = BoardTester::create();
        //Open the way to the left bishop!
        board.multi_move(&["2d","3d","4d","5d"]);
        {
            let move_result = board.try_move("1c", "2d");
            assert!(move_result.is_ok_and(|moved| moved), "Failed to move bishop in diagonal");
        }
        {
            let move_result = board.try_move("2d", "4b");
            assert!(move_result.is_ok_and(|moved| moved), "Failed to move bishop in diagonal multiple steps");
        }
        {
            let move_result = board.try_move("4b", "7e");
            assert!(move_result.is_ok_and(|moved| moved), "Failed to eat an enemy piece");
        }
        {
            let move_result = board.try_move("4b", "7e");
            assert!(move_result.is_ok_and(|moved| moved), "Failed to eat an enemy piece");
        }
    }
}