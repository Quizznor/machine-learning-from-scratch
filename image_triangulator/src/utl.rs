// use image::{DynamicImage, GenericImageView};
// use image::{ImageBuffer, imageops};

// use delaunator::{triangulate, Point};
// use image::ImageFormat::Png;
// use ndarray::{array, Array1};
// use ndarray::{s, Array2};
// use rand::{thread_rng, Rng};

// use crate::picture::Picture;
// use crate::triangle::Triangle;

// use ndarray::prelude::*;
// use ndarray_conv::{ConvExt::conv, ConvMode};

// use ndarray_conv::{ConvExt, ConvMode};

// use image::ImageError;
// use ndarray_image::
// use ndarray::{Array3, Array2};

#[macro_export]
macro_rules! runtime {
    ($fctn:expr, $name:literal) => {{
        let start = ::std::time::Instant::now();
        let rv = $fctn;
        println!("{:.<40}: DONE! ({:#?})", $name, start.elapsed());
        rv
    }};
}

use rand::Rng;
use image::{open, RgbImage};
use ndarray::{s, Array1, Array2, Array3};
use delaunator::{triangulate, Point};
use ndarray_conv::{ConvExt, ConvMode, PaddingMode};
use std::path::PathBuf;

use crate::mesh::*;
type LUT = (Vec<Vec<(usize, usize)>>, Vec<Array1<u8>>);

/// read the image at <path> and return the data as [`Array3<u8>`]
pub fn read_image(path: &PathBuf) -> Array3<u8> {
    let image = open(path).expect("");
    image_to_array(image.to_rgb8())
}

/// convert [`RgbImage`] to [`Array3<u8>`]
pub fn image_to_array(image: RgbImage) -> Array3<u8> {
    let height = image.height();
    let width = image.width();
    let raw = image.into_vec();

    Array3::from_shape_vec((height as usize, width as usize, 3), raw).expect("")
}

/// convert [`Array3<u8>`] to [`RgbImage`]
pub fn array_to_image(array: &Array3<u8>) -> RgbImage {
    let (height, width, _) = array.dim();
    let raw = array.clone().into_raw_vec_and_offset();

    RgbImage::from_raw(width as u32, height as u32, raw.0).expect("")
}

/// save [`Array3<u8>`] as an image to out/[`str`]_<original_file_name>.png
pub fn save_image(array: &Array3<u8>, path: &PathBuf, extension: &str) -> () {
    let image = array_to_image(array);
    let mut full_path = path.to_owned();

    full_path.set_file_name(format!(
        "out/{extension}_{}",
        path.file_name().unwrap().to_str().unwrap()
    ));
    image.save(full_path).expect("")
}

/// perform rudimentary edge detection using Sobel convolution
pub fn contrast(array: &Array3<u8>) -> Array3<u8> {
    let array_conv = array.clone().map(|x| f64::from(*x));
    let kernel = ndarray::array!([-1., 0., 1.], [-2., 0., 2.], [-1., 0., 1.]);

    // calculate contrast over rgb channels individually
    let (width, height, depth) = array.dim();
    let mut contrast = Array3::<f64>::zeros((width - 2, height - 2, depth));
    for color in [0_usize, 1_usize, 2_usize] {
        let channel = array_conv.index_axis(ndarray::Axis(2), color).to_owned();
        let x_edges = channel
            .conv(&kernel, ConvMode::Valid, PaddingMode::Zeros)
            .expect("");
        let y_edges = channel
            .conv(&kernel.t(), ConvMode::Valid, PaddingMode::Zeros)
            .expect("");
        let edges = x_edges.map(|x| x.abs()) + y_edges.map(|x| x.abs());
        contrast.slice_mut(s![.., .., color]).assign(&edges);
    }

    // MAX_PIXEL_VALUE should be 8 * 255,
    // need to renormalize to u8 [0, 255]
    // we devide by 4 to get some saturation
    contrast.map(|x| (x / 4.).round() as u8).to_owned()
}

