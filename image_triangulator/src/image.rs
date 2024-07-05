use triangulate;
use std::vec::Vec;
use triangulate::PolygonList;

pub struct Image {

    answer : i64,
}

pub fn new(path : &str) -> Image {
    Image {
        answer : 42,
    }
}

impl Image {

    pub fn generate_new_mesh(&self, n_points : usize) -> () {

        let polygons = vec![
        vec![[0f32, 0f32], [0., 1.], [1., 1.], [1., 0.]], 
        vec![[0.05, 0.05], [0.05, 0.95], [0.95, 0.95], [0.95, 0.05]]
        ];
        let mut triangulated_indices = Vec::<[usize; 2]>::new();
        polygons.triangulate(triangulate::formats::IndexedListFormat::new(&mut triangulated_indices).into_fan_builder()).expect("Triangulation failed");
        println!("First triangle: {:?}, {:?}, {:?}", 
            polygons.get_vertex(triangulated_indices[0]), 
            polygons.get_vertex(triangulated_indices[1]), 
            polygons.get_vertex(triangulated_indices[2]));
        
        println!("works");
    }

}