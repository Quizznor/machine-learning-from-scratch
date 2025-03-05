use clap::{arg, Command};
use std::fs::canonicalize;
use std::path::PathBuf;

// mod triangle;
// mod picture;

mod utl;

/// Gives the triangulation of an image
// #[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
// struct Cli {
//     /// target path of the image
//     #[arg(short, long)]
//     image: path::Path,

//     /// fraction of triangles/image pixel count
//     #[arg(short, long, default_value_t = 0.003)]
//     n_tri: f64,

//     /// mesh generation permissivity
//     #[arg(short, long, default_value_t = 3.)]
//     power: f64,
// }

pub fn main() {
    let matches = Command::new("ImageTriangulator")
        .version("1.0")
        .about("Return a low-poly representation of a given image")
        .arg(arg!([image] "path to image").required(true))
        // placeholders TODO
        .arg(
            arg!(-n --points <VALUE>)
                .help("Number of points to use for triangulation")
                .default_value("100")
                .required(false)
                .value_parser(clap::value_parser!(u32)),
        )
        .get_matches();

    let path = canonicalize(PathBuf::from(matches.get_one::<String>("image").unwrap()))
        .expect("Image does not exist =(");
    let n_points = matches.get_one::<u32>("points").unwrap();

    let original_image = runtime!(utl::read_image(&path), "reading image");
    #[cfg(debug_assertions)]
    {
        utl::save_image(&original_image, &path, "original");
    }

    let edges = runtime!(utl::contrast(&original_image), "finding contrast");
    #[cfg(debug_assertions)]
    {
        utl::save_image(&edges, &path, "edges");
    }

    let (triangulation, points) = runtime!(utl::get_triangulation(n_points, edges), "building mesh");
    #[cfg(debug_assertions)]
    {
        let mesh = utl::build_mesh_image(&triangulation, &points);
        utl::save_image(&mesh, &path, "mesh")
    }
    
    // let mesh = runtime!(utl::create_mesh(coordinates), "triangulating image");

    // let test = utl::image_to_array(&original_image);
    // let test2 = utl::array_to_image(&test);

    // let _ = runtime!(utl::save_image(image, "test/out_original.png"), "saving original image");

    // let edges = runtime!(utl::get_edges(&image), "getting edges");

    // TODO!!!!!
    // better sobel edge detection
    // better mesh creation scheiss
    // fix runtime error

    // let image = image::open(&args.image).expect("Error reading image").to_rgba8();
    // let image: NdColor = NdImage(&image).into();

    // println!("{}", type_of(&image));

    // let edges = runtime!(utl::sobel(&image), "searching for object edges");

    // let grayscale = runtime!(original_image.to_grayscale(), "transforming to grayscale");

    // determine triangulation parameters
    // let dimensions = grayscale.dim();
    // let n_triangles = (args.n_tri * dimensions.0 as f64 * dimensions.1 as f64) as usize;

    // #[cfg(debug_assertions)]
    // utl::save_gray_image(&grayscale, &(args.image.to_owned() + "_grayscale"));

    // utl::save_gray_image(&edges, &(args.image.to_owned() + "_grayscale"));

    // #[cfg(debug_assertions)]
    // utl::save_gray_image(&edges, &(args.image.to_owned() + "_edges"));

    // let mesh = runtime!(utl::create_mesh(edges, n_triangles, args.power), "building delauney mesh");

    // #[cfg(debug_assertions)]
    // utl::save_mesh_image(&mesh, dimensions, &(args.image.to_owned() + "_mesh"));

    // let color_information = runtime!(utl::calculate_colors(&original_image, mesh), "sorting pixels, building triangles");

    // runtime!(original_image.apply_colormap(color_information), "apply colormap to image");
    // runtime!(original_image.save(&(args.image.to_owned() + "_triangulation")), "save triangulation picture");
}
