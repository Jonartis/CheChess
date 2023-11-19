
use super::error::InputError;
use super::board::BOARD_HEIGTH;

#[derive(Copy, Clone)]
pub struct Location
{
    pub row: usize,
    pub col: usize
}

impl Location
{   
    fn char_to_row(ch : char) -> Result<usize, InputError>
    {
        const RADIX:u32 = 10;//Decimal base
        let row_digit = ch.to_digit(RADIX);
        let row = match row_digit 
        {
            Some(digit) => digit as usize,
            None => return Err(InputError::InvalidInput(std::format!("Row '{ch}' wasn't a digit")))
        };
        if row == 0 || row > BOARD_HEIGTH
        {
            return Err(InputError::InvalidInput(std::format!("Invalid row value '{ch}'")));
        }
        //Convert from board space to indices
        let row_idx = BOARD_HEIGTH - row;
        Ok(row_idx)
    }

    fn char_to_colum(ch : char) -> Result<usize, InputError>
    {
        let col: usize = if ch.is_ascii_alphabetic() && ch.is_ascii_lowercase()
        {
            ch as usize - b'a' as usize
        }
        else
        {
            return Err(InputError::InvalidInput(std::format!("Column wasn't a valid character '{ch}'")))
        };
        Ok(col)
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