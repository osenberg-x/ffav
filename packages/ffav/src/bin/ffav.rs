use std::fs::File;
use ffav_util::args::Args;

fn main() {
	println!("Hello, ffav!");

	let args = Args::new();

	println!("input arg: {}", args.url);

	let f  = File::open("~/Downloads/example.mp4");
}