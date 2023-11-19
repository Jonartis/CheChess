
mod game;

fn main()
{
    let mut game = game::Game::default();
    
    loop {

        game.update();
        if game.should_quit()
        {
            break;
        }
    }
    println!("Quitting...")
}
