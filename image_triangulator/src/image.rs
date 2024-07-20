use image::open;
use ndarray::{Array1, Array2, Array3};

pub struct Image {

    height : u32,
    width : u32,
    pixels : Array3<u8>,
}

impl From<&str> for Image {
    fn from(image: &str) -> Self {

        let image = open(image)
        .expect(&format!("Couldn't open image: {}", image))
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
    // pub fn get_sub_array(&self, x : usize, y : usize) -> ArrayView3<u8> {
    //     self.pixels.slice(s![x..x+5, y..y+5, ..])
    // }

    pub fn to_grayscale(&self) -> Array2<u8> {

        println!("generating grayscale image...");

        let mut out_array = Array2::<u8>::zeros((self.height as usize, self.width as usize));

        for (i_row, row) in self.pixels.axis_iter(ndarray::Axis(0)).enumerate() {
            for (i_col, col) in row.axis_iter(ndarray::Axis(0)).enumerate() {
                out_array[[i_row, i_col]] = (0.299 * col[[0]] as f64 + 0.578 * col[[1]] as f64 + 0.114 * col[[2]] as f64) as u8;
            }
        }

        out_array
    }
}
