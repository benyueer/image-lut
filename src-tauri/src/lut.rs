use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Cursor, Seek, SeekFrom, Read},
};

use base64::{encode, write::EncoderWriter};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb, RgbImage};

#[derive(Clone, Copy, Debug)]
struct Rgbvec {
    r: f64,
    g: f64,
    b: f64,
}

struct Lut3D {
    lutsize: i32,
    lutsize2: i32,
    lut: Vec<Rgbvec>,
}

#[derive(Default, Debug)]
pub struct LutBuilder {
    AgressiveLut: Option<Vec<Rgbvec>>,
    FashionLut: Option<Vec<Rgbvec>>,
    HiConLut: Option<Vec<Rgbvec>>,
}

impl LutBuilder {
    pub fn init_lut(&mut self) {
        self.AgressiveLut = Some(resolve_lut_file("/Users/mac/Desktop/pro/image-lut/src-tauri/luts/Agressive.cube"));
        self.FashionLut = Some(resolve_lut_file("/Users/mac/Desktop/pro/image-lut/src-tauri/luts/Fashion 1 33 E-E.cube"));
        self.HiConLut = Some(resolve_lut_file("/Users/mac/Desktop/pro/image-lut/src-tauri/luts/HiCon 33 E-E.cube"));
    }

    pub fn use_lut(&mut self, lut: &str, image_path: &str) -> Option<String>{
        let img = image::open(image_path).expect("Failed open image");
        let width = img.width();
        let height = img.height();

        let lut3d = Lut3D {
            lutsize: 33,
            lutsize2: 33 * 33,
            lut: match lut {
                "Agressive" => self.AgressiveLut.clone().unwrap(),
                "Fashion" => self.FashionLut.clone().unwrap(),
                "HiCon" => self.HiConLut.clone().unwrap(),
                _ => unreachable!(),
            },
        };

        let mut new_pixel: Vec<Rgbvec> = vec![];
        let pixel = get_image_pixel(img);
        println!("begin");

        for i in pixel {
            let (r, g, b) = (i.r, i.g, i.b);
            let s = Rgbvec {
                r: r / 255.0 * 32.0,
                g: g / 255.0 * 32.0,
                b: b / 255.0 * 32.0,
            };
            let c = interp_trilinear(&lut3d, s);
            // let c = interp_nearest(&lut3d, s);

            let (r, g, b) = (c.r, c.g, c.b);
            let s = Rgbvec {
                r: r * 255.0,
                g: g * 255.0,
                b: b * 255.0,
            };
            new_pixel.push(s);

            // println!("{:?} -> {:?}", i, s);
        }
        println!("to file");
        pixel_to_image(new_pixel, width, height)
    }
}

const fn prev(x: f64) -> i32 {
    x as i32
}

const fn next(x: f64, lutsize: i32) -> i32 {
    let result = (x as i32) + 1;
    if result < lutsize {
        result
    } else {
        lutsize - 1
    }
}

fn near(x: f64) -> f64 {
    (x + 0.5) as i32 as f64
}

fn lerpf(v0: f64, v1: f64, f: f64) -> f64 {
    v0 + (v1 - v0) * f
}

fn lerp(v0: Rgbvec, v1: Rgbvec, f: f64) -> Rgbvec {
    Rgbvec {
        r: lerpf(v0.r, v1.r, f),
        g: lerpf(v0.g, v1.g, f),
        b: lerpf(v0.b, v1.b, f),
    }
}

fn interp_trilinear(lut3d: &Lut3D, s: Rgbvec) -> Rgbvec {
    let lutsize2 = lut3d.lutsize2;
    let lutsize = lut3d.lutsize;
    let prev_ = vec![prev(s.r), prev(s.g), prev(s.b)];
    let next_ = vec![next(s.r, lutsize), next(s.g, lutsize), next(s.b, lutsize)];
    let d = Rgbvec {
        r: s.r - prev_[0] as f64,
        g: s.g - prev_[1] as f64,
        b: s.b - prev_[2] as f64,
    };
    let c000 = lut3d.lut[(prev_[0] * lutsize2 + prev_[1] * lutsize + prev_[2]) as usize];
    let c001 = lut3d.lut[(prev_[0] * lutsize2 + prev_[1] * lutsize + next_[2]) as usize];
    let c010 = lut3d.lut[(prev_[0] * lutsize2 + next_[1] * lutsize + prev_[2]) as usize];
    let c011 = lut3d.lut[(prev_[0] * lutsize2 + next_[1] * lutsize + next_[2]) as usize];
    let c100 = lut3d.lut[(next_[0] * lutsize2 + prev_[1] * lutsize + prev_[2]) as usize];
    let c101 = lut3d.lut[(next_[0] * lutsize2 + prev_[1] * lutsize + next_[2]) as usize];
    let c110 = lut3d.lut[(next_[0] * lutsize2 + next_[1] * lutsize + prev_[2]) as usize];
    let c111 = lut3d.lut[(next_[0] * lutsize2 + next_[1] * lutsize + next_[2]) as usize];

    let c00 = lerp(c000, c100, d.r);
    let c10 = lerp(c010, c110, d.r);
    let c01 = lerp(c001, c101, d.r);
    let c11 = lerp(c011, c111, d.r);

    let c0 = lerp(c00, c10, d.g);
    let c1 = lerp(c01, c11, d.g);
    let c = lerp(c0, c1, d.b);

    return c;
}

