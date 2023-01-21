mod tiles;
// use tiles::Tiles::{Tile};


fn main(){
    let tiles = tiles::tiles::gen_tiles(2);
    for i in tiles{
        println!("{}", i)
    }
    // let _ = gen_tiles(0);
    // let _ = gen_image(vec![Tile::new()], 1, 1);

    
}