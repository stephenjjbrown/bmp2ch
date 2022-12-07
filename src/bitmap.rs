const BITMAP_PIXEL_DATA_ADDRESS: usize = 0x0a;
const BITMAP_WIDTH: usize = 0x12;
const BITMAP_HEIGHT: usize = 0x16;
const BITMAP_BPP: usize = 0x1c;
const BITMAP_COMPRESSION: usize = 0x1e;

pub struct Bitmap {
    pub bytes: Vec<u8>
}

impl Bitmap {
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn verify_header(&self) {
        if
            self.len() < 0x10 ||
            self.bytes[0] != 0x42 ||
            self.bytes[1] != 0x4d
        {
            panic!("File is not a valid bitmap");
        }
    }

    pub fn get_width(&self) -> i32 {
        let width: [u8; 4] = self.bytes[BITMAP_WIDTH..BITMAP_WIDTH+4]
            .try_into()
            .expect("Could not get image width. File may have a corrupt or invalid bmp header");
        
        i32::from_le_bytes(width)
    }

    pub fn get_height(&self) -> i32 {
        let height: [u8; 4] = self.bytes[BITMAP_HEIGHT..BITMAP_HEIGHT+4]
            .try_into()
            .expect("Could not get image height. File may have a corrupt or invalid bmp header");
        
        i32::from_le_bytes(height)
    }

    pub fn get_bpp(&self) -> u16 {
        let bpp: [u8; 2] = self.bytes[BITMAP_BPP..BITMAP_BPP+2]
            .try_into()
            .expect("Could not get image bits per pixel. File may have a corrupt or invalid bmp header");

        u16::from_le_bytes(bpp)
    }

    pub fn get_compression_format(&self) -> u32 {
        let format: [u8; 4] = self.bytes[BITMAP_COMPRESSION..BITMAP_COMPRESSION+4]
            .try_into()
            .expect("Could not determine image compression format. File may have a corrupt or invalid bmp header");

        u32::from_le_bytes(format)
    }

    // Get the address where all the file's pixel data is located from the Bitmap header
    pub fn get_pixel_data_address(&self) -> usize {
        let address: [u8; 4] = self.bytes[
            BITMAP_PIXEL_DATA_ADDRESS..BITMAP_PIXEL_DATA_ADDRESS+4
        ]
        .try_into()
        .expect("Could not get pixel data. File may have a corrupt or invalid bmp header");

        // Address is stored in bmp as little endian
        u32::from_le_bytes(address) as usize
    }
}