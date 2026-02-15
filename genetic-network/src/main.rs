use clap::{arg, Command};

mod utl;

fn main() {
    let matches = Command::new("GeneticNetwork")
        .version("1.0")
        .about("Run a genetic optimization of 'traffic' on a graph")
        .arg(
            arg!(-n --points <VALUE>)
            .help("Number of nodes")
            .default_value("100")
            .required(false)
            .value_parser(clap::value_parser!(usize))
        )
        .get_matches();

    let n_points = *matches.get_one::<usize>("points").unwrap();

    utl::create_nodes(n_points);
}
