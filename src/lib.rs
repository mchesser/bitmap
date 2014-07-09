#![crate_name = "bitmap"]
#![crate_type = "lib"]

use std::io;

/// Main bitmap structure
pub struct Bitmap {
    // The width of the bitmap in pixels, as an unsigned integer (4 bytes).
    width:  i32,

    // The height of the bitmap in pixels, as an unsigned integer (4 bytes).
    height: i32,

    // The pixel array, each row of the array is padded to 4 bytes in size.
    pixels: Vec<u8>,
}

impl Bitmap {
    /// Create a new blank (white) bitmap of a specified size.
    ///
    /// # Arguments
    /// `width` - The width of the bitmap.
    /// `height` - The height of the bitmap.
    pub fn new(width: i32, height: i32) -> Bitmap {
        Bitmap {
            width:  width,
            height: height,

            // Create a vector to store the pixels in, ensuring that it is padded to a multiple of 4
            // bytes of each row.
            pixels: Vec::from_elem((height * (width * 3 + width % 4)) as uint, 0xFFu8)
        }
    }

    /// Set a pixel at (x, y) to a specified color.
    ///
    /// # Arguments
    /// `x` - The x coordinate of the pixel.
    /// `y` - The y coordinate of the pixel.
    /// `color` - color values in the form (r, g, b).
    pub fn set_pixel(&mut self, x: i32, y: i32, color: (u8, u8, u8)) {
        // Calculate the byte offset for x
        let i = ((self.height - y - 1) * (self.width * 3) + x * 3) as uint;

        let (r, g, b) = color;
        // Note: Pixel order for bitmaps is (blue, green, red)
        *self.pixels.get_mut(i + 0) = b;
        *self.pixels.get_mut(i + 1) = g;
        *self.pixels.get_mut(i + 2) = r;
    }

    /// Write the stored data to a file with given filename.
    ///
    /// # Arguments
    /// `filename` - The filename of the target file.
    pub fn write_to_file(&self, filename: &str) -> io::IoResult<()> {
        static FILE_HEADER_SIZE:  u32 = 14;
        static BMP_INFO_SIZE:     u32 = 40;
        static TOTAL_HEADER_SIZE: u32 = FILE_HEADER_SIZE + BMP_INFO_SIZE;

        let image_size = (self.height * self.width*3 + self.height * (self.width % 4)) as u32;
        let file_size = image_size + TOTAL_HEADER_SIZE;

        // Bitmap file header
        let file_header: [u8, ..FILE_HEADER_SIZE] = [
            'B' as u8, 'M' as u8,
            file_size as u8, (file_size>>8) as u8, (file_size>>16) as u8, (file_size>>24) as u8,
            0, 0,
            0, 0,
            TOTAL_HEADER_SIZE as u8, 0, 0, 0
        ];
        // Bitmap information header
        let info_header: [u8, ..BMP_INFO_SIZE] = [
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
        try!(file.write(file_header));
        try!(file.write(info_header));

        // Write data to file
        file.write(self.pixels.as_slice())
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