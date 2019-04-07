mod louds;
mod louds_builder;
mod louds_index;
mod louds_node_num;

use crate::{BitVector, BitVectorBuilder};

/// LOUDS
pub struct Louds {
    /// LBS (LOUDS Bit String)
    lbs: BitVector,
}

/// Builder
pub struct LoudsBuilder {
    bv_builder: BitVectorBuilder,
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
