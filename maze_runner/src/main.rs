use ndarray_npy::write_npy;
mod generator;
mod solver;

fn main() {

    const N_VERTICES : usize = 40;
    const CONNECTIVITY : f64 = 0.7;    // mustn't be too low to ensure a fully connected graph 
    
    const CREATE_METHOD : &str = "delauney";      // method used to construct graph vertices   
    const SOLVE_METHOD : &str = "dijkstra";       // method used to calculate shortest path

    let (graph, adjacency_matrix) = generator::make_graph(N_VERTICES, CONNECTIVITY, CREATE_METHOD);

    write_npy("positions.npy", &graph)
        .expect("Failed to write positions to file =(");
    write_npy("distances.npy", &adjacency_matrix)
        .expect("Failed to write distances to file =(");
    
    // let (paths, distances) = solver::solve_graph(adjacency_matrix, SOLVE_METHOD);
    // let _ = solver::write_to_file(paths, distances, "minimum_distance_paths.txt");

    // design goal
    //     -> generate weighted graph using generator.rs -- DONE
    //     -> solve for shortest Path from A to B... -- DONE?
    //     -> have python or something plot the result -- DONE
    
}



