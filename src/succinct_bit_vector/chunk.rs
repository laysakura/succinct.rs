use super::Blocks;
use crate::internal_data_structure::raw_bit_vector::RawBitVector;

impl super::Chunk {
    /// Constructor.
    pub fn new(value: u64, length: u16, rbv: &RawBitVector, i_chunk: u64) -> Self {
        let blocks = Blocks::new(rbv, i_chunk, length);
        Self {
            value,
            length,
            blocks,
        }
    }

    /// Returns the content of the chunk.
    pub fn value(&self) -> u64 {
        self.value
    }
}
