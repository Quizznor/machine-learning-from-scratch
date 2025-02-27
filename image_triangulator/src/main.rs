use clap::Parser;

mod triangle;
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
    #[arg(short, long, default_value_t = 0.003)]
    n_tri: f64,

    /// mesh generation permissivity
    #[arg(short, long, default_value_t = 3.)]
    power: f64,
}

pub fn main() {
    let args = Cli::parse();

    // TODO!!!!!
    // better sobel edge detection
    // better mesh creation scheiss
    // fix runtime error

    let mut original_image = runtime!(picture::Picture::from(&args.image), "reading image");
    let grayscale = runtime!(original_image.to_grayscale(), "transforming to grayscale");

    // determine triangulation parameters
    let dimensions = grayscale.dim();
    let n_triangles = (args.n_tri * dimensions.0 as f64 * dimensions.1 as f64) as usize;

    #[cfg(debug_assertions)]
    utl::save_gray_image(&grayscale, &(args.image.to_owned() + "_grayscale"));

    let edges = runtime!(utl::sobel_operator(grayscale), "searching for object edges");

    #[cfg(debug_assertions)]
    utl::save_gray_image(&edges, &(args.image.to_owned() + "_edges"));

    let mesh = runtime!(utl::create_mesh(edges, n_triangles, args.power), "building delauney mesh");

    #[cfg(debug_assertions)]
    utl::save_mesh_image(&mesh, dimensions, &(args.image.to_owned() + "_mesh"));

    let color_information = runtime!(utl::calculate_colors(&original_image, mesh), "sorting pixels, building triangles");

    runtime!(original_image.apply_colormap(color_information), "apply colormap to image");
    runtime!(original_image.save(&(args.image.to_owned() + "_triangulation")), "save triangulation picture");
}
