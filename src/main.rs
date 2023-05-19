use chrono::Utc;
use rand::Rng;
use rust_tga::tga::{color_model::Grayscale, color_model::RGB, color_model::RGBA, TgaImage};

const NUMBER_OF_RANDOM_PIXELS: u16 = 500;
fn main() {
    let timestamp: String = Utc::now().format("%Y%m%d%H%M%S").to_string();

    make_rgb_heart(&timestamp);
    make_rgba_heart(&timestamp);
    make_grayscale_heart(&timestamp);
    make_random_rgb(&timestamp);
    make_random_rgba(&timestamp);
    make_random_grayscale(&timestamp);
    read_and_write_arrow(&timestamp);
}

fn read_and_write_arrow(timestamp: &String) {
    let mut arrow: TgaImage<RGB> = TgaImage::new(0, 0);
    match arrow.read_tga_file("./input/arrow.tga") {
        Ok(_) => println!("Successfully read heart file!"),
        Err(e) => println!("Error: {}", e),
    }
    let filename = format!("./out/output-arrow-{}.tga", &timestamp);
    match arrow.write_tga_file(&filename) {
        Ok(_) => println!("Successfully wrote arrow file!"),
        Err(e) => println!("Error: {}", e),
    }
}

fn make_rgb_heart(timestamp: &String) {
    let mut heart: TgaImage<RGB> = TgaImage::new(15, 15);
    match draw_heart(&mut heart) {
        Ok(_) => println!("Successfully drew heart!"),
        Err(e) => println!("Error: {}", e),
    }
    let filename = format!("./out/output-heart-{}.tga", &timestamp);
    match heart.write_tga_file(&filename) {
        Ok(_) => println!("Successfully wrote heart file!"),
        Err(e) => println!("Error: {}", e),
    }
}

fn make_rgba_heart(timestamp: &String) {
    let mut heart: TgaImage<RGBA> = TgaImage::new(15, 15);
    match draw_rgba_heart(&mut heart) {
        Ok(_) => println!("Successfully drew rgba heart!"),
        Err(e) => println!("Error: {}", e),
    }
    let filename = format!("./out/output-heart-rgba-{}.tga", &timestamp);
    match heart.write_tga_file(&filename) {
        Ok(_) => println!("Successfully wrote rgba heart file!"),
        Err(e) => println!("Error: {}", e),
    }
}

fn make_grayscale_heart(timestamp: &String) {
    let mut heart: TgaImage<Grayscale> = TgaImage::new(15, 15);
    match draw_grayscale_heart(&mut heart) {
        Ok(_) => println!("Successfully drew grayscale heart!"),
        Err(e) => println!("Error: {}", e),
    }
    let filename = format!("./out/output-heart-grayscale-{}.tga", &timestamp);
    match heart.write_tga_file(&filename) {
        Ok(_) => println!("Successfully wrote grayscale heart file!"),
        Err(e) => println!("Error: {}", e),
    }
}

fn make_random_rgb(timestamp: &String) {
    let mut rgb_image: TgaImage<RGB> = TgaImage::new(100, 100);
    for _ in 0..NUMBER_OF_RANDOM_PIXELS {
        match generate_random_rgb_pixel(&mut rgb_image) {
            Ok(_) => println!("Successfully wrote random rgb pixel!"),
            Err(e) => println!("Error: {}", e),
        }
    }

    let filename = format!("./out/output-rgb-{}.tga", &timestamp);
    match rgb_image.write_tga_file(&filename) {
        Ok(_) => println!("Successfully wrote rgb file!"),
        Err(e) => println!("Error: {}", e),
    }
}

fn make_random_rgba(timestamp: &String) {
    let mut rgba_image: TgaImage<RGBA> = TgaImage::new(100, 100);
    for _ in 0..NUMBER_OF_RANDOM_PIXELS {
        match generate_random_rgba_pixel(&mut rgba_image) {
            Ok(_) => println!("Successfully wrote random rgba pixel!"),
            Err(e) => println!("Error: {}", e),
        }
    }

    let filename = format!("./out/output-rgba-{}.tga", &timestamp);
    match rgba_image.write_tga_file(&filename) {
        Ok(_) => println!("Successfully wrote rgba file!"),
        Err(e) => println!("Error: {}", e),
    }
}

