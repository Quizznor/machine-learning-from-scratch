use clap::Parser;

mod picture;
mod utl;

/// Gives the triangulation of an image
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// target path of the image
    #[arg(short, long)]
    image: String,

    /// fraction of triangles/image pixel count
    #[arg(short, long, default_value_t = 0.001)]
    n_tri: f64,

    /// mesh generation permissivity
    #[arg(short, long, default_value_t = 3.)]
    power: f64,
}

pub fn main() {
    let args = Cli::parse();

    let mut original_image = picture::Picture::from(&args.image);
    let grayscale = original_image.to_grayscale();

    // determine triangulation parameters
    let dimensions = grayscale.dim();
    let n_triangles = (args.n_tri * dimensions.0 as f64 * dimensions.1 as f64) as usize;

    #[cfg(debug_assertions)]
    utl::save_gray_image(&grayscale, &(args.image.to_owned() + "_grayscale"));

    let edges = utl::sobel_operator(grayscale);

    #[cfg(debug_assertions)]
    utl::save_gray_image(&edges, &(args.image.to_owned() + "_edges"));

    let mesh = utl::create_mesh(edges, n_triangles, args.power);

    #[cfg(debug_assertions)]
    utl::save_mesh_image(&mesh, dimensions, &(args.image.to_owned() + "_mesh"));

    let color_information = utl::sort_pixels_into_triangles(&original_image, mesh);

    original_image.apply_colormap(color_information);
    original_image.save(&(args.image.to_owned() + "_triangulation"));
}
