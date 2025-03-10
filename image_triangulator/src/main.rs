use clap::{arg, Command};
use std::fs::canonicalize;
use std::path::PathBuf;

mod utl;
mod mesh;

pub fn main() {
    let matches = Command::new("ImageTriangulator")
        .version("1.0")
        .about("Return a low-poly representation of a given image")
        .arg(arg!([image] "path to image").required(true))
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

    let mesh = runtime!(utl::get_mesh(n_points, edges), "building mesh");
    #[cfg(debug_assertions)]
    {
        let mesh_image = utl::build_mesh_image(&mesh);
        utl::save_image(&mesh_image, &path, "mesh")
    }

    let color_lookup_table = runtime!(utl::color_lookup_table(mesh, &original_image), "calculating groups");
    let triangulated_image = runtime!(utl::color(color_lookup_table, original_image), "applying colormap");
    utl::save_image(&triangulated_image, &path, "triangulated");


}