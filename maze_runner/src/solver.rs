use ndarray::Array2;
use ndarray::Array1;
use ndarray::Axis;
use ndarray::s;

pub fn solve_graph(adjacency_matrix : Array2<f64>) -> Vec<String> {
    
    // diagnostics/runtime performance?
    // your preferred algorithm here
    return dijkstra_solve(adjacency_matrix);
}

fn dijkstra_solve(adjacency_matrix : Array2<f64>) -> Vec<String> {

    let n_vertices : usize = adjacency_matrix.len_of(Axis(0));
    let mut not_visited : Vec<usize> = (0..n_vertices).collect();
    let mut distances = Array1::<f64>::from_elem(n_vertices, f64::INFINITY);
    let mut paths = vec!["".to_string(); n_vertices];

    let (mut depth, mut next_node, mut curr_distance) = (0, 0, 0.);

    while not_visited.len() != 0 {

        // get connections at next node
        let connections = adjacency_matrix.slice(s![next_node, ..]);
        not_visited.retain(|x| *x != next_node);

        let mut shortest_path_from_current_node = f64::INFINITY;
        for &n in not_visited.iter() {
            if connections[n] == 0. {continue;}

            // check if current path is closer than last
            if (curr_distance + connections[n]) > distances[n] {continue;}

            // connection is closer, update distances + path
            // paths[n] = depth * next_node + paths[next_node];       // does this work?
            distances[n] = curr_distance + connections[n];
            println!("{}", distances[n]);

            if connections[n] < shortest_path_from_current_node {
                (next_node, shortest_path_from_current_node) = (n, connections[n]);
            }        
        }

        // println!("{}", not_visited.len());
        curr_distance += connections[next_node];
        depth += 1;
        // println!("{}", connections);
        // println!("{}", adjacency_matrix.slice(s![next_node, ..]));
        
        // update paths
        // update distances
        // get next node
    }

    return paths;
}