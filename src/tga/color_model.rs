#[derive(Copy, Clone)]
pub enum ModelBPP {
    GRAYSCALE = 1,
    RGB = 3,
    RGBA = 4,
}

pub trait ColorModel {
    fn new() -> Self;
    const BYTES_PER_PIXEL: ModelBPP;
}

#[derive(Copy, Clone)]
pub struct Grayscale {
    pub c: u8,
}

impl ColorModel for Grayscale {
    fn new() -> Grayscale {
        Grayscale { c: 0 }
    }
    const BYTES_PER_PIXEL: ModelBPP = ModelBPP::GRAYSCALE;
}

#[derive(Copy, Clone)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ColorModel for RGBA {
    fn new() -> RGBA {
        RGBA {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }
    const BYTES_PER_PIXEL: ModelBPP = ModelBPP::RGBA;
}

#[derive(Copy, Clone)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorModel for RGB {
    fn new() -> RGB {
        RGB { r: 0, g: 0, b: 0 }
    }
    const BYTES_PER_PIXEL: ModelBPP = ModelBPP::RGB;
}
