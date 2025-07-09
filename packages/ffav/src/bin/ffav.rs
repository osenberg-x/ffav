use std::fs::File;
use ffav_utils::Args;
use ffav_app::{app::ffav_register_all, App};


fn main() {
	println!("Hello, ffav!");

	// let args = Args::new();

	// println!("input arg: {}", args.url);

	ffav_register_all();

	App::list_demuxers();
}