fn interp_nearest(lut3d: &Lut3D, s: Rgbvec) -> Rgbvec {
    lut3d.lut[(near(s.r) * lut3d.lutsize2 as f64 + near(s.g) * lut3d.lutsize as f64 + near(s.b))
        as usize]
}

fn resolve_lut_file(path: &str) -> Vec<Rgbvec> {
    let mut start = false;
    let mut result: Vec<Rgbvec> = vec![];

    let file = File::open(path).expect("failed open");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                if start && line.len() > 0 {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() != 3 {
                        continue;
                    }

                    result.push(Rgbvec {
                        b: parts[0].parse().unwrap(),
                        g: parts[1].parse().unwrap(),
                        r: parts[2].parse().unwrap(),
                    })
                }

                if line == "LUT_3D_SIZE 33".to_string() {
                    start = true;
                }
            }
            Err(err) => {
                eprintln!("Error read linr: {}", err);
            }
        }
    }

    result
}

fn get_image_pixel(img: DynamicImage) -> Vec<Rgbvec> {
    // let width = img.width();
    // let height = img.height();

    // let mut result = vec![];

    img.pixels().map(|(_, _, pixel)| {
        let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
            return Rgbvec {
                r: r as f64,
                g: g as f64,
                b: b as f64,
            };
    }).collect()

    // for y in 0..height {
    //     for x in 0..width {
    //         let pixel = img.get_pixel(x, y);
    //         let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
    //         result.push(Rgbvec {
    //             r: r as f64,
    //             g: g as f64,
    //             b: b as f64,
    //         });
    //     }
    // }

    // result
}

fn pixel_to_image(pixel: Vec<Rgbvec>, width: u32, height: u32) -> Option<String> {
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let index = (width * y + x) as usize;
            let rgbvec = pixel[index];
            let p = Rgb([rgbvec.r as u8, rgbvec.g as u8, rgbvec.b as u8]);
            img.put_pixel(x, y, p);
        }
    }
    
    let dynamic_image: DynamicImage = DynamicImage::ImageRgb8(img);
    let mut cursor = Cursor::new(Vec::new());
    dynamic_image.write_to(&mut cursor, image::ImageOutputFormat::Png).expect("Failed to encode image");
    cursor.seek(SeekFrom::Start(0)).expect("Failed to seek cursor");
    let buffer = cursor.into_inner();
    let base64_data = encode(&buffer);
    return Some(base64_data);
}

pub fn image_to_base64(image_path: String) -> Option<String> {
    if let Ok(image_data) = fs::read(image_path) {
        let base64_data = encode(&image_data);
        return Some(base64_data);
    }
    return None;
}

#[test]
fn test_read_file() {
    let s = resolve_lut_file("/Users/mac/Desktop/pro/img-lut/luts/Fashion 1 33 E-E.cube");
    println!("{:?}", s);
}

#[test]
fn image_resolve_save() {
    let image_path = "/Users/mac/Desktop/pro/img-lut/input.JPG";
    let img = image::open(image_path).expect("Failed open image");
    let width = img.width();
    let height = img.height();
    let pixel = get_image_pixel(img);

    pixel_to_image(pixel, width, height);
}

#[test]
fn test_lut_builder() {
    let mut lb = LutBuilder::default();
    lb.init_lut();
    let r = lb.use_lut("FashionLut", "/Users/mac/Desktop/图/Snipaste_2023-10-10_18-28-56.png");
    println!("{}", r.unwrap());

}