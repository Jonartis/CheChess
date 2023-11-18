
mod pieces;
mod board;

fn main()
{
    let my_board = board::Board::default();
    
    println!("Board \n{}", my_board);
}
