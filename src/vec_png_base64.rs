extern crate rustc_serialize;
extern crate png;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use png::HasParameters;
use std::io::Read;

use rustc_serialize::base64::{ToBase64, MIME};

pub fn png_to_file(width: u32, height: u32, array: &std::vec::Vec<u8>) {
    let path = Path::new(r"buffer.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
     let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&array).unwrap();
}

pub fn file_to_base64() -> String {
    let mut file = File::open(r"buffer.png").unwrap();
    let mut vec = Vec::new();
    let _ = file.read_to_end(&mut vec);
    let base64 = vec.to_base64(MIME);
    return format!("data:image/png;base64,{}", base64.replace("\r\n", ""));
}

pub fn png_to_base64(width: u32, height: u32, array: &std::vec::Vec<u8>) -> String {
    png_to_file(width, height, array);
    return file_to_base64();
}

#[cfg(test)]
mod tests {
    use crate::vec_png_base64::png_to_base64;
    #[test]
    fn png_to_base64_works() {
        let expected = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAIAAAACCAYAAABytg0kAAAAE0lEQVR4nGP8z8DwH0QwAmkwBQA2BgUArjfvtQAAAABJRU5ErkJggg==";
        let res = png_to_base64(2, 2, &vec![255, 0, 0 , 255,
                        0, 255, 0, 255,
                        0, 0, 255, 255,
                        0, 0, 0, 255]);
        assert_eq!(res, expected);
    }
}