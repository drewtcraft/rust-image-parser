extern crate image;

use std::env;
use std::fs::File;
use std::path::Path;

use image::{ImageFormat, GenericImageView};

struct Color(u8, u8, u8);

fn process_image (f: &String) -> Vec<Vec<Option<Color>>> {
	let image = match image::open(Path::new(f)) {
		Ok(img) => img,
		Err(e) => panic!("could not read image: {}", e),
	};

	let (width, _) = image.dimensions();

	// probably selected the wrong file
	if width != 100 { panic!("width is not 100px -- actual: {}px", width); }

	let mut v = vec![];

	for (i, pixel) in image.pixels().enumerate() {
		// pull rgb value out of pixel
		let rgba = (pixel.2).0;
		let color = match rgba[3] {
			255 => {
				Some(Color(
					rgba[0] as u8,
					rgba[1] as u8,
					rgba[2] as u8
				))
			},
			_ => None
		};

		// convert to f64 for our weird y-coord math below
		let f = i as f64;
		let w = width as f64;

		let x = (f % w) as usize;

		// on our first pass, we want to instantiate all of the
		// nested vectors
		if x == 0 { v.push(vec![color]); }
		// otherwise, push to the nested array
		else { v[x].push(color); }
//		let y = {
//			let yy = f / w;
//			let yyy = yy.ceil() - 1.0;
//			yyy as usize
//		};
	}
	v
}

fn main() {
	let f = if env::args().count() == 2 {
		env::args().nth(1).unwrap()
	} else {
		panic!("no file dummy")
	};

	process_image(&f);
}