fn make_random_grayscale(timestamp: &String) {
    let mut grayscale_image: TgaImage<Grayscale> = TgaImage::new(100, 100);
    for _ in 0..NUMBER_OF_RANDOM_PIXELS {
        match generate_random_grayscale_pixel(&mut grayscale_image) {
            Ok(_) => println!("Successfully wrote random grayscale pixel!"),
            Err(e) => println!("Error: {}", e),
        }
    }

    let filename = format!("./out/output-grayscale-{}.tga", &timestamp);
    match grayscale_image.write_tga_file(&filename) {
        Ok(_) => println!("Successfully wrote grayscale file!"),
        Err(e) => println!("Error: {}", e),
    }
}
struct RGBPixels {}

impl RGBPixels {
    fn new() -> Self {
        RGBPixels {}
    }

    fn w(&self) -> RGB {
        RGB {
            r: 255,
            g: 255,
            b: 255,
        }
    }

    fn r(&self) -> RGB {
        RGB { r: 0, g: 0, b: 255 }
    }

    fn b(&self) -> RGB {
        RGB { r: 0, g: 0, b: 0 }
    }

    fn d(&self) -> RGB {
        RGB { r: 0, g: 0, b: 139 }
    }
}

struct RGBAPixels {}

impl RGBAPixels {
    fn new() -> Self {
        RGBAPixels {}
    }

