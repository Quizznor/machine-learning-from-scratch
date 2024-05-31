use ndarray::Array2;
use ndarray_rand::{rand_distr::Standard, RandomExt};

pub fn make_graph(n_vertices : usize) -> Array2<u8> {
    return Array2::<u8>::random((n_vertices, n_vertices), Standard);
}

