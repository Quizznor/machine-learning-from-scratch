mod generator;
use solver;
use ndarray_npy::write_npy;

fn main() {

    let n_vertices : usize = 100;
    let connectivity : f64 = 1e-7;

    let (graph, adjacency_matrix) = generator::make_graph(n_vertices, connectivity);

    write_npy("positions.npy", &graph)
        .expect("Failed to write positions to file =(");
    write_npy("distances.npy", &adjacency_matrix)
        .expect("Failed to write distances to file =(");
    
    println!("{:.3}", adjacency_matrix);

    // design goal
    //     -> generate weighted graph using generator.rs -- DONE
    //     -> solve for shortest Path from A to B...
    //     -> have python or something plot the result -- DONE
    
}



