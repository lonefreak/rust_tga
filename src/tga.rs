use std::clone::Clone;
use std::fs::File;
use std::io::Write;

pub mod color_model;
pub mod spec;
use self::color_model::{ColorModel, DatatypeCode, ModelBPP};
use self::spec::{TgaFooter, TgaHeader};

// implementation goal
// https://github.com/ssloy/tinyrenderer/blob/909fe20934ba5334144d2c748805690a1fa4c89f/tgaimage.cpp
// bool TGAImage::read_tga_file(const char *filename)
// bool TGAImage::load_rle_data(std::ifstream &in)
// bool TGAImage::unload_rle_data(std::ofstream &out)
// bool TGAImage::write_tga_file(const char *filename, bool rle) - missing the rle part
// unsigned char *TGAImage::buffer() - returns the data
// bool TGAImage::scale(int w, int h)
// void TGAImage::clear()

pub struct TgaImage<T: ColorModel> {
    width: u16,
    height: u16,
    bytespp: ModelBPP,
    data: Vec<T>,
}

impl<T: ColorModel + Clone> TgaImage<T> {
    pub fn new(width: u16, height: u16) -> TgaImage<T> {
        TgaImage {
            width,
            height,
            bytespp: T::BYTES_PER_PIXEL,
            data: vec![T::new(); (width * height) as usize],
        }
    }

    pub fn get_width(&self) -> u16 {
        self.width
    }

    pub fn get_height(&self) -> u16 {
        self.height
    }

    pub fn get_bytespp(&self) -> u8 {
        self.bytespp as u8
    }

    pub fn set(&mut self, x: u16, y: u16, color: T) -> Result<(), String> {
        if x >= self.height || y >= self.width {
            return Err(format!(
                "x or y out of bounds: x={}, y={}, width={}, height={}",
                x, y, self.width, self.height
            ));
        }
        self.data[(y + x * self.width) as usize] = color;
        Ok(())
    }

    pub fn get(&self, x: u16, y: u16) -> Result<T, String> {
        if x >= self.height || y >= self.width {
            return Err(format!(
                "x or y out of bounds: x={}, y={}, width={}, height={}",
                x, y, self.width, self.height
            ));
        }
        Ok(self.data[(y + x * self.width) as usize].clone())
    }

    pub fn write_tga_file(&self, filename: &str) -> Result<(), String> {
        let mut file = File::create(filename).map_err(|e| e.to_string())?;

        // Write header
        let mut tga_header = TgaHeader::new();
        tga_header.datatypecode = match self.bytespp {
            ModelBPP::GRAYSCALE => DatatypeCode::UncompressedBlackAndWhiteImage.into_u8(),
            ModelBPP::RGB => DatatypeCode::UncompressedTrueColorImage.into_u8(),
            ModelBPP::RGBA => DatatypeCode::UncompressedTrueColorImage.into_u8(),
        };
        tga_header.width = self.width;
        tga_header.height = self.height;
        tga_header.bitsperpixel = self.get_bytespp() << 3;
        tga_header.write(&mut file)?;

        // Write data
        for i in 0..self.data.len() {
            let color = self.data[i].clone();
            let bytes = unsafe { self.any_as_u8_slice(&color) };
            file.write_all(bytes).map_err(|e| e.to_string())?;
        }

        // Write footer
        let tga_footer = TgaFooter::new();
        tga_footer.write(&mut file)?;

        Ok(())
    }

    pub fn flip_horizontally(&mut self) -> Result<(), String> {
        if self.data.len() == 0 {
            return Err(format!("No image data found"));
        }
        let half = self.height >> 1; // height / 2 ^1
        for x in 0..half {
            for y in 0..self.width {
                let color1: T = self.get(x, y).unwrap();
                let color2: T = self.get(self.height - 1 - x, y).unwrap();
                self.set(x, y, color2)?;
                self.set(self.height - 1 - x, y, color1)?;
            }
        }
        Ok(())
    }

    pub fn flip_vertically(&mut self) -> Result<(), String> {
        if self.data.len() == 0 {
            return Err(format!("No image data found"));
        }
        let half = self.width >> 1; // width / 2 ^1
        for x in 0..self.height {
            for y in 0..half {
                println!("{},{} => {},{}", x, y, x, self.width - 1 - y);
                let color1: T = self.get(x, y).unwrap();
                let color2: T = self.get(x, self.width - 1 - y).unwrap();
                self.set(x, y, color2)?;
                self.set(x, self.width - 1 - y, color1)?;
            }
        }
        Ok(())
    }

    // reference
    // https://stackoverflow.com/questions/31281201/how-to-convert-a-struct-into-a-u8-slice
    unsafe fn any_as_u8_slice<W: Sized>(&self, p: &W) -> &[u8] {
        ::core::slice::from_raw_parts((p as *const W) as *const u8, ::core::mem::size_of::<W>())
    }
}
