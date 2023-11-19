

#[derive(Debug)]
pub enum MovementError
{
    SourceOutOfBounds,
    DestinationOutOfBounds,
}

#[derive(Debug)]
pub enum InputError
{
    InvalidInput(String)
}