    fn w(&self) -> RGBA {
        RGBA {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }

    fn r(&self) -> RGBA {
        RGBA {
            r: 0,
            g: 0,
            b: 255,
            a: 130,
        }
    }

    fn b(&self) -> RGBA {
        RGBA {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }

    fn d(&self) -> RGBA {
        RGBA {
            r: 0,
            g: 0,
            b: 139,
            a: 200,
        }
    }
}

struct GrayscalePixels {}

impl GrayscalePixels {
    fn new() -> Self {
        GrayscalePixels {}
    }

    fn w(&self) -> Grayscale {
        Grayscale { c: 255 }
    }

    fn r(&self) -> Grayscale {
        Grayscale { c: 155 }
    }

    fn b(&self) -> Grayscale {
        Grayscale { c: 0 }
    }

    fn d(&self) -> Grayscale {
        Grayscale { c: 95 }
    }
}

fn generate_random_grayscale_pixel(image: &mut TgaImage<Grayscale>) -> Result<(), String> {
    let mut rng = rand::thread_rng();
    let x: u16 = rng.gen_range(0..image.get_width());
    let y: u16 = rng.gen_range(0..image.get_height());
    let c: u8 = rng.gen_range(0..255);
    let color: Grayscale = Grayscale { c };
    image.set(x, y, color)
}

fn generate_random_rgba_pixel(image: &mut TgaImage<RGBA>) -> Result<(), String> {
    let mut rng = rand::thread_rng();
    let x: u16 = rng.gen_range(0..image.get_width());
    let y: u16 = rng.gen_range(0..image.get_height());
    let r: u8 = rng.gen_range(0..255);
    let g: u8 = rng.gen_range(0..255);
    let b: u8 = rng.gen_range(0..255);
    let a: u8 = rng.gen_range(0..255);
    let color: RGBA = RGBA { r, g, b, a };
    image.set(x, y, color)
}

fn generate_random_rgb_pixel(image: &mut TgaImage<RGB>) -> Result<(), String> {
    let mut rng = rand::thread_rng();
    let x: u16 = rng.gen_range(0..image.get_width());
    let y: u16 = rng.gen_range(0..image.get_height());
    let r: u8 = rng.gen_range(0..255);
    let g: u8 = rng.gen_range(0..255);
    let b: u8 = rng.gen_range(0..255);
    let color: RGB = RGB { r, g, b };
    image.set(x, y, color)
}

fn draw_heart(image: &mut TgaImage<RGB>) -> Result<(), String> {
    //WWWWWWWWWWWWWWW
    let pixel = RGBPixels::new();
    let pixels = [
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
    ];
    draw_line(&pixels, image, 0);

    //WWW...WWW...WWW
    let pixels = [
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.b(),
        pixel.b(),
        pixel.b(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.b(),
        pixel.b(),
        pixel.b(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
    ];
    draw_line(&pixels, image, 1);

    //WWBRRRBWBRRDBWW
    let pixels = [
        pixel.w(),
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.b(),
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.r(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
        pixel.w(),
    ];
    draw_line(&pixels, image, 2);

    // W.RWWRR.RRRRD.W
    let pixels = [
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.w(),
        pixel.w(),
        pixel.r(),
        pixel.r(),
        pixel.b(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
    ];
    draw_line(&pixels, image, 3);

    // W.RWRRRRRRRRD.W
    let pixels = [
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.w(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
    ];
    draw_line(&pixels, image, 4);

    // W.RRRRRRRRRRD.W
    let pixels = [
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
    ];
    draw_line(&pixels, image, 5);

    // W.RWRRRRRRRRD.W
    let pixels = [
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.w(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
    ];
    draw_line(&pixels, image, 6);

    // WBRRRRRRRRRRDBW
    let pixels = [
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
    ];
    draw_line(&pixels, image, 7);

    // WWBRRRRRRRRDBWW
    let pixels = [
        pixel.w(),
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
        pixel.w(),
    ];
    draw_line(&pixels, image, 8);

    // WWWBRRRRRRDBWWW
    let pixels = [
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
    ];
    draw_line(&pixels, image, 9);

    // WWWWBRRRRDBWWWW
    let pixels = [
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
    ];
    draw_line(&pixels, image, 10);

    // WWWWWBRRDBWWWWW
    let pixels = [
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.r(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
    ];
    draw_line(&pixels, image, 11);

    // WWWWWWBDBWWWWWW
    let pixels = [
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.b(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
    ];
    draw_line(&pixels, image, 12);

    // WWWWWWW.WWWWWWW
    let pixels = [
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.b(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
    ];
    draw_line(&pixels, image, 13);

    // WWWWWWWWWWWWWWW
    let pixels = [
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
    ];
    draw_line(&pixels, image, 14);

    Ok(())
}

fn draw_rgba_heart(image: &mut TgaImage<RGBA>) -> Result<(), String> {
    //WWWWWWWWWWWWWWW
    let pixel = RGBAPixels::new();
    let pixels = [
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
    ];
    draw_rgba_line(&pixels, image, 0);

    //WWW...WWW...WWW
    let pixels = [
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.b(),
        pixel.b(),
        pixel.b(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.b(),
        pixel.b(),
        pixel.b(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
    ];
    draw_rgba_line(&pixels, image, 1);

    //WWBRRRBWBRRDBWW
    let pixels = [
        pixel.w(),
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.b(),
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.r(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
        pixel.w(),
    ];
    draw_rgba_line(&pixels, image, 2);

    // W.RWWRR.RRRRD.W
    let pixels = [
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.w(),
        pixel.w(),
        pixel.r(),
        pixel.r(),
        pixel.b(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
    ];
    draw_rgba_line(&pixels, image, 3);

    // W.RWRRRRRRRRD.W
    let pixels = [
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.w(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
    ];
    draw_rgba_line(&pixels, image, 4);

    // W.RRRRRRRRRRD.W
    let pixels = [
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
    ];
    draw_rgba_line(&pixels, image, 5);

    // W.RWRRRRRRRRD.W
    let pixels = [
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.w(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
    ];
    draw_rgba_line(&pixels, image, 6);

    // WBRRRRRRRRRRDBW
    let pixels = [
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
    ];
    draw_rgba_line(&pixels, image, 7);

    // WWBRRRRRRRRDBWW
    let pixels = [
        pixel.w(),
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
        pixel.w(),
    ];
    draw_rgba_line(&pixels, image, 8);

    // WWWBRRRRRRDBWWW
    let pixels = [
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
    ];
    draw_rgba_line(&pixels, image, 9);

    // WWWWBRRRRDBWWWW
    let pixels = [
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
    ];
    draw_rgba_line(&pixels, image, 10);

    // WWWWWBRRDBWWWWW
    let pixels = [
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.r(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
    ];
    draw_rgba_line(&pixels, image, 11);

    // WWWWWWBDBWWWWWW
    let pixels = [
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.b(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
    ];
    draw_rgba_line(&pixels, image, 12);

    // WWWWWWW.WWWWWWW
    let pixels = [
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.b(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
    ];
    draw_rgba_line(&pixels, image, 13);

    // WWWWWWWWWWWWWWW
    let pixels = [
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
    ];
    draw_rgba_line(&pixels, image, 14);

    Ok(())
}

fn draw_grayscale_heart(image: &mut TgaImage<Grayscale>) -> Result<(), String> {
    //WWWWWWWWWWWWWWW
    let pixel = GrayscalePixels::new();
    let pixels = [
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
    ];
    draw_grayscale_line(&pixels, image, 0);

    //WWW...WWW...WWW
    let pixels = [
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.b(),
        pixel.b(),
        pixel.b(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.b(),
        pixel.b(),
        pixel.b(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
    ];
    draw_grayscale_line(&pixels, image, 1);

    //WWBRRRBWBRRDBWW
    let pixels = [
        pixel.w(),
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.b(),
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.r(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
        pixel.w(),
    ];
    draw_grayscale_line(&pixels, image, 2);

    // W.RWWRR.RRRRD.W
    let pixels = [
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.w(),
        pixel.w(),
        pixel.r(),
        pixel.r(),
        pixel.b(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
    ];
    draw_grayscale_line(&pixels, image, 3);

    // W.RWRRRRRRRRD.W
    let pixels = [
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.w(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
    ];
    draw_grayscale_line(&pixels, image, 4);

    // W.RRRRRRRRRRD.W
    let pixels = [
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
    ];
    draw_grayscale_line(&pixels, image, 5);

    // W.RWRRRRRRRRD.W
    let pixels = [
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.w(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
    ];
    draw_grayscale_line(&pixels, image, 6);

    // WBRRRRRRRRRRDBW
    let pixels = [
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
    ];
    draw_grayscale_line(&pixels, image, 7);

    // WWBRRRRRRRRDBWW
    let pixels = [
        pixel.w(),
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
        pixel.w(),
    ];
    draw_grayscale_line(&pixels, image, 8);

    // WWWBRRRRRRDBWWW
    let pixels = [
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
    ];
    draw_grayscale_line(&pixels, image, 9);

    // WWWWBRRRRDBWWWW
    let pixels = [
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.r(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
    ];
    draw_grayscale_line(&pixels, image, 10);

    // WWWWWBRRDBWWWWW
    let pixels = [
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.b(),
        pixel.r(),
        pixel.r(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
    ];
    draw_grayscale_line(&pixels, image, 11);

    // WWWWWWBDBWWWWWW
    let pixels = [
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.b(),
        pixel.d(),
        pixel.b(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
    ];
    draw_grayscale_line(&pixels, image, 12);

    // WWWWWWW.WWWWWWW
    let pixels = [
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.b(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
    ];
    draw_grayscale_line(&pixels, image, 13);

    // WWWWWWWWWWWWWWW
    let pixels = [
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
        pixel.w(),
    ];
    draw_grayscale_line(&pixels, image, 14);

    Ok(())
}

fn draw_line(pixels: &[RGB], image: &mut TgaImage<RGB>, line: u16) {
    let mut i = 0;
    for pixel in pixels.iter() {
        match image.set(line, i, *pixel) {
            // Ok(_) => println!("Successfully wrote pixel on line {}!", line),
            Ok(_) => continue,
            Err(e) => println!("Error: {}", e),
        }
        i += 1;
    }
}

fn draw_rgba_line(pixels: &[RGBA], image: &mut TgaImage<RGBA>, line: u16) {
    let mut i = 0;
    for pixel in pixels.iter() {
        match image.set(line, i, *pixel) {
            Ok(_) => println!("Successfully wrote pixel!"),
            Err(e) => println!("Error: {}", e),
        }
        i += 1;
    }
}

fn draw_grayscale_line(pixels: &[Grayscale], image: &mut TgaImage<Grayscale>, line: u16) {
    let mut i = 0;
    for pixel in pixels.iter() {
        match image.set(line, i, *pixel) {
            Ok(_) => println!("Successfully wrote pixel!"),
            Err(e) => println!("Error: {}", e),
        }
        i += 1;
    }
}

/*
WWWWWWWWWWWWWWW
WWW...WWW...WWW
WW.RRR.W.RRD.WW
W.RWWRR.RRRRD.W
W.RWRRRRRRRRD.W
W.RRRRRRRRRRD.W
W.RWRRRRRRRRD.W

WBRRRRRRRRRRDBW
WWBRRRRRRRRDBWW
WWWBRRRRRRDBWWW
WWWWBRRRRDBWWWW
WWWWWBRRDBWWWWW
WWWWWWBDBWWWWWW
WWWWWWW.WWWWWWW
WWWWWWWWWWWWWWW
 */
