mod generator;
mod solver;
use ndarray_npy::write_npy;

fn main() {

    let n_vertices : usize = 10;    // the way the code is structured we can only do 10 =(
    let connectivity : f64 = 1.;   // mustn't be too low to ensure a fully connected graph 

    let (graph, adjacency_matrix) = generator::make_graph(n_vertices, connectivity);

    write_npy("positions.npy", &graph)
        .expect("Failed to write positions to file =(");
    write_npy("distances.npy", &adjacency_matrix)
        .expect("Failed to write distances to file =(");
    
    let min_distance_paths = solver::solve_graph(adjacency_matrix);


    // println!("{:.3}", mind_distance_paths);

    // design goal
    //     -> generate weighted graph using generator.rs -- DONE
    //     -> solve for shortest Path from A to B...
    //     -> have python or something plot the result -- DONE
    
}



