use super::{Louds, LoudsIndex, LoudsNodeNum};

impl Louds {
    /// # Panics
    /// `node_num` does not exist in this LOUDS.
    pub fn node_num_to_index(&self, node_num: LoudsNodeNum) -> LoudsIndex {
        // TODO panic
        LoudsIndex::new(3)
    }

    /// # Panics
    /// `index` does not point to any node in this LOUDS.
    pub fn index_to_node_num(&self, index: LoudsIndex) -> LoudsNodeNum {
        // TODO panic
        LoudsNodeNum::new(1)
    }

    /// # Panics
    /// `index` does not point to any node in this LOUDS.
    pub fn child_to_parent(&self, index: LoudsIndex) -> LoudsNodeNum {
        LoudsNodeNum::new(1)
    }

    pub fn parent_to_children(&self, node_num: LoudsNodeNum) -> Vec<LoudsIndex> {
        vec![]
    }
}
