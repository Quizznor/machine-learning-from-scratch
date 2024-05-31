mod generator;
// use solver;

use rand::Rng;

fn main() {
    let n_vertices : usize = 20;
    let connectivity : f64 = 0.5;
    let size = n_vertices * n_vertices;

    let mut graph = generator::make_graph(size);

    // graphical representation from full graph?

    // deleting some connections to make the graph more interesting
    let mut rng = rand::thread_rng();
    for _ in 1..( (1. - connectivity) * (size as f64) ) as i64 {
        let i : usize = rng.gen_range(0..size);
        graph[[i / n_vertices, i % n_vertices]] = 0;
    }


    // design goal
    //     -> generate weighted graph using generator.rs -- DONE
    //     -> solve for shortest Path from A to B...
    //     -> have python or something plot the result
    
}



