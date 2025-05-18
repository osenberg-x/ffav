use std::env;
use bitflags::bitflags;
use avcodec::codec_id::CodecID;

pub mod wav;

pub mod format_context;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct FormatFlags: u32 {
        const NO_FILE = 0x0001;
        /**< Needs '%d' in filename. */
        const NEED_NUMBER = 0x0002;
        /**
         * The muxer/demuxer is experimental and should be used with caution.
         * - demuxers: will not be selected automatically by probing,
         *  must be specified explicitly.
         */
        const EXPERIMENTAL = 0x0004;
        /**< Show format stream IDs numbers. */
        const SHOW_IDS = 0x0008;
        /**< Format wants global header. */
        const GLOBAL_HEADER = 0x0040;
        /**< Format does not need / have any timestamps. */
        const NO_TIMESTAMPS = 0x0080;
        /**< Use generic index building code. */
        const GENERIC_INDEX = 0x0100;
        /**< Format allows timestamp discontinuities. Note, m
         * uxers always require valid (monotone) timestamps
         */
        const TS_DISCONT = 0x0200;
        /**< Format allows variable fps. */
        const VARIABLE_FPS = 0x0400;
        /**< Format does not need width/height */
        const NO_DIMENSIONS = 0x0800;
        /**< Format does not require any streams */
        const NO_STREAMS = 0x1000;
        /**< Format does not allow to fall back on binary search
         *  via read_timestamp
         */
        const NO_BINSEARCH = 0x2000;
        /**< Format does not allow to fall back on generic search */
        const NO_GENSEARCH = 0x4000;
        /**< Format does not allow seeking by bytes */
        const NO_BYTE_SEEK = 0x8000;
         /**< Format does not require strictly increasing timestamps,
          *  but they must still be monotonic
          */
        const TS_NON_STRICT = 0x20000;
        /**< Format allows muxing negative timestamps. 
         * If not set the timestamp will be shifted in av_write_frame 
         * and av_interleaved_write_frame so they start from 0.
         * The user or muxer can override this through AVFormatContext.
         * avoid_negative_ts
         */
        const TS_NEGATIVE = 0x40000;
        /**< Seeking is based on PTS */
        const SEEK_TO_PTS = 0x4000000;
    }
}

pub struct CodecTag {
    id: CodecID,
    tag: u32,
}

#[derive(Debug)]
pub struct InputFormat {
    /**
     * A comma separated list of short names for the format. New names
     * may be appended with a minor bump.
     */
    name: Option<String>,
    /**
     * Descriptive name for the format, meant to be more human-readable
     * than name. You should use the NULL_IF_CONFIG_SMALL() macro
     * to define it.
     */
    long_name: Option<String>,
    /**
     * A list of mime types supported by the format.
     */
    flags: FormatFlags,
    /**
     * If extensions are defined, then no probe is done. You should
     * usually not use extension format guessing because it is not
     * reliable enough
     */
    extensions: Vec<&'static str>,
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
