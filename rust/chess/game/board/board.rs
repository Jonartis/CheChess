
//Always 8x8
pub const BOARD_SIZE:i32 = 8;
pub const BOARD_SIZE_US:usize = BOARD_SIZE as usize;

#[cfg(test)]
use super::error::InputError;

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

    pub fn create() -> Board
    {
        const BLACK: PieceOwnerType = PieceOwnerType::Black;
        const WHITE: PieceOwnerType = PieceOwnerType::White;
        const EMPTY: Option<Piece> = None;
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

    #[cfg(test)]
    pub fn create_empty() -> Board
    {
        const EMPTY: Option<Piece> = None;
        const EMPTY_LINE: RowType = [EMPTY; BOARD_SIZE_US];

        Board {table: [EMPTY_LINE; BOARD_SIZE_US]}
    }

    #[cfg(test)]
    pub fn add_piece(&mut self, location : Location, piece: Piece) -> Result<(), InputError>
    {
        let board_loc: BoardLocation = match location.try_into() {
            Ok(new_loc) => new_loc,
            Err(_) => { return Err(InputError::InvalidInput("Location is not inside board when adding piece".to_string())); }
        };

        self.table[board_loc.get_row()][board_loc.get_col()] = Some(piece);
        Ok(())
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

    pub fn has_piece_diagonal(&self, in_from : Location, in_to : Location) -> Result<bool, MovementError>
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

        let col_range: Vec<usize> = 
        if start.get_col() > dest.get_col() 
        { 
            (dest.get_col()+1..start.get_col()).collect()
        }
        else 
        { 
            (start.get_col()+1..dest.get_col()).rev().collect()
        };

        let row_range: Vec<usize> = 
        if start.get_row() > dest.get_row() 
        {
            (dest.get_row()+1..start.get_row()).collect()
        }
        else
        {
            (start.get_row()+1..dest.get_row()).rev().collect()
        };

        for it in col_range.into_iter().zip(row_range)
        {
            let (col, row) = it;
            let test_loc = BoardLocation::try_create(row, col).unwrap();
            if self.get_piece(test_loc).is_some()
            {
                found_piece = true;
                break;
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
    use crate::game::board::{piece::{Piece, PieceOwnerType}, Location};

    use super::Board;

    struct BoardTester
    {
        board : Board
    }

    impl BoardTester
    {
        pub fn create() -> Self
        {
            Self{board: Board::create_empty()}
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

        pub fn add_pawn(&mut self, loc_str: &str, owner: PieceOwnerType)
        {
            self.add_piece(loc_str, Piece::make_pawn(owner));
        }

        pub fn add_rook(&mut self, loc_str: &str, owner: PieceOwnerType)
        {
            self.add_piece(loc_str, Piece::make_rook(owner));
        }

        pub fn add_bishop(&mut self, loc_str: &str, owner: PieceOwnerType)
        {
            self.add_piece(loc_str, Piece::make_bishop(owner));
        }

        pub fn add_queen(&mut self, loc_str: &str, owner: PieceOwnerType)
        {
            self.add_piece(loc_str, Piece::make_queen(owner));
        }

        fn add_piece(&mut self, loc_str: &str, piece: Piece)
        {
            let loc = Location::try_from(loc_str).unwrap();
            match self.board.add_piece(loc, piece) {
                Ok(_) => {}
                Err(input_error) => {assert!(false, "{:?}", input_error);}
            }
        }

    }

    #[test]
    fn board_tracks_movement()
    {
        let mut board = BoardTester::create();

        board.add_pawn("2a", PieceOwnerType::White);

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

        board.add_pawn("2a", PieceOwnerType::White);
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
            //Place an enemy piece in the diagonal to test eating
            board.add_pawn("4b", PieceOwnerType::Black);
            board.add_pawn("5a", PieceOwnerType::Black);

            let move_result = board.try_move("3a", "4b");
            assert!(move_result.is_ok_and(|moved| moved), "We should be able to move diagonally if there is an enemy piece on the way (right diagonal)");
            let move_result = board.try_move("4b", "5a");
            assert!(move_result.is_ok_and(|moved| moved), "We should be able to move diagonally if there is an enemy piece on the way (left diagonal)");
        }
    }

    fn rook_movement_helper(board : &mut BoardTester, piece_name: &str)
    {
        {
            let move_result = board.try_move("1a", "3a");
            assert!(move_result.is_ok_and(|moved| moved), "Failed to move {piece_name} forward");
        }
        {
            let move_result = board.try_move("3a", "1a");
            assert!(move_result.is_ok_and(|moved| moved), "Failed to move {piece_name} backwards");
        }
        {
            let move_result = board.try_move("1a", "1d");
            assert!(move_result.is_ok_and(|moved| moved), "Failed to move {piece_name} to the right");
        }
        {
            let move_result = board.try_move("1d", "1a");
            assert!(move_result.is_ok_and(|moved| moved), "Failed to move {piece_name} to the left");
        }
        
        //Enemy joins the battle
        board.add_pawn("3a", PieceOwnerType::Black);

        {
            let move_result = board.try_move("1a", "7a");
            assert!(move_result.is_ok_and(|moved| !moved), "A {piece_name} shouldn't we able to move accross pieces");
        }
        {
            let move_result = board.try_move("1a", "3a");
            assert!(move_result.is_ok_and(|moved| moved), "A {piece_name} should be able to eat enemy pieces");
        }
    }

    #[test]
    fn rook_movement()
    {
        let mut board = BoardTester::create();
        board.add_rook("1a", PieceOwnerType::White);
        rook_movement_helper(&mut board, "rook");
    }

    fn bishop_movement_helper(board : &mut BoardTester, piece_name: &str)
    {
        {
            let move_result = board.try_move("1c", "3e");
            assert!(move_result.is_ok_and(|moved| moved), "Failed to move {piece_name} in diagonal (up right)");
        }
        {
            let move_result = board.try_move("3e", "5c");
            assert!(move_result.is_ok_and(|moved| moved), "Failed to move {piece_name} in diagonal (up left)");
        }
        {
            let move_result = board.try_move("5c", "3e");
            assert!(move_result.is_ok_and(|moved| moved), "Failed to move {piece_name} in diagonal (down right)");
        }
        {
            let move_result = board.try_move("3e", "1c");
            assert!(move_result.is_ok_and(|moved| moved), "Failed to move {piece_name} in diagonal (down left)");
        }
        board.add_pawn("2b", PieceOwnerType::Black);
        {
            let move_result = board.try_move("1c", "3a");
            assert!(move_result.is_ok_and(|moved| !moved), "A {piece_name} shouldn't be able to move accross pieces");
        }
        {
            let move_result = board.try_move("1c", "2b");
            assert!(move_result.is_ok_and(|moved| moved), "A {piece_name} should be able to eat enemy pieces");
        }
    }

    #[test]
    fn bishop_movement()
    {
        let mut board = BoardTester::create();
        board.add_bishop("1c", PieceOwnerType::White);
        bishop_movement_helper(&mut board, "bishop");
    }

    #[test]
    fn queen_movement()
    {
        let mut board = BoardTester::create();
        board.add_queen("1c", PieceOwnerType::White);
        bishop_movement_helper(&mut board, "queen");

        board = BoardTester::create();
        board.add_queen("1a", PieceOwnerType::White);
        rook_movement_helper(&mut board, "queen");
    }

}