mod tiles;

use tiles::tiles::{gen_image, gen_tiles};
use image::{Rgb};

use std::env;
use std::str::FromStr;

const USAGE: &str = "
Usage:
    wave_function_collapse --fg_color COLOR --bg_color COLOR -o OUTPUT
    wave_function_collapse -h | --help

Options:
    -h --help               Show this screen.
    --fg_color COLOR        Set the foreground color in hex format (e.g. ff0000 or #ff0000).
    --bg_color COLOR        Set the background color in hex format (e.g. 0000ff or #0000ff).
    -o --output-file OUTPUT Set the output file.
";

#[derive(Debug, Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color{
    fn to_rgb(&self) -> Rgb<u8>{
        Rgb([self.r, self.g, self.b])
    }
}

impl FromStr for Color {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("#") {
            let s = &s[1..];
            if s.len() == 6 {
                let r = u8::from_str_radix(&s[0..2], 16);
                let g = u8::from_str_radix(&s[2..4], 16);
                let b = u8::from_str_radix(&s[4..6], 16);
                match (r, g, b) {
                    (Ok(r), Ok(g), Ok(b)) => Ok(Color { r, g, b }),
                    _ => Err("Invalid color format nums".to_string()),
                }
            } else {
                Err("Invalid color format length".to_string())
            }
        } else {
            let s = &s[..];
            if s.len() == 6 {
                let r = u8::from_str_radix(&s[0..2], 16);
                let g = u8::from_str_radix(&s[2..4], 16);
                let b = u8::from_str_radix(&s[4..6], 16);
                match (r, g, b) {
                    (Ok(r), Ok(g), Ok(b)) => Ok(Color { r, g, b }),
                    _ => Err("Invalid color format nums".to_string()),
                }
            } else {
                Err("Invalid color format length".to_string())
            }
        }
    }
}


fn main(){

    let args: Vec<String> = env::args().collect();

    let mut fg_color = Color { r: 255, g: 255, b: 255 };
    let mut bg_color = Color { r: 0, g: 0, b: 0 };
    let mut output_file = "output.bmp";
    // dbg!(&args);

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                println!("{}", USAGE);
                return;
            }
            "--fg_color" => {
                i += 1;
                if i == args.len(){
                    println!("No fg_color argument, if used \"#\" make sure to enclose the color in string literals");
                    return;
                }
                fg_color = Color::from_str(&args[i]).unwrap();
            }
            "--bg_color" => {
                i += 1;
                if i == args.len(){
                    println!("No bg_color argument, if used \"#\" make sure to enclose the color in string literals");
                    return;
                }
                bg_color = Color::from_str(&args[i]).unwrap();
            }
            "-o" | "--output-file" => {
                i += 1;
                if i == args.len(){
                    println!("No output argument");
                    return;
                }
                output_file = &args[i];
            }
            _ => {
                println!("Invalid argument: {}", args[i]);
                println!("{}", USAGE);
                return;
            }
        }
        i += 1;
    }

    println!("Foreground color: {:?}", fg_color);
    println!("Background color: {:?}", bg_color);
    println!("Output file: {}", output_file);


    println!("Generating tiles...");
    let tiles = gen_tiles(9);
    println!("Tiles ready.");
    let img = gen_image(5, 5, &tiles);
    println!("{}", img);
    let bmp = img.to_image(fg_color.to_rgb(), bg_color.to_rgb());
    bmp.save(output_file).unwrap()
}


    