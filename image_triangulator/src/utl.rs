use ndarray::{s, Array2};
use ndarray::{array, Array1};
use ::image::GrayImage;
use image::ImageFormat::Png;
use std::{io, io::Write};
use delaunator::{triangulate, Point};
use plotters::prelude::*;
use rand::Rng;

pub fn save_gray_image(target: &Array2<u8>, name: &str) -> () {

    let (width, height) = target.dim();
    let raw_vector = target.clone().into_raw_vec();

    GrayImage::from_raw(width as u32, height as u32, raw_vector)
        .expect("Couldn't build image from provided arguments")
        .save_with_format("out/".to_owned() + name + ".png", Png)
        .expect("Couldn't save image from provided arguments")
}

pub fn save_mesh_image(mesh_info: &(Vec<Point>, Vec<usize>), dimensions: (usize, usize), name: &str) -> () {

    let (points, triangles) = mesh_info;

    let image_name = "out/".to_owned() + name + ".png";
    let image = BitMapBackend::new(&image_name, (dimensions.0 as u32, dimensions.1 as u32)).into_drawing_area();
    image.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&image)
        .x_label_area_size(0)
        .y_label_area_size(0)
        .build_cartesian_2d(0.0..dimensions.0 as f64, 0.0..dimensions.1 as f64)
        .unwrap();

    for i in (0..triangles.len()).step_by(3) {

        let mut triangle = triangles[i..i+3].iter().map(|p| (points[*p].x, points[*p].y)).collect::<Vec<(f64, f64)>>();
        triangle.push((points[triangles[i]].x, points[triangles[i]].y));

        chart.draw_series(LineSeries::new(
            triangle,
            &RED,
        )).unwrap();
    }

}



























pub fn sobel_operator(gray_image: Array2<u8>) -> Array2<u8> {

    print!("searching for object edges...");
    let _ = io::stdout().flush();

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

    println!("DONE");

    edges
}

pub fn create_mesh(source: Array2<u8>, n_points: usize) -> (Vec<Point>, Vec<usize>) {

    print!("try building delauney mesh...");
    let _ = io::stdout().flush();

    let mut rng = rand::thread_rng();
    let (pdf_x, pdf_y) = make_marginal_pdfs(source);
    let array_shape = (pdf_x.len(), pdf_y.len());

    // perform rejection sampling
    let mut points = Vec::<Point>::new();
    
    loop {

        let (mut x, mut y): (usize, usize);
        loop {
            x = rng.gen_range(0..array_shape.0);
            if rng.gen_range(0.0..1.0) <= pdf_x[[x]] {break;}
        }

        loop {
            y = rng.gen_range(0..array_shape.1);
            if rng.gen_range(0.0..1.0) <= pdf_y[[y]] {break;}
        }

        points.push(Point{ x: x as f64, y: y as f64 });
        if points.len() == n_points {break;}
    }

    let triangulation = triangulate(&points);
    println!("DONE");

    (points, triangulation.triangles)
}

fn make_marginal_pdfs(source: Array2<u8>) -> (Array1<f64>, Array1<f64>) {

    let weight: f64 = source.mapv(|x| f64::from(x)).sum();
    let array_shape = source.dim();

    let pdf_x = (0..array_shape.0).map(|x| source.column(x).mapv(|x| f64::from(x)).sum() / weight).collect::<Array1<f64>>();
    let pdf_y = (0..array_shape.1).map(|x| source.row(x).mapv(|x| f64::from(x)).sum() / weight).collect::<Array1<f64>>();

    (pdf_x, pdf_y)
}