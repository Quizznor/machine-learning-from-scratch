use ndarray::{s, Array2};
use ndarray::array;

pub fn sobel_operator(gray_image: Array2<u8>) -> Array2<u8> {

    println!("searching for object edges...");

    let x_filter: Array2<f64> = array![
        [ -5., -4.,  0.,  4.,  5.], 
        [ -8.,-10.,  0., 10.,  8.],
        [-10.,-20.,  0., 20., 10.],
        [ -8.,-10.,  0., 10.,  8.],
        [ -5., -4.,  0.,  4.,  5.]   
    ]; 

    let y_filter: Array2<f64> = array![
        [ -5., -8.,-10., -8., -5.],
        [ -4.,-10.,-20.,-10., -4.],
        [  0.,  0.,  0.,  0.,  0.],
        [  4., 10., 20., 10.,  4.],
        [  5.,  8., 10.,  8.,  5.],
    ];

    let (width, height) = gray_image.dim();
    let mut edges = Array2::<u8>::zeros((width - 3, height - 3));

    // TODO: look for a better way to do this

    for iw in 2..width-3 {
        for ih in 2..height-3 {

            let surrounding = gray_image.slice(s![iw-2..iw+3, ih-2..ih+3]);
            let matrix = surrounding.mapv(|x| f64::from(x));

            let diff_x = (x_filter.dot(&matrix.t()) / 240.).sum();
            let diff_y = (matrix.t().dot(&y_filter) / 240.).sum();

            edges[[iw, ih]] = ((diff_x.powf(2.) + diff_y.powf(2.))).powf(0.5) as u8;
        }
    }

    edges
}
