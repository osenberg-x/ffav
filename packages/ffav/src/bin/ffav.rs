use std::fs::File;
use ffav_utils::Args;
use ffav_app::{
	app::ffav_register_all, 
	AppError,
	App
};


fn main() -> Result<(), AppError> {
	println!("Hello, ffav!");

	// let args = Args::new();

	// println!("input arg: {}", args.url);

	ffav_register_all();

	App::list_demuxers();
	let mut app = App::new_with_source(ffav::DataKind::Local("C:/Users/xdo/Downloads/wav-test.wav".to_string()))?;

	let demuxer = app.find_demuxer()?;

	println!("find demuxer: {:?}", demuxer.name());

	Ok(())
}