mod algorithm;
use std::env;
mod image;
mod utl;


pub fn main() {

    let args: Vec<String> = env::args().collect();
    let image: &str = &args[1];

    let original_image = image::Image::from(image);
    let grayscale = original_image.to_grayscale();
    
    #[cfg(debug_assertions)]
    utl::save_gray_image(&grayscale, &(image.to_owned() + "_grayscale"));
    
    let edges = algorithm::sobel_operator(grayscale);

    #[cfg(debug_assertions)]
    utl::save_gray_image(&edges, &(image.to_owned() + "_edges"));
    
}