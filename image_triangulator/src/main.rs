mod image;

pub fn main() {
    println!("Hello World!");

    let OriginalImage = image::Image::from("image.png");

    let slice = OriginalImage.get_sub_array(0, 0);

    println!("{:#?}", slice);
}