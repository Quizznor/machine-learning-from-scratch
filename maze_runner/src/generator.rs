use ndarray::s;
use ndarray::Array2;
use rand::prelude::SliceRandom;
use ndarray_rand::{rand_distr::Standard, RandomExt};
use ndarray_rand::rand_distr::num_traits::Pow;

pub fn make_graph(n_vertices : usize, connectivity : f64) -> (Array2<f64>, Array2<f64>) {

    let positions = Array2::<f64>::random((n_vertices, 2), Standard);
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

    // deleting some connections to make the graph more interesting
    let mut temp: Vec<usize> = (0..n_vertices.pow(2)).collect();
    temp.shuffle(&mut rand::thread_rng());
    let indices_to_delete = &temp[0..(( (1. - connectivity) * (n_vertices.pow(2) as f64 )) as usize )];

    for index in indices_to_delete {

        // we have to ensure graph is still fully connected
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

    return (positions, adjacency_matrix);

    // return Array2::<u8>::random((n_vertices, n_vertices), Standard);
}

// pub fn multi_dimensional_scaling(graph : Array2, ) -> Array2 {
//     let Z : Array2<f64> = Array2::zeros((2, graph.shape().get(0)));

//     println("{:.1}", Z);
// }