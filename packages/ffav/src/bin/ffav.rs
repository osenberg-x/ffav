use std::fs::File;
use ffav_utils::Args;

fn main() {
	println!("Hello, ffav!");

	let args = Args::new();

	println!("input arg: {}", args.url);

	let f  = File::open("~/Downloads/example.mp4");
}