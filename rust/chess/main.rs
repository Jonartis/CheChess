
mod game;
mod error;

use std::io;
use game::Game;

fn main()
{
    let mut game = Game::default();
    
    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read command");

        let command = user_input.trim_end();

        game.update(command);
        if game.should_quit()
        {
            break;
        }
    }
    println!("Quitting...")
}
