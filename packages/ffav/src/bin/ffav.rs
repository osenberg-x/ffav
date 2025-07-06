use std::fs::File;
use ffav_utils::Args;
use ffav_pipeline::{pipeline::ffav_register_all, Pipeline};


fn main() {
	println!("Hello, ffav!");

	// let args = Args::new();

	// println!("input arg: {}", args.url);

	ffav_register_all();

	Pipeline::list_demuxers();
}