extern crate image;

use std::env;
use std::fs::File;
use std::path::Path;

use image::{ImageFormat, GenericImageView};

fn process_image (f: &String) {
	let image = match image::open(Path::new(f)) {
		Ok(img) => img,
		Err(e) => panic!("could not read image: {}", e),
	};

	let (width, height) = image.dimensions();
	println!("width: {}, height: {}", width, height);


	for pixel in image.pixels() {
		println!("{:?}", pixel);
	}

}

fn main() {
	
	let f = if env::args().count() == 2 {
		env::args().nth(1).unwrap()
	} else {
		panic!("no file dummy")
	};

	process_image(&f);
}
