
mod game;
mod error;

use game::Game;

fn main()
{
    let mut game = Game::default();
    
    loop {

        game.update();
        if game.should_quit()
        {
            break;
        }
    }
    println!("Quitting...")
}
