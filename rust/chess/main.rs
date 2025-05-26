
mod game;

fn main()
{
    let mut game = game::Game::default();
    game.init();
    loop {

        game.update();
        if game.should_quit()
        {
            break;
        }
    }
    println!("Quitting...")
}
