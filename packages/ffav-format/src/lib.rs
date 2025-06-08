use std::env;
use ffav_codec::codec_id::CodecID;

pub mod wav;

pub mod format_context;

pub mod demux;

pub mod mux;

pub mod error;

pub mod format_flag;

pub mod stream;

pub struct CodecTag {
    id: CodecID,
    tag: u32,
}

pub fn print_executable_path() {
    let exe_path = env::current_exe().unwrap();
    println!("Executable path: {:?}", exe_path);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wav_header() {
        print_executable_path();
        let header = crate::wav::WavHeader::new("/Users/xdo/Downloads/file_example_WAV_1MG.wav");
        println!("{:?}", header);
    }
}
