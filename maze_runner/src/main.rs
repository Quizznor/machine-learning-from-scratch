use ndarray_npy::write_npy;
mod generator;
mod solver;

fn main() {

    const N_VERTICES : usize = 100;
    const CONNECTIVITY : f64 = 0.3;               // mustn't be too low to ensure a FC graph 
    
    const CREATE_METHOD : &str = "delauney";      // method used to construct graph vertices   
    const SOLVE_METHOD : &str = "dijkstra";       // method used to calculate shortest path

    let (graph, adjacency_matrix) = generator::make_graph(N_VERTICES, CONNECTIVITY, CREATE_METHOD);

    write_npy("positions.npy", &graph)
        .expect("Failed to write positions to file =(");
    write_npy("distances.npy", &adjacency_matrix)
        .expect("Failed to write distances to file =(");
    
    let (paths, distances) = solver::solve_graph(adjacency_matrix, SOLVE_METHOD);
    let _ = solver::write_to_file(paths, distances, "minimum_distance_paths.txt");
}



