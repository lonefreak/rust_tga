#![allow(dead_code)]
use chrono::Utc;
use rand::Rng;
use rust_tga::tga::{color_model::Grayscale, color_model::RGB, color_model::RGBA, TgaImage};

const NUMBER_OF_RANDOM_PIXELS: u16 = 500;
fn main() {
    let timestamp: String = Utc::now().format("%Y%m%d%H%M%S").to_string();

    make_rgb_heart(&timestamp);
    // make_rgba_heart(&timestamp);
    // make_grayscale_heart(&timestamp);
    // make_random_rgb(&timestamp);
    // make_random_rgba(&timestamp);
    // make_random_grayscale(&timestamp);
    // read_and_write_arrow(&timestamp);
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

    fn white(&self) -> RGB {
        RGB::set(255, 255, 255)
    }

    fn red(&self) -> RGB {
        RGB::set(255, 0, 0)
    }

    fn black(&self) -> RGB {
        RGB::set(0, 0, 0)
    }

    fn dark_red(&self) -> RGB {
        RGB::set(139, 0, 0)
    }
}

struct RGBAPixels {}

impl RGBAPixels {
    fn new() -> Self {
        RGBAPixels {}
    }

    fn white(&self) -> RGBA {
        RGBA::set(255, 255, 255, 255)
    }

    fn red(&self) -> RGBA {
        RGBA::set(255, 0, 0, 130)
    }

    fn blue(&self) -> RGBA {
        RGBA::set(0, 0, 255, 130)
    }

    fn black(&self) -> RGBA {
        RGBA::set(0, 0, 0, 255)
    }

    fn dark_red(&self) -> RGBA {
        RGBA::set(139, 0, 0, 200)
    }

    fn dark_blue(&self) -> RGBA {
        RGBA::set(0, 0, 139, 200)
    }
}

struct GrayscalePixels {}

impl GrayscalePixels {
    fn new() -> Self {
        GrayscalePixels {}
    }

    fn white(&self) -> Grayscale {
        Grayscale::set(255)
    }

    fn light_gray(&self) -> Grayscale {
        Grayscale::set(155)
    }

    fn black(&self) -> Grayscale {
        Grayscale::set(0)
    }

    fn dark_gray(&self) -> Grayscale {
        Grayscale::set(95)
    }
}

fn generate_random_grayscale_pixel(image: &mut TgaImage<Grayscale>) -> Result<(), String> {
    let mut rng = rand::thread_rng();
    let x: u16 = rng.gen_range(0..image.get_width());
    let y: u16 = rng.gen_range(0..image.get_height());
    let c: u8 = rng.gen_range(0..255);
    let color: Grayscale = Grayscale::set(c);
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
    let color = RGBA::set(r, g, b, a);
    image.set(x, y, color)
}

fn generate_random_rgb_pixel(image: &mut TgaImage<RGB>) -> Result<(), String> {
    let mut rng = rand::thread_rng();
    let x: u16 = rng.gen_range(0..image.get_width());
    let y: u16 = rng.gen_range(0..image.get_height());
    let r: u8 = rng.gen_range(0..255);
    let g: u8 = rng.gen_range(0..255);
    let b: u8 = rng.gen_range(0..255);
    let color: RGB = RGB::set(r, g, b);
    image.set(x, y, color)
}

