use image::open;
use ndarray::{Array1, Array3};
use ndarray::{s, ArrayView3};

pub struct Image {

    height : u32,
    width : u32,
    pixels : Array3<u8>,
}

pub struct GrayScaleImage {
    height : u32,
    width : u32,
    pixels : Array2<u8>,
}

impl From<&str> for Image {
    fn from(image: &str) -> Self {
        println!("works: {}", image);

        let image = open(image)
        .expect("Couldn't open image =(")
        .into_rgb8();

        let (height, width) = image.dimensions();
        let pixels_tmp = Array1::from(
            image.into_vec()
        );

        let pixels = pixels_tmp
            .into_shape((height as usize, width as usize, 3))
            .expect("Couldn't convert to ndarray =(");

        Image {
            height, 
            width,
            pixels,
        }
    }
}

impl Image {
    pub fn get_sub_array(&self, x : usize, y : usize) -> ArrayView3<u8> {
        self.pixels.slice(s![x..x+5, y..y+5, ..])
    }

    pub fn get_grayscale(&self) -> Array2<u8> {
        
    }
}
