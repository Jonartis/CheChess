
mod pieces;

use math::vector::IVec2;
use pieces::pawn::Pawn;
//mod math;
//use math::IVec2;

fn main()
{
    let my_pawn = Pawn {
        piece: pieces::Piece { pos: IVec2 { x: 1, y: 2 } }
    };
    
    println!("Hello, world! {}", my_pawn.piece.pos.x);
}
