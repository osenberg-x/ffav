/**
 * Types and functions for working with PacketSideData.
 */
pub enum PacketSideDataType {
    /**
     * An PALETTE side data packet contains exactly AVPALETTE_SIZE
     * bytes worth of palette. This side data signals that a new palette is
     * present.
     */
    PALETTE,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketSideData {
	data: Vec<u8>,
}