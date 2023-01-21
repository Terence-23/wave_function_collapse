mod tiles;

use tiles::tiles::{gen_image, gen_tiles};
use image::{ImageBuffer, Rgb};

const COLOR_FULL: Rgb<u8> = Rgb([255,255,255]);
const COLOR_EMPTY: Rgb<u8> = Rgb([255, 186, 234]);


fn main(){
    println!("Generating tiles...");
    let tiles = gen_tiles(9);
    println!("Tiles ready.");
    let img = gen_image(2, 2, &tiles);
    println!("{}", img);
    let bmp = create_bmp_from_array(&img.to_array());
    bmp.save("image.bmp").unwrap()
}

fn create_bmp_from_array(array: &Vec<Vec<bool>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut img = ImageBuffer::new(array[0].len() as u32, array.len() as u32);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let color = if array[y as usize][x as usize] {
            COLOR_FULL
        } else {
            COLOR_EMPTY
        };
        *pixel = color;
    }

    img
}
    