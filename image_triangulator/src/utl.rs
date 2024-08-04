use ::image::GrayImage;
use delaunator::{triangulate, Point};
use image::ImageFormat::Png;
use ndarray::{array, Array1};
use ndarray::{s, Array2};
use plotters::prelude::*;
use rand::{thread_rng, Rng};

use crate::picture::Picture;
use crate::triangle::Triangle;

#[macro_export]
macro_rules! runtime {
    ($fctn:expr, $name:literal) => [{

        let start = ::std::time::Instant::now();
        let rv = $fctn;
        let duration = start.elapsed();

        println!("{:.<40}: DONE! ({:#?})", $name, duration);

        rv
    }];
}

pub fn save_gray_image(target: &Array2<u8>, name: &str) -> () {
    let (width, height) = target.dim();
    let raw_vector = target.clone().into_raw_vec();

    GrayImage::from_raw(width as u32, height as u32, raw_vector)
        .expect("Couldn't build image from provided arguments")
        .save_with_format("out/".to_owned() + name + ".png", Png)
        .expect("Couldn't save image from provided arguments")
}

pub fn save_mesh_image(
    mesh: &Vec<Triangle>,
    dimensions: (usize, usize),
    name: &str,
) -> () {

    let image_name = "out/".to_owned() + name + ".png";
    let image = BitMapBackend::new(&image_name, (dimensions.0 as u32, dimensions.1 as u32))
        .into_drawing_area();
    image.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&image)
        .x_label_area_size(0)
        .y_label_area_size(0)
        .build_cartesian_2d(0.0..dimensions.0 as f64, 0.0..dimensions.1 as f64)
        .unwrap();

    for tt in mesh.iter() {
        chart.draw_series(LineSeries::new(tt.get_path(), &RED)).unwrap();
    }
}

pub fn sobel_operator(gray_image: Array2<u8>) -> Array2<u8> {

    let x_filter: Array2<f64> = array![
        [-5., -4., 0., 4., 5.],
        [-8., -10., 0., 10., 8.],
        [-10., -20., 0., 20., 10.],
        [-8., -10., 0., 10., 8.],
        [-5., -4., 0., 4., 5.]
    ];

    let y_filter: Array2<f64> = array![
        [-5., -8., -10., -8., -5.],
        [-4., -10., -20., -10., -4.],
        [0., 0., 0., 0., 0.],
        [4., 10., 20., 10., 4.],
        [5., 8., 10., 8., 5.],
    ];

    let (width, height) = gray_image.dim();
    let mut edges = Array2::<u8>::zeros((width - 3, height - 3));

    // TODO: look for a better way to do this
    for iw in 2..width - 3 {
        for ih in 2..height - 3 {
            let surrounding = gray_image.slice(s![iw - 2..iw + 3, ih - 2..ih + 3]);
            let matrix = surrounding.mapv(|x| f64::from(x));

            let diff_x = (x_filter.dot(&matrix.t()) / 240.).sum();
            let diff_y = (matrix.t().dot(&y_filter) / 240.).sum();

            edges[[iw, ih]] = (diff_x.powf(2.) + diff_y.powf(2.)).powf(0.5) as u8;
        }
    }

    edges
}

pub fn create_mesh(source: Array2<u8>, n_points: usize) -> Vec<Triangle> {

    let mut rng = thread_rng();
    let (cdf_x, cdf_y) = get_cdf(source);

    // perform inverse transform sampling
    let mut points = Vec::<Point>::new();
    loop {

        // generate random x
        let x_chance: f64 = rng.gen();
        let mut x = 0;

        loop {
            if cdf_x[x] > x_chance {break;}
            x += 1;
        }

        // generate random y
        let y_chance: f64 = rng.gen();
        let mut y = 0;

        loop {
            if cdf_y[y] > y_chance {break;}
            y += 1;
        }
        
        points.push(Point {
            x: x as f64,
            y: y as f64,
        });

        if points.len() == n_points {break;}
    }

    let triangulation = triangulate(&points).triangles;
    let mut triangles = Vec::<Triangle>::new();

    for triplet in triangulation.chunks(3).into_iter() {
        let p1 = Point{ x: points[triplet[0]].x, y: points[triplet[0]].y };
        let p2 = Point{ x: points[triplet[1]].x, y: points[triplet[1]].y };
        let p3 = Point{ x: points[triplet[2]].x, y: points[triplet[2]].y };
        triangles.push(Triangle::new(p1, p2, p3))
    }

    triangles
}

pub fn calculate_colors(
    original_image: &Picture,
    mesh: Vec<Triangle>
) -> (Array2<u8>, Array2<usize>) {

    let mut colors = Array2::<u64>::zeros((mesh.len(), 3));
    let mut triangle_table = Array2::<usize>::zeros(original_image.dimension());
    let mut triangle_pixel_counts = Array1::<usize>::ones(mesh.len());

    let (width, height) = original_image.dimension();

    for w in 0..width {
        for h in 0..height {
            for (t, triangle) in mesh.iter().enumerate() {
                
                if triangle.contains(Point{ x: w as f64, y: h as f64 }) {

                    let pixel = original_image.get_pixel(w, h);
                    let (r, g, b) = (pixel[[0]], pixel[[1]], pixel[[2]]);

                    triangle_table[[w, h]] = t;
                    triangle_pixel_counts[t] += 1;
    
                    colors[[t, 0]] += r as u64;
                    colors[[t, 1]] += g as u64;
                    colors[[t, 2]] += b as u64;
                }
            }
        }
    }

    for i in 0..mesh.len() {
        colors[[i, 0]] /= triangle_pixel_counts[[i]] as u64;
        colors[[i, 1]] /= triangle_pixel_counts[[i]] as u64;
        colors[[i, 2]] /= triangle_pixel_counts[[i]] as u64;
    }

    (
        colors.mapv(|x| u8::try_from(x).expect(&format!("{} encountered", x))),
        triangle_table,
    )
}

fn get_cdf(source: Array2<u8>) -> (Array1<f64>, Array1<f64>) {
    let weight: f64 = source.mapv(|x| f64::from(x)).sum();
    let array_shape = source.dim();

    let mut cdf_x = Array1::<f64>::zeros(array_shape.0);
    let mut cdf_y = Array1::<f64>::zeros(array_shape.1);

    cdf_x[0] = source.column(0).mapv(|x| f64::from(x)).sum();
    cdf_y[0] = source.row(0).mapv(|x| f64::from(x)).sum();

    // calculate x cdf
    for i in 1..array_shape.0 {
        cdf_x[i] = source.column(i).mapv(|x| f64::from(x)).sum();
        cdf_x[i] += cdf_x[i-1];
    }

    // calculate x cdf
    for i in 1..array_shape.1 {
        cdf_y[i] = source.row(i).mapv(|x| f64::from(x)).sum();
        cdf_y[i] += cdf_y[i-1];
    }

    (cdf_x / weight, cdf_y / weight)
}