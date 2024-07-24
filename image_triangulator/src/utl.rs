use ::image::GrayImage;
use delaunator::{triangulate, Point};
use image::ImageFormat::Png;
use ndarray::{array, Array1};
use ndarray::{s, Array2};
use plotters::prelude::*;
use rand::Rng;
use std::{io, io::Write};

use crate::picture::Picture;

pub fn save_gray_image(target: &Array2<u8>, name: &str) -> () {
    let (width, height) = target.dim();
    let raw_vector = target.clone().into_raw_vec();

    GrayImage::from_raw(width as u32, height as u32, raw_vector)
        .expect("Couldn't build image from provided arguments")
        .save_with_format("out/".to_owned() + name + ".png", Png)
        .expect("Couldn't save image from provided arguments")
}

pub fn save_mesh_image(
    mesh_info: &(Vec<Point>, Vec<Vec<usize>>),
    dimensions: (usize, usize),
    name: &str,
) -> () {
    let (points, triangles) = mesh_info;

    let image_name = "out/".to_owned() + name + ".png";
    let image = BitMapBackend::new(&image_name, (dimensions.0 as u32, dimensions.1 as u32))
        .into_drawing_area();
    image.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&image)
        .x_label_area_size(0)
        .y_label_area_size(0)
        .build_cartesian_2d(0.0..dimensions.0 as f64, 0.0..dimensions.1 as f64)
        .unwrap();

    for tt in triangles.iter() {
        let mut triangle = tt
            .iter()
            .map(|p| (points[*p].x, points[*p].y))
            .collect::<Vec<(f64, f64)>>();
        triangle.push((points[tt[0]].x, points[tt[0]].y));

        chart.draw_series(LineSeries::new(triangle, &RED)).unwrap();
    }
}

pub fn sobel_operator(gray_image: Array2<u8>) -> Array2<u8> {
    print!("searching for object edges...");
    let _ = io::stdout().flush();

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

    println!("DONE");

    edges
}

pub fn create_mesh(source: Array2<u8>, n_points: usize, power: f64) -> (Vec<Point>, Vec<Vec<usize>>) {
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
            // make x coordinate
            x = rng.gen_range(0..array_shape.0);
            if rng.gen_range(0.0..1.0) <= pdf_x[[x]].powf(power) {
                break;
            }
        }

        loop {
            // make y coordinate
            y = rng.gen_range(0..array_shape.1);
            if rng.gen_range(0.0..1.0) <= pdf_y[[y]].powf(power) {
                break;
            }
        }

        points.push(Point {
            x: x as f64,
            y: y as f64,
        });
        if points.len() == n_points {
            break;
        }
    }

    let triangulation = triangulate(&points);

    println!("DONE");

    (
        points,
        triangulation
            .triangles
            .chunks(3)
            .map(|s| s.into())
            .collect(),
    )
}

pub fn sort_pixels_into_triangles(
    original_image: &Picture,
    mesh_info: (Vec<Point>, Vec<Vec<usize>>),
) -> (Array2<u8>, Array2<usize>) {
    print!("sorting pixels and colours...");
    let _ = io::stdout().flush();

    let (points, triangles) = mesh_info;
    let mut colors = Array2::<u64>::zeros((triangles.len(), 3));
    let mut triangle_table = Array2::<usize>::zeros(original_image.dimension());
    let mut triangle_pixel_counts = Array1::<usize>::zeros(triangles.len());

    let (width, height) = original_image.dimension();

    for w in 0..width {
        for h in 0..height {
            let pixel = original_image.get_pixel(w, h);
            let (r, g, b) = (pixel[[0]], pixel[[1]], pixel[[2]]);

            for (t, triangle) in triangles.iter().enumerate() {
                let triangle_positions = triangle
                    .iter()
                    .map(|p| &points[*p])
                    .collect::<Vec<&Point>>();

                if !point_is_in_triangle(
                    &Point {
                        x: w as f64,
                        y: h as f64,
                    },
                    triangle_positions,
                ) {
                    continue;
                }

                // point is in triangle... do calculations
                triangle_table[[w, h]] = t;
                triangle_pixel_counts[t] += 1;

                colors[[t, 0]] += r as u64;
                colors[[t, 1]] += g as u64;
                colors[[t, 2]] += b as u64;
            }
        }
    }

    for i in 0..triangles.len() {
        colors[[i, 0]] /= triangle_pixel_counts[[i]] as u64;
        colors[[i, 1]] /= triangle_pixel_counts[[i]] as u64;
        colors[[i, 2]] /= triangle_pixel_counts[[i]] as u64;
    }

    println!("DONE");

    (
        colors.mapv(|x| u8::try_from(x).expect(&format!("{} encountered", x))),
        triangle_table,
    )
}

fn make_marginal_pdfs(source: Array2<u8>) -> (Array1<f64>, Array1<f64>) {
    let weight: f64 = source.mapv(|x| f64::from(x)).sum();
    let array_shape = source.dim();

    let pdf_x = (0..array_shape.0)
        .map(|x| source.column(x).mapv(|x| f64::from(x)).sum() / weight)
        .collect::<Array1<f64>>();
    let pdf_y = (0..array_shape.1)
        .map(|x| source.row(x).mapv(|x| f64::from(x)).sum() / weight)
        .collect::<Array1<f64>>();

    (pdf_x, pdf_y)
}

fn point_is_in_triangle(point: &Point, triangle: Vec<&Point>) -> bool {
    let d1 = tsign(point, triangle[0], triangle[1]);
    let d2 = tsign(point, triangle[1], triangle[2]);
    let d3 = tsign(point, triangle[2], triangle[0]);

    let has_neg = (d1 < 0.) || (d2 < 0.) || (d3 < 0.);
    let has_pos = (d1 > 0.) || (d2 > 0.) || (d3 > 0.);

    !(has_neg && has_pos)
}

fn tsign(p1: &Point, p2: &Point, p3: &Point) -> f64 {
    (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y)
}
