mod louds;
mod louds_bit_string;
mod louds_builder;
mod louds_index;
mod louds_node_num;

use crate::bit_vector::BitVectorString;

/// LOUDS
pub struct Louds {}

/// Builder
pub struct LoudsBuilder {}

/// LBS
pub struct LoudsBitString {
    bvs: BitVectorString,
}

#[derive(PartialEq, Eq, Debug)]
/// Node number.
pub struct LoudsNodeNum {
    value: u64,
}

#[derive(PartialEq, Eq, Debug)]
/// Index.
pub struct LoudsIndex {
    value: u64,
}
