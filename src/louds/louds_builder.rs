use super::{Louds, LoudsBuilder};
use crate::bit_vector::BitVectorBuilder;
use crate::BitString;

impl super::LoudsBuilder {
    pub fn from_bit_string(bs: BitString) -> LoudsBuilder {
        let bv_builder = BitVectorBuilder::from_bit_string(bs);
        LoudsBuilder { bv_builder }
    }

    pub fn build(&self) -> Louds {
        let bv = self.bv_builder.build();
        Louds { lbs: bv }
    }
}
