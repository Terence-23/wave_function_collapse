pub mod tiles{
    use std::{fmt, error::Error};
    use rand::Rng;

    #[allow(dead_code)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct TileCanvas{
        vec: Vec<Vec<Tile>>
    }
    #[allow(dead_code)]
    impl TileCanvas{
        pub fn new(x:usize, y:usize) ->TileCanvas{
            TileCanvas {vec: vec![vec![Tile{ready:false, map:[[false ; 3]; 3]}; x]; y]}
        }
        pub fn fits_at_position(&self, tile:&Tile, pos:&Point2) -> bool{
            let mut fits = self.vec[pos.y][pos.x].ready == false;
            fits = fits && tile.fits_to_left(&self.vec[pos.y][pos.x-1]);
            fits = fits && tile.fits_to_right(&self.vec[pos.y][pos.x+1]);
            fits = fits && tile.fits_to_bottom(&self.vec[pos.y -1][pos.x]);
            fits = fits && tile.fits_to_top(&self.vec[pos.y+1][pos.x]);

            fits 
        }
        
        fn which_fit_at_position(&self, tiles:&Vec<Tile>, pos:&Point2) -> Vec<usize>{
            let mut result = vec![];
            let size = tiles.len();

            for i in 0..size{
                if self.fits_at_position(&tiles[i], pos){
                    result.push(i);
                }
            }

            return result
        }

        pub fn fill_position(&mut self, tiles:Vec<Tile>) -> Result<(), InsertionError>{

            let mut min_fits = tiles.len();
            let mut where_min = Point2{x:0, y:0};
            let mut what_min: Vec<usize> = vec![];

            let canvas_x = self.vec[0].len();
            let canvas_y = self.vec.len();


            for y in 0..canvas_y{

                for x in 0..canvas_x{
                    let pos = Point2{x:x, y:y};

                    let fits_index = self.which_fit_at_position(&tiles, &pos);
                    if fits_index.len()  < min_fits{
                        where_min = pos;
                        min_fits = fits_index.len();
                        what_min = fits_index;
                    }
                }
            }
    
            if what_min == vec![]{
                return Err(InsertionError::new("No places to insert found"));
            }

            let mut rng = rand::thread_rng();
            let r_index = rng.gen_range(0..what_min.len());

            self.vec[where_min.y][where_min.x] = tiles[what_min[r_index]];

            Ok(())
        }
    }


    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct Point2{
        x:usize, y:usize
    }

    #[derive(PartialEq, Eq, Debug, Clone, Copy)]
    pub struct Tile{
        ready: bool,
        map: [[bool;3];3]
    }


    impl Tile{
        pub fn fits_to_left(&self, other:&Tile) -> bool{
            if !other.ready{true}
            else{
                self.map[0][2] == other.map[0][0] && 
                self.map[1][2] == other.map[1][0] && 
                self.map[2][2] == other.map[2][0]
            }
        }

        pub fn fits_to_right(&self, other:&Tile) -> bool{
            if !other.ready{true}
            else{
                self.map[0][0] == other.map[0][2] && 
                self.map[1][0] == other.map[1][2] && 
                self.map[2][0] == other.map[2][2]
            }
        }

        pub fn fits_to_top(&self, other:&Tile) -> bool{
            if !other.ready{true}
            else{
                self.map[0] == other.map[2]
            }
        }

        pub fn fits_to_bottom(&self, other:&Tile) -> bool{
            if !other.ready{true}
            else{
                self.map[2] == other.map[0]
            }
        }

    }


    impl std::fmt::Display for Tile{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for i in 0..3 {
                write!(f, "[")?;
                for j in 0..3 {
                    write!(f, "{}", self.map[i][j])?;
                    if j != 2 {
                        write!(f, ",")?;
                    }
                }
                write!(f, "]")?;
                if i != 2 {
                    write!(f, "\n")?;
                }
            }
            Ok(())
        }
    }

    impl<'a> PartialEq<&'a Tile> for Tile {
        fn eq(&self, other: &&'a Tile) -> bool {
            self == *other
        }
    }
    
    impl<'a> PartialEq<Tile> for &'a Tile {
        fn eq(&self, other: &Tile) -> bool {
            *self == other
        }
    }

    pub fn gen_tile() -> Tile{

        let mut _rng = rand::thread_rng();
        let table: [[bool;3];3] = _rng.gen();
        return Tile{ready:true, map:table};
    }

    fn gen_unique_tile(tiles:&Vec<Tile>) -> Tile{
        loop{
            
            let mut found = true;
            let candidate = gen_tile();
            for i in tiles{
                if i == candidate{
                    found = true;
                    break;
                }
            }
            if !found{
                return candidate;
            }
        }
    }

    pub fn gen_tiles(n: u8) -> Vec<Tile>{
        let mut tiles = vec![];
        for _ in 0..n{
            tiles.push(gen_unique_tile(&tiles));
        }

        return tiles;
    }


    #[derive(Debug)]
    pub struct InsertionError {
        details: String
    }

    impl InsertionError {
        fn new(msg: &str) -> InsertionError {
            InsertionError { details: msg.to_string() }
        }
    }

    impl fmt::Display for InsertionError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "InsertionError: {}", self.details)
        }
    }

    impl Error for InsertionError {
        fn description(&self) -> &str {
            &self.details
        }
    }

}
