use std::{env::args, fs};
use bitmap::Bitmap;

pub mod bitmap;

const TILE_SIZE: usize = 8;
const ROW_WIDTH: usize = 128;

fn main() -> Result<(), String> {
    let args: Vec<String> = args().collect();
    
    if args.len() < 2 {
        return Err(String::from("Please supply .bmp as first argument"));
    }

    let file_path = &args[1];
    let result = fs::read(file_path);

    if let Ok(bytes) = result {
        let bitmap = Bitmap { bytes };
        return process_file(file_path, bitmap);
    } else {
        let err = result.unwrap_err();
        return Err(err.to_string());
    }
}


fn process_file(path: &String, bitmap: Bitmap) -> Result<(), String> {
    bitmap.verify_header();

    if bitmap.get_width() != (ROW_WIDTH as i32) {
        return Err(String::from(format!("Image must be {} pixels wide", ROW_WIDTH)));
    }
    if bitmap.get_height() % (TILE_SIZE as i32) != 0 {
        return Err(String::from(format!("Image height must be a multiple of {} pixels", TILE_SIZE)));
    }
    if bitmap.get_bpp() > 8 {
        return Err(String::from(
            "Image must be in indexed color mode and have 256 colors or fewer. (< 8bpp)"
        ));
    }
    if bitmap.get_compression_format() != 0 {
        return Err(String::from("Image must be a standard uncompressed bitmap format"));
    }

    let file_length = bitmap.len();

    print!("Converting bmp to chr...");

    let pixel_data_address = bitmap.get_pixel_data_address();
    let pixel_data = &bitmap.bytes[pixel_data_address..file_length];

    let mut reversed_pixel_data: Vec<u8> = Vec::new();
    // Reverse scanlines since bitmap starts with last row and goes backwards
    // TODO: this step is probably unnecessary and could be accounted for in the later loop
    let mut i = (pixel_data.len() - ROW_WIDTH) as i32;
    while i >= 0 {
        for j in 0..ROW_WIDTH {
            let data = pixel_data[(i as usize) + j];
            reversed_pixel_data.push(data);
        }

        i -= 128;
    }

    process_pixel_bytes(path, reversed_pixel_data)
}

fn process_pixel_bytes(path: &String, pixel_data: Vec<u8>) -> Result<(), String> {
    //let scanline_count = pixel_data.len() / 128;
    let mut tile_data: Vec<Vec<u8>> = Vec::new();

    // Divide / reorder rows of 128 pixels into 8x8 tiles
    let mut i = 0;
    while i < pixel_data.len() { // Iterate every 8 scanlines

        let mut j = 0;
        while j < ROW_WIDTH { // Then iterate every 8 columns
            let mut tile: Vec<u8> = Vec::new();

            let mut k = 0;
            while k < (ROW_WIDTH * TILE_SIZE) { // Starting from there, iterate over 8 8-pixel rows

                for l in 0..TILE_SIZE { // Then each of the 8 pixels in that row
                    tile.push(pixel_data[i + k + j + l]);
                }

                k += ROW_WIDTH;
            }

            tile_data.push(tile);
            j += TILE_SIZE;
        }
        i += ROW_WIDTH * TILE_SIZE;
    }
    
    process_tiles(path, tile_data)
}

fn process_tiles(path: &String, tile_data: Vec<Vec<u8>>) -> Result<(), String> {
    let mut output_data: Vec<u8> = Vec::new();

    for tile in tile_data {
        let mut low_bit_plane: Vec<u8> = Vec::new();
        let mut hi_bit_plane: Vec<u8> = Vec::new();

        for i in 0..TILE_SIZE {
            let mut low_bit_plane_for_row = 0x00;
            let mut hi_bit_plane_for_row = 0x00;

            for j in 0..TILE_SIZE {
                let pixel_byte = tile[(i * TILE_SIZE) + j];

                let mut low_bit = pixel_byte & 0b0000_0001;
                let mut hi_bit = (pixel_byte & 0b0000_0010) >> 1;

                low_bit = low_bit << (7 - j);
                hi_bit = hi_bit << (7 - j);

                low_bit_plane_for_row |= low_bit as u8;
                hi_bit_plane_for_row |= hi_bit as u8;
            }

            low_bit_plane.push(low_bit_plane_for_row);
            hi_bit_plane.push(hi_bit_plane_for_row);
        }

        output_data.append(&mut low_bit_plane);
        output_data.append(&mut hi_bit_plane);
    }

    if let Err(err) = fs::write(path.replace(".bmp", ".chr"), output_data) {
        return Err(err.to_string());
    }

    println!("complete!");

    Ok(())
}