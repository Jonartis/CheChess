

#[derive(Debug)]
pub enum MovementError
{
    SourceOutOfBounds,
    SourcePieceNotFound,
    DestinationOutOfBounds
}

#[derive(Debug)]
pub enum InputError
{
    InvalidInput(String)
}

#[derive(Debug)]
pub enum ChessError
{
    MovementError(MovementError),
    InputError(InputError)
}

impl From<MovementError> for ChessError
{
    fn from(value: MovementError) -> Self {
        ChessError::MovementError(value)
    }
}

impl From<InputError> for ChessError
{
    fn from(value: InputError) -> Self {
        ChessError::InputError(value)
    }
}

pub fn print_error(error: ChessError)
{
    eprintln!("Error! {:?}", error);
}