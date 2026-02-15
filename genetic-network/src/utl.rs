use ndarray::{Array, Array2};
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::{Uniform, Exp};
use ndarray_rand::rand::rng;

pub fn create_nodes(n_points: usize) {

    // need per node: x, y, density (spawn size), popularity (destination probability)
    // let node_positions: ArrayBase<OwnedRepr<f64>> = Array::random((n_points, 2), Uniform::new(0., 10.));

    let position_rng = Uniform::new(0.0, 10.0).unwrap();
    let pop_dense_rng = Exp::new(4.).unwrap();

    let positions = Array2::random_using((n_points, 2), position_rng, &mut rng());
    let mut density = Array::random_using(n_points, pop_dense_rng, &mut rng());
    let mut populations = Array::random_using(n_points, pop_dense_rng, &mut rng());

    // stack node parameters?
    // return nodes
}