/// triangulate image from randomly drawn n coordinates in a heatmap
pub fn get_mesh(n: &u32, edge_heatmap: Array3<u8>) -> Mesh {
    let (height, width, _) = edge_heatmap.dim();
    let mut cdf_x = Array1::<f64>::zeros(width);
    let mut cdf_y = Array1::<f64>::zeros(height);
    let mut points = Vec::<Point>::new();

    // bounding box 
    let (north, south) = (height as f64 - 1., 0.);
    let (east, west) = (width as f64 - 1., 0.);
    points.push(Point{ x: east, y: north });
    points.push(Point{ x: east, y: south });
    points.push(Point{ x: west, y: north });
    points.push(Point{ x: west, y: south });

    // get piecewise cdf for x and y from grayscale image
    {
        let channel_coefficients = [0.2627, 0.6789, 0.0593]; // this should sum to = 1.?
        let mut grayscale = Array2::<f64>::zeros((height, width));

        for (color, coeff) in channel_coefficients.iter().enumerate() {
            let channel = edge_heatmap.index_axis(ndarray::Axis(2), color);
            grayscale = grayscale + *coeff * channel.map(|x| f64::from(*x)) / 255.;
        }

        let mut running_sum = 0.;
        for x in 0..width {
            // println!("{x}");
            running_sum += grayscale.column(x).sum();
            cdf_x[x] = running_sum;
        }

        running_sum = 0.;
        for y in 0..height {
            // println!("{y}");
            running_sum += grayscale.row(y).sum();
            cdf_y[y] = running_sum;
        }

        cdf_x /= running_sum;
        cdf_y /= running_sum;
    }

    // chad inverse transform sampling
    // x and y are independent here, which they shouldn't be
    // how do we account for the correct prior probabilities?
    {
        let mut rng = rand::rng();
        loop {
            let (x_chance, y_chance): (f64, f64) = rng.random();

            let (mut min_x, mut min_y) = (f64::MAX, f64::MAX);
            let (mut x, mut y): (usize, usize) = (0, 0);

            for (ix, x_candidate) in cdf_x.iter().enumerate() {
                let dist = (x_candidate - x_chance).abs();
                if dist < min_x {
                    min_x = dist;
                    x = ix;
                }
            }

            for (iy, y_candidate) in cdf_y.iter().enumerate() {
                let dist = (y_candidate - y_chance).abs();
                if dist < min_y {
                    min_y = dist;
                    y = iy;
                }
            }

            points.push(Point {
                x: x as f64,
                y: y as f64,
            });

            if points.len() == (*n) as usize {
                break;
            }
        }
    }

    Mesh::new(triangulate(&points), points)
}

/// get 2d image representation of a delauney triangulation
pub fn build_mesh_image(mesh: &Mesh) -> Array3<u8> {

    let (tri, points) = (mesh.get_triangulation(), mesh.get_points());
    let (width, height) = (points[0].x as usize + 1, points[0].y as usize + 1);
    let mut mesh_image = Array3::<u8>::from_elem((height, width, 3), 255);

    // draw nodes as hollow circles
    for point in points {
        for circum_point in point.circle(1.) {
            _brush(&mut mesh_image, circum_point, 1);
        }
    }

    // color the points and edges of the triangulation in tri
    for triplet in tri.triangles.chunks(3).into_iter() {

        // if triplet[0] < 4 || triplet[1] < 4 || triplet[2] < 4  {continue;}

        let p1 = &points[triplet[0]];
        let p2 = &points[triplet[1]];
        let p3 = &points[triplet[2]];

        for line_point in p1.line(p2) {_brush(&mut mesh_image, line_point, 1);}
        for line_point in p2.line(p3) {_brush(&mut mesh_image, line_point, 1);}
        for line_point in p3.line(p1) {_brush(&mut mesh_image, line_point, 1);}
    }

    mesh_image
}

/// build pixel groupings for a mesh of triangles
pub fn color_lookup_table(mesh: Mesh, image: &Array3<u8>) -> LUT {
    
    let (width, height, n_triangles) = mesh.get_size();
    let mut pixel_groups = vec![Vec::<(usize, usize)>::new(); n_triangles];
    let mut temp_color = vec![Array1::<u64>::zeros(3); n_triangles];
    let mut color_table = Vec::<Array1<u8>>::new();
    
    for x in 0..width {
        for y in 0..height {
            let i = mesh.contains_where(Point{x: x as f64, y: y as f64});
            temp_color[i] += &image.slice(s![y, x,..]).mapv(u64::from);
            pixel_groups[i].push((y, x));
        }
    }


    for (col, pts) in temp_color.iter_mut().zip(&pixel_groups) {
        if pts.len() != 0 {
            color_table.push(col.map(|x| (*x / pts.len() as u64) as u8));
        }
        else {
            color_table.push(Array1::<u8>::zeros(3))
        }
    }

    (pixel_groups, color_table)
}

/// apply a color lookup table 
pub fn color(hashtable: LUT, original_image: Array3<u8>) -> Array3<u8> {

    let mut image = Array3::<u8>::zeros(original_image.dim()); 
    for (group, color) in hashtable.0.into_iter().zip(hashtable.1.into_iter()) {
        for pixel in group.into_iter() {
            image.slice_mut(s![pixel.0, pixel.1, ..]).assign(&color);
        }
    }

    image
}

/// helper function for build_mesh_image, not public
/// (maybe) fill image at a location given by point w/ black
fn _brush(image: &mut Array3<u8>, point: Point, brush_size: i32) -> () {

    let (height, width, _) = image.dim();
    let (x, y) = (point.x as i32, point.y as i32);

    if x - brush_size < 0               // pixel under image
    || y - brush_size < 0               // pixel over image
    || x + brush_size >= width as i32   // pixel east of image
    || y + brush_size >= height as i32  // pixel west of image
    {
        return
    }

    image.slice_mut(s![y - brush_size..y + brush_size,
                            x - brush_size..x + brush_size, ..]).fill(0)
}