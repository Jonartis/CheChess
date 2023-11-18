
mod pieces;
mod board;

use std::io;

struct Game
{
    board: board::Board,
    exit_requested : bool 
}

impl Game
{
    fn default() -> Game
    {
        Game
        {
            board: board::Board::default(),
            exit_requested: false
        }
    }

    fn process_command(&mut self, cmd : &str)
    {
        if cmd == "db"
        {
            println!("Board \n{}", self.board);
        }
        else if cmd == "q"
        {
            self.exit_requested = true;
        }
    }

    fn should_quit(&self) -> bool
    {
        self.exit_requested
    }
}

fn main()
{
    let mut game = Game::default();
    
    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read command");

        let command = user_input.trim_end();

        game.process_command(command);
        if game.should_quit()
        {
            break;
        }
    }

}
