use rand::Rng;
use ndarray::Array2;
use ndarray_rand::{rand_distr::Standard, RandomExt};
use ndarray_rand::rand_distr::num_traits::Pow;

pub fn make_graph(n_vertices : usize, connectivity : f64) -> (Array2<f64>, Array2<f64>) {

    let positions = Array2::<f64>::random((n_vertices, 2), Standard);
    let mut adjacency_matrix = Array2::<f64>::zeros((n_vertices, n_vertices));

    // calculate distances into adjacency matrix
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
    let mut rng = rand::thread_rng();
    for _ in 1..(( (1. - connectivity) * (n_vertices.pow(2) as f64 )) as i64 ) {
        let i : usize = rng.gen_range(0..n_vertices.pow(2));
        adjacency_matrix[[i / n_vertices, i % n_vertices]] = 0.;
        adjacency_matrix[[i % n_vertices, i / n_vertices]] = 0.;
    }

    return (positions, adjacency_matrix);

    // return Array2::<u8>::random((n_vertices, n_vertices), Standard);
}

// pub fn multi_dimensional_scaling(graph : Array2, ) -> Array2 {
//     let Z : Array2<f64> = Array2::zeros((2, graph.shape().get(0)));

//     println("{:.1}", Z);
// }