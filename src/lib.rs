use std::io;
use std::iter;

/// Main bitmap structure
pub struct Bitmap {
    width:  i32,
    height: i32,
    pixels: Vec<u8>,
}

impl Bitmap {
    /// Create a new blank (white) bitmap of a specified size.
    pub fn new(width: i32, height: i32) -> Bitmap {
        Bitmap {
            width: width,
            height: height,
            // Create a vector to store the pixels in, ensuring that it is padded to a multiple of 4
            // bytes of each row.
            pixels: iter::repeat(0xFF).take((height * (width * 3 + width % 4)) as usize).collect(),
        }
    }

    /// Set a pixel at (x, y) to a specified color (r, g, b).
    pub fn set_pixel(&mut self, x: i32, y: i32, color: (u8, u8, u8)) {
        // Calculate the byte offset for x
        let i = ((self.height - y - 1) * (self.width * 3) + x * 3) as usize;

        let (r, g, b) = color;
        // Note: Pixel order for bitmaps is (blue, green, red)
        self.pixels[i + 0] = b;
        self.pixels[i + 1] = g;
        self.pixels[i + 2] = r;
    }

    /// Write the stored data to a file with given filename.
    pub fn write_to_file(&self, filename: &str) -> io::IoResult<()> {
        const FILE_HEADER_SIZE: usize = 14;
        const BMP_INFO_SIZE: usize = 40;
        const TOTAL_HEADER_SIZE: usize = FILE_HEADER_SIZE + BMP_INFO_SIZE;

        let image_size = (self.height * self.width*3 + self.height * (self.width % 4)) as usize;
        let file_size = image_size + TOTAL_HEADER_SIZE;

        // Bitmap file header
        let file_header: [u8; FILE_HEADER_SIZE] = [
            'B' as u8, 'M' as u8,
            file_size as u8, (file_size>>8) as u8, (file_size>>16) as u8, (file_size>>24) as u8,
            0, 0,
            0, 0,
            TOTAL_HEADER_SIZE as u8, 0, 0, 0
        ];
        // Bitmap information header
        let info_header: [u8; BMP_INFO_SIZE] = [
            BMP_INFO_SIZE as u8, 0, 0, 0,
            self.width as u8, (self.width>>8) as u8, (self.width>>16) as u8, (self.width>>24) as u8,
            self.height as u8, (self.height>>8) as u8, (self.height>>16) as u8, (self.height>>24) as u8,
            1, 0,
            24, 0,
            0, 0, 0, 0,
            image_size as u8, (image_size>>8) as u8, (image_size>>16) as u8, (image_size>>24) as u8,
            72, 0, 0, 0,
            72, 0, 0, 0,
            0, 0, 0, 0,
            0, 0, 0, 0
        ];

        // Set up the file writer
        let mut file = io::File::create(&Path::new(filename));

        // Write the bitmap headers to file
        try!(file.write(&file_header));
        try!(file.write(&info_header));

        // Write data to file
        file.write(&*self.pixels)
    }

    /// Get the width of the bitmap.
    pub fn width(&self) -> i32 {
        self.width
    }

    /// Get the height of the bitmap.
    pub fn height(&self) -> i32 {
        self.height
    }
}
