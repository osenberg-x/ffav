use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct FormatFlag: u32 {
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
