use std::env;
mod picture;
mod utl;

pub fn main() {

    let args: Vec<String> = env::args().collect();
    let image: &str = &args[1];

    let mut original_image = picture::Picture::from(image);
    let grayscale = original_image.to_grayscale();
    let dimensions = grayscale.dim();
    
    #[cfg(debug_assertions)]
    utl::save_gray_image(&grayscale, &(image.to_owned() + "_grayscale"));
    
    let edges = utl::sobel_operator(grayscale);

    #[cfg(debug_assertions)]
    utl::save_gray_image(&edges, &(image.to_owned() + "_edges"));

    let mesh = utl::create_mesh(edges, 500);

    #[cfg(debug_assertions)]
    utl::save_mesh_image(&mesh, dimensions, &(image.to_owned() + "_mesh"));

    let color_information = utl::sort_pixels_into_triangles(&original_image, mesh);

    original_image.apply_colormap(color_information);
    original_image.into(&(image.to_owned() + "_triangulation.png"));
    
}