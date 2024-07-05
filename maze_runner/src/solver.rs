use ndarray::ArrayView1;
use ndarray::Array2;
use ndarray::Array1;
use std::io::Error;
use std::io::Write;
use std::fs::File;
use ndarray::Axis;
use ndarray::s;

pub fn solve_graph(adjacency_matrix : Array2<f64>, method : &str) -> (Vec<String>, Array1<f64>) {
    
    // diagnostics/runtime performance?
    match method {
        "dijkstra" => {
            return dijkstra_solve(adjacency_matrix);
        }

        _ => {
            panic! ("Need a valid solve method");
        }
    }
}

pub fn write_to_file(paths : Vec<String>, distances : Array1<f64>, target : &str) -> Result<(), Error> {

    let mut file = File::create(target)?;
    let line_iterator = paths.iter().zip(distances.iter());

    for (path, distance) in line_iterator {
        write!(file, "{}, {}\n", path, distance)?;
    }
    
    return Ok(());
}

fn dijkstra_solve(adjacency_matrix : Array2<f64>) -> (Vec<String>, Array1<f64>) {

    let n_vertices : usize = adjacency_matrix.len_of(Axis(0));
    let mut not_visited : Vec<usize> = (0..n_vertices).collect();
    let mut distances = Array1::<f64>::from_elem(n_vertices, f64::INFINITY);
    let mut paths = vec!["".to_string(); n_vertices];
    
    // needed for initialization
    paths[0] = "0".to_string();
    distances[0] = 0.0;

    let mut this_node: usize = 0;

    while not_visited.len() != 0 {
        
        // get connections at next node
        let connections: ArrayView1<f64> = adjacency_matrix.slice(s![this_node, ..]);        // get this_node connections
        not_visited.retain(|x| *x != this_node);                    // mark this_node as visited

        // print!("Connections at node {}: (", this_node);
        for (n, this_length) in connections.iter().enumerate() {
            if *this_length == 0. {continue;}

            // print!(" {}", n);
            if distances[this_node] + this_length < distances[n] {
                distances[n] = distances[this_node] + connections[n];// update distances
                paths[n] = format!("{},{}", paths[this_node], n);   // save path info 
            }
        }
        // print!(")\n");
        
        // find next node
        let mut min_distance: f64 = f64::INFINITY;
        for &n in not_visited.iter() {
            if distances[n] < min_distance {
                min_distance = distances[n];
                this_node = n;
            }
        }     
    }

    return (paths, distances);
}