fn draw_heart(image: &mut TgaImage<RGB>) -> Result<(), String> {
    let pixel = RGBPixels::new();
    let pixels = [
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
    ];
    draw_line(&pixels, image, 0);

    let pixels = [
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.black(),
        pixel.black(),
        pixel.black(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.black(),
        pixel.black(),
        pixel.black(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
    ];
    draw_line(&pixels, image, 1);

    let pixels = [
        pixel.white(),
        pixel.white(),
        pixel.black(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.black(),
        pixel.white(),
        pixel.black(),
        pixel.red(),
        pixel.red(),
        pixel.dark_red(),
        pixel.black(),
        pixel.white(),
        pixel.white(),
    ];
    draw_line(&pixels, image, 2);

    // W.RWWRR.RRRRD.W
    let pixels = [
        pixel.white(),
        pixel.black(),
        pixel.red(),
        pixel.white(),
        pixel.white(),
        pixel.red(),
        pixel.red(),
        pixel.black(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.dark_red(),
        pixel.black(),
        pixel.white(),
    ];
    draw_line(&pixels, image, 3);

    // W.RWRRRRRRRRD.W
    let pixels = [
        pixel.white(),
        pixel.black(),
        pixel.red(),
        pixel.white(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.dark_red(),
        pixel.black(),
        pixel.white(),
    ];
    draw_line(&pixels, image, 4);

    // W.RRRRRRRRRRD.W
    let pixels = [
        pixel.white(),
        pixel.black(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.dark_red(),
        pixel.black(),
        pixel.white(),
    ];
    draw_line(&pixels, image, 5);

    // W.RWRRRRRRRRD.W
    let pixels = [
        pixel.white(),
        pixel.black(),
        pixel.red(),
        pixel.white(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.dark_red(),
        pixel.black(),
        pixel.white(),
    ];
    draw_line(&pixels, image, 6);

    // WBRRRRRRRRRRDBW
    let pixels = [
        pixel.white(),
        pixel.black(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.dark_red(),
        pixel.black(),
        pixel.white(),
    ];
    draw_line(&pixels, image, 7);

    // WWBRRRRRRRRDBWW
    let pixels = [
        pixel.white(),
        pixel.white(),
        pixel.black(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.dark_red(),
        pixel.black(),
        pixel.white(),
        pixel.white(),
    ];
    draw_line(&pixels, image, 8);

    // WWWBRRRRRRDBWWW
    let pixels = [
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.black(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.dark_red(),
        pixel.black(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
    ];
    draw_line(&pixels, image, 9);

    // WWWWBRRRRDBWWWW
    let pixels = [
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.black(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.red(),
        pixel.dark_red(),
        pixel.black(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
    ];
    draw_line(&pixels, image, 10);

    // WWWWWBRRDBWWWWW
    let pixels = [
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.black(),
        pixel.red(),
        pixel.red(),
        pixel.dark_red(),
        pixel.black(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
    ];
    draw_line(&pixels, image, 11);

    // WWWWWWBDBWWWWWW
    let pixels = [
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.black(),
        pixel.dark_red(),
        pixel.black(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
    ];
    draw_line(&pixels, image, 12);

    // WWWWWWW.WWWWWWW
    let pixels = [
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.black(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
    ];
    draw_line(&pixels, image, 13);

    // WWWWWWWWWWWWWWW
    let pixels = [
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
    ];
    draw_line(&pixels, image, 14);

    Ok(())
}

fn draw_rgba_heart(image: &mut TgaImage<RGBA>) -> Result<(), String> {
    let pixel = RGBAPixels::new();
    let pixels = [
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
    ];
    draw_rgba_line(&pixels, image, 0);

    let pixels = [
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.black(),
        pixel.black(),
        pixel.black(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.black(),
        pixel.black(),
        pixel.black(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
    ];
    draw_rgba_line(&pixels, image, 1);

    let pixels = [
        pixel.white(),
        pixel.white(),
        pixel.black(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.black(),
        pixel.white(),
        pixel.black(),
        pixel.blue(),
        pixel.blue(),
        pixel.dark_blue(),
        pixel.black(),
        pixel.white(),
        pixel.white(),
    ];
    draw_rgba_line(&pixels, image, 2);

    // W.RWWRR.RRRRD.W
    let pixels = [
        pixel.white(),
        pixel.black(),
        pixel.blue(),
        pixel.white(),
        pixel.white(),
        pixel.blue(),
        pixel.blue(),
        pixel.black(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.dark_blue(),
        pixel.black(),
        pixel.white(),
    ];
    draw_rgba_line(&pixels, image, 3);

    // W.RWRRRRRRRRD.W
    let pixels = [
        pixel.white(),
        pixel.black(),
        pixel.blue(),
        pixel.white(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.dark_blue(),
        pixel.black(),
        pixel.white(),
    ];
    draw_rgba_line(&pixels, image, 4);

    // W.RRRRRRRRRRD.W
    let pixels = [
        pixel.white(),
        pixel.black(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.dark_blue(),
        pixel.black(),
        pixel.white(),
    ];
    draw_rgba_line(&pixels, image, 5);

    // W.RWRRRRRRRRD.W
    let pixels = [
        pixel.white(),
        pixel.black(),
        pixel.blue(),
        pixel.white(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.dark_blue(),
        pixel.black(),
        pixel.white(),
    ];
    draw_rgba_line(&pixels, image, 6);

    // WBRRRRRRRRRRDBW
    let pixels = [
        pixel.white(),
        pixel.black(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.dark_blue(),
        pixel.black(),
        pixel.white(),
    ];
    draw_rgba_line(&pixels, image, 7);

    // WWBRRRRRRRRDBWW
    let pixels = [
        pixel.white(),
        pixel.white(),
        pixel.black(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.dark_blue(),
        pixel.black(),
        pixel.white(),
        pixel.white(),
    ];
    draw_rgba_line(&pixels, image, 8);

    // WWWBRRRRRRDBWWW
    let pixels = [
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.black(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.dark_blue(),
        pixel.black(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
    ];
    draw_rgba_line(&pixels, image, 9);

    // WWWWBRRRRDBWWWW
    let pixels = [
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.black(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.blue(),
        pixel.dark_blue(),
        pixel.black(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
    ];
    draw_rgba_line(&pixels, image, 10);

    // WWWWWBRRDBWWWWW
    let pixels = [
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.black(),
        pixel.blue(),
        pixel.blue(),
        pixel.dark_blue(),
        pixel.black(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
    ];
    draw_rgba_line(&pixels, image, 11);

    // WWWWWWBDBWWWWWW
    let pixels = [
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.black(),
        pixel.dark_blue(),
        pixel.black(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
    ];
    draw_rgba_line(&pixels, image, 12);

    // WWWWWWW.WWWWWWW
    let pixels = [
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.black(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
    ];
    draw_rgba_line(&pixels, image, 13);

    // WWWWWWWWWWWWWWW
    let pixels = [
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
    ];
    draw_rgba_line(&pixels, image, 14);

    Ok(())
}

fn draw_grayscale_heart(image: &mut TgaImage<Grayscale>) -> Result<(), String> {
    let pixel = GrayscalePixels::new();
    let pixels = [
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
    ];
    draw_grayscale_line(&pixels, image, 0);

    let pixels = [
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.black(),
        pixel.black(),
        pixel.black(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.black(),
        pixel.black(),
        pixel.black(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
    ];
    draw_grayscale_line(&pixels, image, 1);

    let pixels = [
        pixel.white(),
        pixel.white(),
        pixel.black(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.black(),
        pixel.white(),
        pixel.black(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.dark_gray(),
        pixel.black(),
        pixel.white(),
        pixel.white(),
    ];
    draw_grayscale_line(&pixels, image, 2);

    let pixels = [
        pixel.white(),
        pixel.black(),
        pixel.light_gray(),
        pixel.white(),
        pixel.white(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.black(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.dark_gray(),
        pixel.black(),
        pixel.white(),
    ];
    draw_grayscale_line(&pixels, image, 3);

    let pixels = [
        pixel.white(),
        pixel.black(),
        pixel.light_gray(),
        pixel.white(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.dark_gray(),
        pixel.black(),
        pixel.white(),
    ];
    draw_grayscale_line(&pixels, image, 4);

    let pixels = [
        pixel.white(),
        pixel.black(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.dark_gray(),
        pixel.black(),
        pixel.white(),
    ];
    draw_grayscale_line(&pixels, image, 5);

    let pixels = [
        pixel.white(),
        pixel.black(),
        pixel.light_gray(),
        pixel.white(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.dark_gray(),
        pixel.black(),
        pixel.white(),
    ];
    draw_grayscale_line(&pixels, image, 6);

    let pixels = [
        pixel.white(),
        pixel.black(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.dark_gray(),
        pixel.black(),
        pixel.white(),
    ];
    draw_grayscale_line(&pixels, image, 7);

    let pixels = [
        pixel.white(),
        pixel.white(),
        pixel.black(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.dark_gray(),
        pixel.black(),
        pixel.white(),
        pixel.white(),
    ];
    draw_grayscale_line(&pixels, image, 8);

    let pixels = [
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.black(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.dark_gray(),
        pixel.black(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
    ];
    draw_grayscale_line(&pixels, image, 9);

    let pixels = [
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.black(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.dark_gray(),
        pixel.black(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
    ];
    draw_grayscale_line(&pixels, image, 10);

    let pixels = [
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.black(),
        pixel.light_gray(),
        pixel.light_gray(),
        pixel.dark_gray(),
        pixel.black(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
    ];
    draw_grayscale_line(&pixels, image, 11);

    let pixels = [
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.black(),
        pixel.dark_gray(),
        pixel.black(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
    ];
    draw_grayscale_line(&pixels, image, 12);

    let pixels = [
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.black(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
    ];
    draw_grayscale_line(&pixels, image, 13);

    let pixels = [
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
        pixel.white(),
    ];
    draw_grayscale_line(&pixels, image, 14);

    Ok(())
}

fn draw_line(pixels: &[RGB], image: &mut TgaImage<RGB>, line: u16) {
    let mut i = 0;
    for pixel in pixels.iter() {
        match image.set(line, i, *pixel) {
            Ok(_) => i += 1,
            Err(e) => println!("Error: {}", e),
        }
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
