use ndarray::Array2;
use ::image::GrayImage;
use image::ImageFormat::Png;

pub fn save_gray_image(target: &Array2<u8>, name: &str) -> () {

    let (width, height) = target.dim();
    let raw_vector = target.clone().into_raw_vec();

    GrayImage::from_raw(width as u32, height as u32, raw_vector)
        .expect("Couldn't build image from provided arguments")
        .save_with_format("out/".to_owned() + name + ".png", Png)
        .expect("Couldn't save image from provided arguments")
}