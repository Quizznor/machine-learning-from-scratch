use image::open;

pub struct Image {

    answer : i64,
}

impl From<&str> for Image {
    fn from(image: &str) -> Self {
        println!("works: {}", image);

        let rgba = open(image).unwrap().into_rgba8();

        println!("{}", rgba.size());        

        Image {
            answer : 42,
        }
    }
}

