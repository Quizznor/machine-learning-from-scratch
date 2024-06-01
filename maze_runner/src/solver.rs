use ndarray::Array2;
use ndarray::Array1;
use ndarray::Axis;
use ndarray::s;

pub fn solve_graph(adjacency_matrix : Array2<f64>) -> Array1<usize> {
    
    // diagnostics/runtime performance?
    // your preferred algorithm here
    return dijkstra_solve(adjacency_matrix);
}

fn dijkstra_solve(adjacency_matrix : Array2<f64>) -> Array1<usize> {
    // IDEA: write paths as integers into 2D array
    // e.g. path 1 -> 2 -> 5 -> 6 becomes 6521

    let n_vertices = adjacency_matrix.len_of(Axis(0));
    
    let mut not_visited : Vec<usize> = (0..n_vertices).collect();
    let mut distances = Array1::<f64>::from_elem(n_vertices, f64::INFINITY);
    let mut paths = Array1::<usize>::zeros(distances.raw_dim());

    let (mut depth, mut next_node, mut curr_distance) = (0, 0, 0.);

    while depth < n_vertices || not_visited.len() != 0 {

        // get connections at next node
        let connections = adjacency_matrix.slice(s![next_node, ..]);
        not_visited.retain(|x| *x != next_node);

        let mut shortest_path_from_current_node = &f64::INFINITY;

        for (n, vertex) in connections.iter().enumerate() {
            if vertex == &0. {continue;}

            // check if current path is closer than last
            if (curr_distance + vertex) > distances[n] {continue;}

            // connection is closer, update distances + path
            paths[n] = depth * next_node + paths[next_node];       // does this work?
            distances[n] = curr_distance + vertex;

            if vertex < &shortest_path_from_current_node {
                (next_node, shortest_path_from_current_node) = (n, vertex);
            }

        }
        
        println!("{}", next_node);
        // println!("{}", not_visited.len());
        curr_distance += connections[next_node];
        depth += 1;
    
        // update paths
        // update distances
        // get next node
    }

    println!("{}", not_visited.len());
    println!("{:?}", not_visited);

    return paths;
}