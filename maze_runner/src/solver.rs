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
    paths[0] = "-1".to_string();
    distances[0] = 0.0;

    let mut this_node = 0;
    let mut last_node = 0;

    while not_visited.len() != 0 {
        
        // get connections at next node
        let connections = adjacency_matrix.slice(s![this_node, ..]);        // get this_node connections
        not_visited.retain(|x| *x != this_node);                            // mark this_node as visited

        let mut min_distance = f64::INFINITY;
        for &n in not_visited.iter() {

            if connections[n] != 0. && (distances[this_node] + connections[n]) < distances[n] {
                distances[n] = distances[this_node] + connections[n];       // update distances
                paths[n] = format!("{},{}", paths[last_node], this_node);   // save path info 
            }
        }

        // find next node + save last node
        last_node = this_node;
        for &n in not_visited.iter() {
            if distances[n] < min_distance {
                min_distance = distances[n];
                this_node = n;
            }
        }     
    }

    println!("{:?}", paths);

    return paths;
}