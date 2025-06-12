use std::fs::File;
use ffav_format::format_context::FormatContext;
use ffav_codec::*;

fn main() {
	println!("Hello, ffav!");

	let f  = File::open("~/Downloads/example.mp4");
	let mut format_context = FormatContext::new("~/Downloads/example.mp4");
}