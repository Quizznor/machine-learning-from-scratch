mod optimizer;
mod image;

fn main() {
    let source_image = image::new("example.png");
    source_image.generate_new_mesh(100);
}
