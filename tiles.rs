// use rand::thread_rng;
// use rand::Rng;


pub mod tiles{
    #[derive(Debug)]
    #[derive(PartialEq)]
    #[derive(Clone)]
    struct TileSelection(Vec<Tile>, bool);
    #[derive(Debug, Clone, PartialEq)]
    struct TilePossibilities(usize, 
        // l , r, u, d
        Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>);

    #[derive(Debug)]
    #[derive(PartialEq)]
    #[derive(Clone, Copy)]
    pub struct Tile{
        // map[x][y]
        map: [[bool;3];3]
    }

    impl Tile{
        pub fn new() -> Tile
        {
            let mut rng = rand::thread_rng();
            let mut map: [[bool;3];3] = [[false; 3];3];
            // Gen random Tile
            for i in 0..3{
                for j in 0..3{
                    map[i][j] = rand::Rng::gen(&mut rng);
                }
            }
            Tile{
                map: map
            }
        }
    }


pub fn gen_image(tiles: Vec<Tile>, size_x: u16, size_y: u16) -> Vec<Vec<Tile>>
{
    
    let mut _tmp: Vec<Vec<TileSelection>> = vec![vec![TileSelection(tiles.clone(), false);size_x as usize]; size_y as usize];

    // let _changable: Vec<TilePossibilities> = {

    //     let mut change: Vec<TilePossibilities>= vec![];
    //     for i in 0..tiles.len(){
    //         let mut tp = TilePossibilities(i, vec![], vec![], vec![], vec![]);
    //         let tile: Tile = tiles[i];
    //         // check where can be added
    //         for (ind, t) in tiles.iter().enumerate()
    //         {
    //             if t.map[0][0] == tile.map[2][0] && t.map[0][1] == tile.map[2][1] && t.map[0][2] == tile.map[2][2] {
    //                 // left
    //                 tp.1.push(ind);
    //             }
    //             if t.map[2][0] == tile.map[0][0] && t.map[2][1] == tile.map[0][1] && t.map[2][2] == tile.map[0][2] {
    //                 // right
    //                 tp.2.push(ind);
    //             }
    //             if t.map[0][2] == tile.map[0][0] && t.map[1][2] == tile.map[1][0] && t.map[2][2] == tile.map[2][0] {
    //                 // up
    //                 tp.3.push(ind);
    //             }
    //             if t.map[0][0] == tile.map[0][2] && t.map[1][0] == tile.map[1][2] && t.map[2][0] == tile.map[2][2] {
    //                 // down
    //                 tp.4.push(ind);
    //             }
                
                
    //         }

    //         change.push(tp);
    //     }
    //     change
    // };

    
    

    return vec![];
}

// number - number of elements in returned vector
pub fn gen_tiles(mut number: u8 ) -> Vec<Tile>
{
    let mut tmp: Vec<Tile> = vec![];

    if number < 1 {return tmp;}
    tmp.push(Tile::new());
    number -= 1;
    // generate {number} candidates
    for _ in 0..number
    {
        // repeat until found satisfying candidate
        loop{
            // generate candidate
            let candidate = Tile::new();
            let mut found = false;
            
            for i in 0..tmp.len(){
                if tmp[i] == candidate{
                    found= true;
                }
            }
            // if candidate reapeted repeat the loop
            if found{
                continue
            }
            // if new candidate is unique push it to the vector and exit loop
            else{
                tmp.push(candidate);
                break;
            }
        }
    }

    tmp
}

}
