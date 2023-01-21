mod tiles;
use tiles::tiles::{Tile, gen_tiles, gen_image};
fn main(){
    let _ = gen_tiles(0);
    let _ = gen_image(vec![Tile::new()], 1, 1);

    
}