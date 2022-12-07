use std::{env::args, fs};


fn main() -> Result<(), String> {
    let args: Vec<String> = args().collect();
    
    if args.len() < 2 {
        return Err("Please supply .bmp as first argument".to_owned())
    }

    let file_path = &args[1];
    let result = fs::read(file_path);

    if let Ok(bytes) = result {
        return process_file(file_path, bytes);
    } else {
        let err = result.unwrap_err();
        return Err(err.to_string());
    }
}

fn process_file(path: &String, bytes: Vec<u8>) -> Result<(), String> {
    let file_length = bytes.len();

    println!("Processing bmp...");

    // Address is little endian, but BitConverter seems to be okay with this
    // May need to add a check to make sure BitConverter.IsLittleEndian is true
    let pixel_data_address = u32::from_le_bytes([
        bytes[0x0a],
        bytes[0x0a+1],
        bytes[0x0a+2],
        bytes[0x0a+3]
    ]) as usize;

    let pixel_data = &bytes[pixel_data_address..file_length];

    let mut reversed_pixel_data: Vec<u8> = Vec::new();
    // Reverse scanlines since bitmap starts with last row and goes backwards
    let mut i = (pixel_data.len() - 128) as i32;
    while i >= 0 {
        for j in 0..128 {
            let data = pixel_data[(i as usize) + j];
            reversed_pixel_data.push(data);
        }

        i -= 128;
    }

    return process_pixel_bytes(path, reversed_pixel_data)
}

fn process_pixel_bytes(path: &String, pixel_data: Vec<u8>) -> Result<(), String> {
    let scanline_count = pixel_data.len() / 128;
    let mut tile_data: Vec<Vec<u8>> = Vec::new();

    // Divide / reorder rows of 128 pixels into 8x8 tiles
    let mut i = 0;
    while i < pixel_data.len() { // Iterate every 8 scanlines

        let mut j = 0;
        while j < 128 { // Then iterate every 8 columns
            let mut tile: Vec<u8> = Vec::new();

            let mut k = 0;
            while k < 1024 { // Starting from there, iterate over 8 8-pixel rows

                for l in 0..8 { // Then each of the 8 pixels in that row
                    tile.push(pixel_data[i + k + j + l]);
                }

                k += 128;
            }

            tile_data.push(tile);
            j += 8;
        }
        i += 128 * 8;
    }
    
    return process_tiles(path, tile_data)
}

fn process_tiles(path: &String, tile_data: Vec<Vec<u8>>) -> Result<(), String> {
    let mut output_data: Vec<u8> = Vec::new();

    for tile in tile_data {
        let mut low_bit_plane: Vec<u8> = Vec::new();
        let mut hi_bit_plane: Vec<u8> = Vec::new();

        for i in 0..8 {
            let mut low_bit_plane_for_row = 0x00;
            let mut hi_bit_plane_for_row = 0x00;

            for j in 0..8 {
                let pixel_byte = tile[(i * 8) + j];

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

    println!("Converting .bmp to .chr complete!");

    Ok(())
}