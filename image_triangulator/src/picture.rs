use image::{open, ImageFormat::Png, RgbImage};
use ndarray::{s, ArrayView1};
use ndarray::{Array1, Array2, Array3};
use std::{io, io::Write};

pub struct Picture {
    height: u32,
    width: u32,
    pixels: Array3<u8>,
}

impl From<&String> for Picture {
    fn from(picture: &String) -> Self {

        let picture = open(&picture)
            .expect(&format!("Couldn't open image: {}", picture))
            .into_rgb8();

        let (height, width) = picture.dimensions();
        let pixels_tmp = Array1::from(picture.into_vec());

        let pixels = pixels_tmp
            .into_shape((height as usize, width as usize, 3))
            .expect("Couldn't convert to ndarray =(");

        Picture {
            height,
            width,
            pixels,
        }
    }
}

impl Picture {
    pub fn to_grayscale(&self) -> Array2<u8> {

        let mut out_array = Array2::<u8>::zeros((self.height as usize, self.width as usize));

        for (i_row, row) in self.pixels.axis_iter(ndarray::Axis(0)).enumerate() {
            for (i_col, col) in row.axis_iter(ndarray::Axis(0)).enumerate() {
                out_array[[i_row, i_col]] = (0.299 * col[[0]] as f64
                    + 0.578 * col[[1]] as f64
                    + 0.114 * col[[2]] as f64) as u8;
            }
        }

        out_array
    }

    pub fn dimension(&self) -> (usize, usize) {
        (self.width as usize, self.height as usize)
    }

    pub fn get_pixel(&self, idx: usize, idy: usize) -> ArrayView1<u8> {
        self.pixels.slice(s![idx, idy, ..])
    }

    pub fn save(&self, path: &str) -> () {
        let raw_vector = self.pixels.clone().into_raw_vec();
        let (width, height) = self.dimension();

        RgbImage::from_raw(width as u32, height as u32, raw_vector)
            .expect("Couldn't build image from provided arguments")
            .save_with_format("out/".to_owned() + path + ".png", Png)
            .expect("Couldn't save image from provided arguments")
    }

    pub fn apply_colormap(&mut self, color_information: (Array2<u8>, Array2<usize>)) -> &Self {
        let (colors, lookup_table) = color_information;

        for w in 0..self.dimension().0 {
            for h in 0..self.dimension().1 {
                self.pixels[[w, h, 0]] = colors[[lookup_table[[w, h]], 0]];
                self.pixels[[w, h, 1]] = colors[[lookup_table[[w, h]], 1]];
                self.pixels[[w, h, 2]] = colors[[lookup_table[[w, h]], 2]];
            }
        }

        self
    }
}
