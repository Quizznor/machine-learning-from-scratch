use ndarray_rand::{rand_distr::Standard, RandomExt};
use ndarray_rand::rand_distr::num_traits::Pow;
use rand::prelude::SliceRandom;
use ndarray::ArrayView1;
use ndarray::Array2;
use ndarray::s;

pub fn make_graph(n_vertices : usize, connectivity : f64, method : &str) -> (Array2<f64>, Array2<f64>) {

    // let mut rng = Isaac64Rng::seed_from_u64(69);
    let mut rng = rand::thread_rng();
    let positions : Array2<f64, > = Array2::<f64>::random_using((n_vertices, 2), Standard, &mut rng);
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

        _ => {todo!("method not implemented");}
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

    let n_vertices : usize = positions.raw_dim()[0];
    let mut new_adjacency_matrix = Array2::<f64>::zeros((n_vertices, n_vertices));

    fn construct_circumcircle(point1 : ArrayView1<f64>,
                              point2 : ArrayView1<f64>,
                              point3 : ArrayView1<f64>) -> (f64, f64, f64) {

            let (x1, y1) = (point1[0], point1[1]);
            let (x2, y2) = (point2[0], point2[1]);
            let (x3, y3) = (point3[0], point3[1]);

            let d = (x1 - x3) * (y2 - y3) - (x2 - x3) * (y1 - y3);
            let center_x = (0.5 * (y2 - y3) * ((x1 - x3) * (x1 + x3) + (y1 - y3) * (y1 + y3))
                         - 0.5 * (y1 - y3) * ((x2 - x3) * (x2 + x3) + (y2 - y3) * (y2 + y3))) / d;
            let center_y = (0.5 * (x1 - x3) * ((x2 - x3) * (x2 + x3) + (y2 - y3) * (y2 + y3))
                         - 0.5 * (x2 - x3) * ((x1 - x3) * (x1 + x3) + (y1 - y3) * (y1 + y3))) / d;
            let radius: f64 = (((x1 - center_x).pow(2.) + (y1 - center_y).pow(2.)) as f64).pow(0.5);

            return (center_x, center_y, radius);
    }

    fn point_lies_in_circle(point : ArrayView1<f64>, 
        (center_x, center_y, radius) : &(f64, f64, f64)) -> bool {
            return (((point[0] - center_x).pow(2.) + (point[1] - center_y).pow(2.)) as f64).pow(0.5) > *radius;
    }
    
    let n_vertices : usize = positions.raw_dim()[0];

    // O(n⁴) go brr
    for p1 in 0..n_vertices {
        let point1 = positions.slice(s![p1, ..]);

        for p2 in p1..n_vertices {
            if p1 == p2 {continue;}
            let point2 = positions.slice(s![p2, ..]);

            for p3 in 0..n_vertices {
                if p3 == p2 || p3 == p1 {continue;}
                let point3 = positions.slice(s![p3, ..]);
                let circumcircle = construct_circumcircle(point1, point2, point3);  

                let mut point_outside = true;
                for p4 in 0..n_vertices {
                    if point_outside && p4 != p3 && p4 != p2 && p4 != p1 {
                        point_outside = point_lies_in_circle(positions.slice(s![p4, ..]), &circumcircle);
                    }
                }

                if point_outside {
                    new_adjacency_matrix[[p1, p2]] = adjacency_matrix[[p1, p2]];
                    new_adjacency_matrix[[p2, p1]] = adjacency_matrix[[p2, p1]];
                    new_adjacency_matrix[[p1, p3]] = adjacency_matrix[[p1, p3]];
                    new_adjacency_matrix[[p3, p1]] = adjacency_matrix[[p3, p1]];
                    new_adjacency_matrix[[p2, p3]] = adjacency_matrix[[p2, p3]];
                    new_adjacency_matrix[[p3, p2]] = adjacency_matrix[[p3, p2]];
                }
            }
        }
    }

    return new_adjacency_matrix;
}