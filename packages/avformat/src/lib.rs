use std::env;

pub mod wav;

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
