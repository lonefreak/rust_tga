use std::clone::Clone;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::mem::MaybeUninit;

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
    header: TgaHeader,
    footer: TgaFooter,
}

impl<T: ColorModel + Clone> TgaImage<T> {
    pub fn new(width: u16, height: u16) -> TgaImage<T> {
        TgaImage {
            width,
            height,
            bytespp: T::BYTES_PER_PIXEL,
            data: vec![T::new(); (width * height) as usize],
            header: TgaHeader::new(),
            footer: TgaFooter::new(),
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

    pub fn set_width(&mut self, width: u16) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: u16) {
        self.height = height;
    }

    pub fn set_bytespp(&mut self, bytespp: ModelBPP) {
        self.bytespp = bytespp;
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

    pub fn write_tga_file(&mut self, filename: &str) -> Result<(), String> {
        let mut file = File::create(filename).map_err(|e| e.to_string())?;

        // Write header
        self.header.datatypecode = match self.bytespp {
            ModelBPP::GRAYSCALE => DatatypeCode::UncompressedBlackAndWhiteImage.into_u8(),
            ModelBPP::RGB => DatatypeCode::UncompressedTrueColorImage.into_u8(),
            ModelBPP::RGBA => DatatypeCode::UncompressedTrueColorImage.into_u8(),
        };
        self.header.width = self.width;
        self.header.height = self.height;
        self.header.bitsperpixel = self.get_bytespp() << 3;
        self.header.write(&mut file)?;

        // Write data
        self.write_data(&mut file)?;

        // Write footer
        self.footer.write(&mut file)?;

        Ok(())
    }

    fn write_data(&mut self, file: &mut File) -> Result<(), String> {
        for i in 0..self.data.len() {
            let color = self.data[i].clone();
            let bytes = unsafe { self.any_as_u8_slice(&color) };
            file.write_all(bytes).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn read_tga_file(&mut self, filename: &str) -> Result<(), String> {
        let file: File = File::open(filename).map_err(|e| e.to_string())?;
        let mut reader = BufReader::new(&file);
        let mut buffer: Vec<u8> = Vec::new();
        let mut offset = 0;
        reader.read_to_end(&mut buffer).map_err(|e| e.to_string())?;

        let mut image: TgaImage<T> = TgaImage::new(0, 0);
        // Read footer
        image.footer.read(&buffer)?;

        // Read header
        image.header.read(&buffer, &mut offset)?;

        // Read data
        self.width = image.header.width;
        self.height = image.header.height;
        self.bytespp = match image.header.bitsperpixel {
            8 => ModelBPP::GRAYSCALE,
            24 => ModelBPP::RGB,
            32 => ModelBPP::RGBA,
            _ => return Err(format!("Unsupported bpp: {}", image.header.bitsperpixel)),
        };

        self.data = vec![T::new(); (self.width * self.height) as usize];
        match image.header.datatypecode {
            2 | 3 => self.load_uncompressed_tga(&buffer, &mut offset)?,
            _ => {
                return Err(format!(
                    "Unsupported data type: {}",
                    image.header.datatypecode
                ))
            }
        }

        //Check image descriptor
        match image.header.imagedescriptor {
            0x00 => self.flip_vertically(),
            0x10 => {
                self.flip_vertically()?;
                self.flip_horizontally()
            }
            0x20 => Ok(()),
            0x30 => self.flip_horizontally(),
            _ => {
                return Err(format!(
                    "Unsupported image descriptor: {}",
                    image.header.imagedescriptor
                ))
            }
        }
    }

    fn load_uncompressed_tga(
        &mut self,
        buffer: &Vec<u8>,
        offset: &mut usize,
    ) -> Result<(), String> {
        for i in 0..self.data.len() {
            let from = *offset;
            let to = (*offset + self.bytespp as usize) - 1;
            let color = unsafe { self.u8_slice_as_any(&buffer[from..=to].to_vec()) };
            self.data[i] = color;
            *offset += self.bytespp as usize;
        }
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

    unsafe fn u8_slice_as_any<W: Sized>(&self, slice: &[u8]) -> W {
        assert_eq!(::core::mem::size_of::<W>(), slice.len());
        let mut out: W = MaybeUninit::uninit().assume_init();
        ::core::ptr::copy_nonoverlapping(
            slice.as_ptr(),
            &mut out as *mut W as *mut u8,
            slice.len(),
        );
        out
    }
}
