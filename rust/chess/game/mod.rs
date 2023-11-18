
mod board;
use board::Board;

pub struct Game
{
    board: Board,
    exit_requested : bool 
}

impl Game
{
    pub fn default() -> Game
    {
        Game
        {
            board: Board::default(),
            exit_requested: false
        }
    }

    pub fn process_command(&mut self, cmd : &str)
    {
        clearscreen::clear().expect("failed to clear screen");
        if cmd == "db"
        {
            println!("Board \n{}", self.board);
        }
        else if cmd == "q"
        {
            self.exit_requested = true;
        }
    }

    pub fn should_quit(&self) -> bool
    {
        self.exit_requested
    }
}