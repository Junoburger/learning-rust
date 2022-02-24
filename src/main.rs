extern crate rexiv2;

fn main() {
    let meta = rexiv2::Metadata::new_from_path("example.jpg").unwrap();
    // let data = rexiv2::
    println!("{:?}", meta);
}
