#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleFormat {
    Unknown = -1,
    U8 = 0,
    S16 = 1,
    S32 = 2,
    Flt = 3,
    Dbl = 4,
    U8P = 5,
    S16P = 6,
    S32P = 7,
    Fltp = 8,
    Dblp = 9,
    S64 = 10,
    S64P = 11,
    Nb,
}