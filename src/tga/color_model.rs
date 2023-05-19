#[derive(Copy, Clone)]
pub enum ModelBPP {
    GRAYSCALE = 1,
    RGB = 3,
    RGBA = 4,
}

pub enum DatatypeCode {
    NoImageDataIncluded,
    UncompressedColorMappedImage,
    UncompressedTrueColorImage,
    UncompressedBlackAndWhiteImage,
    RunLengthEncodedColorMappedImage,
    RunLengthEncodedTrueColorImage,
    RunLengthEncodedBlackAndWhiteImage,
}

impl DatatypeCode {
    pub fn into_u8(self) -> u8 {
        match self {
            DatatypeCode::NoImageDataIncluded => 0,
            DatatypeCode::UncompressedColorMappedImage => 1,
            DatatypeCode::UncompressedTrueColorImage => 2,
            DatatypeCode::UncompressedBlackAndWhiteImage => 3,
            DatatypeCode::RunLengthEncodedColorMappedImage => 9,
            DatatypeCode::RunLengthEncodedTrueColorImage => 10,
            DatatypeCode::RunLengthEncodedBlackAndWhiteImage => 11,
        }
    }
}

pub trait ColorModel {
    fn new() -> Self;
    const BYTES_PER_PIXEL: ModelBPP;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Grayscale {
    c: u8,
}

impl Grayscale {
    pub fn set(shade: u8) -> Self {
        Grayscale { c: shade }
    }
}

impl ColorModel for Grayscale {
    fn new() -> Self {
        Grayscale::set(0)
    }
    const BYTES_PER_PIXEL: ModelBPP = ModelBPP::GRAYSCALE;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RGBA {
    b: u8,
    g: u8,
    r: u8,
    a: u8,
}

impl RGBA {
    pub fn set(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        RGBA {
            b: blue,
            g: green,
            r: red,
            a: alpha,
        }
    }
}

impl ColorModel for RGBA {
    fn new() -> Self {
        RGBA::set(0, 0, 0, 0)
    }
    const BYTES_PER_PIXEL: ModelBPP = ModelBPP::RGBA;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RGB {
    b: u8,
    g: u8,
    r: u8,
}

impl RGB {
    pub fn set(red: u8, green: u8, blue: u8) -> Self {
        RGB {
            b: blue,
            g: green,
            r: red,
        }
    }
}

impl ColorModel for RGB {
    fn new() -> Self {
        RGB::set(0, 0, 0)
    }
    const BYTES_PER_PIXEL: ModelBPP = ModelBPP::RGB;
}
