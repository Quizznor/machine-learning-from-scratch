use ndarray_rand::{rand_distr::Standard, RandomExt};
use ndarray_rand::rand_distr::num_traits::Pow;
use rand::prelude::SliceRandom;
use ndarray::Array2;
use ndarray::s;

pub fn make_graph(n_vertices : usize, connectivity : f64, method : &str) -> (Array2<f64>, Array2<f64>) {

    let positions : Array2<f64, > = Array2::<f64>::random((n_vertices, 2), Standard);
    let mut adjacency_matrix = make_all_connections(&positions);

    match method {
        "chaotic" => {
            adjacency_matrix = make_chaotic_connections(adjacency_matrix, &positions, connectivity);
        }
        "planar" => {
            adjacency_matrix = make_planar_connections(adjacency_matrix, &positions);
        }
        "delauney" => {
            adjacency_matrix = make_delauney_connections(adjacency_matrix, &positions);
        }

        _ => {println!("method not implemented");}
    }

    return (positions, adjacency_matrix);
}

fn make_all_connections(positions : &Array2<f64>) -> Array2<f64> {

    let n_vertices : usize = positions.raw_dim()[0];
    let mut adjacency_matrix = Array2::<f64>::zeros((n_vertices, n_vertices));

    // calculate distances into adjacency_matrix
    // there must be a better way to do this...
    for i in 0..n_vertices {
        let xi = positions[[i, 0]];
        let yi = positions[[i, 1]];

        for j in i..n_vertices {
            let xj = positions[[j, 0]];
            let yj = positions[[j, 1]];

            let distance : f64 = ( ((xi - xj).pow(2) + (yi - yj).pow(2)) as f64 ).pow(1./2.);
            adjacency_matrix[[i, j]] = distance;
            adjacency_matrix[[j, i]] = distance;
        }
    }

    return adjacency_matrix;
}

fn make_chaotic_connections(mut adjacency_matrix : Array2<f64>, 
    positions : &Array2<f64>, connectivity : f64) -> Array2<f64> {

        let n_vertices : usize = positions.raw_dim()[0];

        // deleting some connections to make the graph more interesting
        let mut temp : Vec<usize> = (0..n_vertices.pow(2)).collect();
        temp.shuffle(&mut rand::thread_rng());
        let indices_to_delete = &temp[0..(( (1. - connectivity) * (n_vertices.pow(2) as f64 )) as usize )];

        for index in indices_to_delete {

            // we have to ensure every vertex has at least one connection
            // makes it more probable that the graph is fully connected
            {
                let (vertex_a, vertex_b) = (index / n_vertices, index % n_vertices);
            
                // checking connectivity in start point
                let connections_a = adjacency_matrix.slice(s![vertex_a, ..]);
                if connections_a.sum() == adjacency_matrix[[vertex_a, vertex_b]] {continue;}
                
                // checking connectivity in end point
                let connections_b = adjacency_matrix.slice(s![vertex_b, ..]);
                if connections_b.sum() == adjacency_matrix[[vertex_a, vertex_b]] {continue;}
            }

            adjacency_matrix[[index / n_vertices, index % n_vertices]] = 0.;
            adjacency_matrix[[index % n_vertices, index / n_vertices]] = 0.;
        }

        return adjacency_matrix;
}

fn make_planar_connections(mut adjacency_matrix : Array2<f64>, 
    positions : &Array2<f64>) -> Array2<f64> {
    
        fn ccw(a : usize, b : usize, c : usize, 
            positions : &Array2<f64>) -> bool {
                let (a_x, a_y) = (positions[[a, 0]], positions[[a, 1]]);
                let (b_x, b_y) = (positions[[b, 0]], positions[[b, 1]]);
                let (c_x, c_y) = (positions[[c, 0]], positions[[c, 1]]);
                return (c_y - a_y) * (b_x - a_x) > (b_y - a_y) * (c_x - a_x);
        }

        let n_vertices : usize = positions.raw_dim()[0];

        for i in 0..n_vertices {
            for j in 0..i {
                if adjacency_matrix[[i, j]] == 0. {continue;}

                for k in 0..n_vertices {
                    for l in 0..k {
                        if i == k && j == l {continue;}
                        if i == l && j == k {continue;}

                        if ccw(i,k,l, &positions) == ccw(j,k,l, &positions) {continue;}
                        if ccw(i,j,k, &positions) == ccw(i,j,l, &positions) {continue;}

                        // the two lines are intersecting if we have reached 
                        // this part of the code... delete one of them.
                        adjacency_matrix[[k, l]] = 0.;
                        adjacency_matrix[[l, k]] = 0.;
                    }
                }
            }
        }

        return adjacency_matrix;
}

fn make_delauney_connections(mut adjacency_matrix : Array2<f64>, 
    positions : &Array2<f64>, ) -> Array2<f64> {
        
    let n_vertices : usize = adjacency_matrix.len();

    println!("{}, implement me!", n_vertices);

    // TODO: implement this
    //  - make 3D projection
    //  - build convex hull
    //  - project to 2D
    //  - tada
    
    return adjacency_matrix;
}