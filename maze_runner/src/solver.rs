use ndarray::Array2;
use ndarray::Array1;
use std::io::Error;
use std::io::Write;
use std::fs::File;
use ndarray::Axis;
use ndarray::s;

pub fn solve_graph(adjacency_matrix : Array2<f64>, method : &str) -> Vec<String> {
    
    // diagnostics/runtime performance?
    match method {
        "dijkstra" => {
            return dijkstra_solve(adjacency_matrix);
        }

        _ => {
            return vec!["".to_string(); adjacency_matrix.len_of(Axis(0))];
        }
    }
}

pub fn write_to_file(min_distance_paths : Vec<String>, target : &str) -> Result<(), Error> {

    let mut file = File::create(target)?;

    for line in min_distance_paths.iter() {
        write!(file, "{}\n", line)?;
    }
    
    return Ok(());
}

fn dijkstra_solve(adjacency_matrix : Array2<f64>) -> Vec<String> {

    let n_vertices : usize = adjacency_matrix.len_of(Axis(0));
    let mut not_visited : Vec<usize> = (0..n_vertices).collect();
    let mut distances = Array1::<f64>::from_elem(n_vertices, f64::INFINITY);
    let mut paths = vec!["".to_string(); n_vertices];
    
    // needed for initialization
    let mut next_node = 0;
    distances[0] = 0.0;

    while not_visited.len() != 0 {

        // get connections at next node
        let connections = adjacency_matrix.slice(s![next_node, ..]);    // get next_node connections
        not_visited.retain(|x| *x != next_node);                        // mark next_node as visited

        let mut min_distance = f64::INFINITY;
        for &n in not_visited.iter() {

            if connections[n] != 0. && (distances[next_node] + connections[n]) < distances[n] {
                distances[n] = distances[next_node] + connections[n];   // update distances
                paths[n] = format!("{},{}", paths[n], next_node);       // save path info 
            }

            // find next_node
            if distances[n] < min_distance {
                min_distance = distances[n];
                next_node = n;
            }     
        }
    }

    println!("{:?}", paths);

    return paths;
}