
use super::error::InputError;
use super::board::{BOARD_SIZE, BOARD_SIZE_US};

#[derive(PartialEq)]
pub enum MovementResult
{
    Moved,
    GameFinished,
    PawnUpgrade,
    FailedToMove
}

#[derive(Copy, Clone)]
pub struct Location
{
    pub row: i32,
    pub col: i32
}

//Readonly by design. It can only be created from a Location
#[derive(Copy, Clone)]
pub struct BoardLocation
{
    row: usize,
    col: usize
}

impl Location
{
    fn char_to_row(ch : char) -> Result<i32, InputError>
    {
        const RADIX:u32 = 10;//Decimal base
        let row_digit = ch.to_digit(RADIX);
        let row = match row_digit 
        {
            Some(digit) => digit as i32,
            None => return Err(InputError::InvalidInput(std::format!("Row '{ch}' wasn't a digit")))
        };
        if row == 0 || row > BOARD_SIZE
        {
            return Err(InputError::InvalidInput(std::format!("Invalid row value '{ch}'")));
        }
        //Convert from board space to indices
        let row_idx = BOARD_SIZE - row;
        Ok(row_idx)
    }
    
    fn char_to_colum(ch : char) -> Result<i32, InputError>
    {
        let col: i32 = if ch.is_ascii_alphabetic() && ch.is_ascii_lowercase()
        {
            ch as i32 - b'a' as i32
        }
        else
        {
            return Err(InputError::InvalidInput(std::format!("Column wasn't a valid character '{ch}'")))
        };
        Ok(col)
    }

    pub fn is_valid(&self) -> bool
    {
        let zero = 0;
        let size = BOARD_SIZE;
        self.row >= zero && self.row < size && self.col >= zero && self.col < size
    }

}

impl TryFrom<&str> for Location
{
    type Error = InputError;

    fn try_from(movement: &str) -> Result<Self, Self::Error>
    {
        if movement.len() < 2
        {
            return Err(Self::Error::InvalidInput(std::format!("Length of movement '{}' was {}", movement, movement.len())));
        }
        let row = Location::char_to_row(movement.chars().nth(0).unwrap())?;
        let col = Location::char_to_colum(movement.chars().nth(1).unwrap())?;
        
        Ok(Location { row, col })
    }
}

impl From<BoardLocation> for Location
{
    fn from(value: BoardLocation) -> Self
    {
        //It is safe to unwrapp the usize->i32 conversion as the BoardLocation must be valid
        Self{row: value.get_row().try_into().unwrap(), col: value.get_col().try_into().unwrap()}
    }
}

//BOARD LOCATION IMPL

impl BoardLocation
{
    pub fn get_row(&self) -> usize
    {
        self.row
    }

    pub fn get_col(&self) -> usize
    {
        self.col
    }

    //Private as the BoardLocation must always be valid externally
    fn is_valid(&self) -> bool
    {
        let zero = 0;
        let size = BOARD_SIZE_US;
        self.row >= zero && self.row < size && self.col >= zero && self.col < size
    }

    pub fn try_create(row : usize, col : usize) -> Result<Self, ()>
    {
        let loc = Self{row, col};

        if loc.is_valid()
        {
            Ok(loc)
        }
        else
        {
            Err(())    
        }
    }
}

impl TryFrom<Location> for BoardLocation
{
    type Error = ();

    fn try_from(value: Location) -> Result<Self, Self::Error>
    {
        if value.is_valid()
        {
            //It is safe to unwrapp if the Location is valid
            Ok(Self{row: value.row.try_into().unwrap(), col: value.col.try_into().unwrap()})
        }
        else
        {
            Err(())    
        }
    }
}