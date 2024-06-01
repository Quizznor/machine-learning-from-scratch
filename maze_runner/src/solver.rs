use ndarray::Array2;

pub fn solve_graph(adjacency_matrix : Array2<f64>) -> Array2<i64> {
    
    // diagnostics/runtime performance?
    // your preferred algorithm here
    return dijkstra_solve(adjacency_matrix);
}

fn dijkstra_solve(adjacency_matrix : Array2<f64>) -> Array2<i64> {
    // IDEA: write paths as integers into 2D array
    // e.g. path 1 -> 2 -> 5 -> 6 becomes 1256

    return Array2::<i64>::zeros(adjacency_matrix.raw_dim());

}