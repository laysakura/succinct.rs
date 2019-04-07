mod louds_feature_test {
    use succinct_rs::{LoudsBuilder, LoudsBitString, LoudsNodeNum, LoudsIndex};

#[test]
fn node_num_to_index() {
    let louds = LoudsBuilder::from_lbs(LoudsBitString::new("10_1110_10_0_1110_0_0_10_110_0_0_0")).build();
    assert_eq!(louds.node_num_to_index(LoudsNodeNum::new(1)), LoudsIndex::new(0));
    assert_eq!(louds.node_num_to_index(LoudsNodeNum::new(2)), LoudsIndex::new(2));
    assert_eq!(louds.node_num_to_index(LoudsNodeNum::new(3)), LoudsIndex::new(3));
    assert_eq!(louds.node_num_to_index(LoudsNodeNum::new(4)), LoudsIndex::new(4));
    assert_eq!(louds.node_num_to_index(LoudsNodeNum::new(5)), LoudsIndex::new(6));
    assert_eq!(louds.node_num_to_index(LoudsNodeNum::new(6)), LoudsIndex::new(9));
    assert_eq!(louds.node_num_to_index(LoudsNodeNum::new(7)), LoudsIndex::new(10));
    assert_eq!(louds.node_num_to_index(LoudsNodeNum::new(8)), LoudsIndex::new(11));
    assert_eq!(louds.node_num_to_index(LoudsNodeNum::new(9)), LoudsIndex::new(15));
    assert_eq!(louds.node_num_to_index(LoudsNodeNum::new(10)), LoudsIndex::new(17));
    assert_eq!(louds.node_num_to_index(LoudsNodeNum::new(11)), LoudsIndex::new(18));
}

#[test]
fn index_to_node_num() {
    let louds = LoudsBuilder::from_lbs(LoudsBitString::new("10_1110_10_0_1110_0_0_10_110_0_0_0")).build();
    assert_eq!(LoudsIndex::new(0), louds.node_num_to_index(LoudsNodeNum::new(1)));
    assert_eq!(LoudsIndex::new(2), louds.node_num_to_index(LoudsNodeNum::new(2)));
    assert_eq!(LoudsIndex::new(3), louds.node_num_to_index(LoudsNodeNum::new(3)));
    assert_eq!(LoudsIndex::new(4), louds.node_num_to_index(LoudsNodeNum::new(4)));
    assert_eq!(LoudsIndex::new(6), louds.node_num_to_index(LoudsNodeNum::new(5)));
    assert_eq!(LoudsIndex::new(9), louds.node_num_to_index(LoudsNodeNum::new(6)));
    assert_eq!(LoudsIndex::new(10), louds.node_num_to_index(LoudsNodeNum::new(7)));
    assert_eq!(LoudsIndex::new(11), louds.node_num_to_index(LoudsNodeNum::new(8)));
    assert_eq!(LoudsIndex::new(15), louds.node_num_to_index(LoudsNodeNum::new(9)));
    assert_eq!(LoudsIndex::new(17), louds.node_num_to_index(LoudsNodeNum::new(10)));
    assert_eq!(LoudsIndex::new(18), louds.node_num_to_index(LoudsNodeNum::new(11)));
}

#[test]
fn child_to_parent() {
    let louds = LoudsBuilder::from_lbs(LoudsBitString::new("10_1110_10_0_1110_0_0_10_110_0_0_0")).build();
    assert_eq!(louds.child_to_parent(LoudsIndex::new(2)), LoudsNodeNum::new(1));
    assert_eq!(louds.child_to_parent(LoudsIndex::new(3)), LoudsNodeNum::new(1));
    assert_eq!(louds.child_to_parent(LoudsIndex::new(4)), LoudsNodeNum::new(1));
    assert_eq!(louds.child_to_parent(LoudsIndex::new(6)), LoudsNodeNum::new(2));
    assert_eq!(louds.child_to_parent(LoudsIndex::new(9)), LoudsNodeNum::new(4));
    assert_eq!(louds.child_to_parent(LoudsIndex::new(10)), LoudsNodeNum::new(4));
    assert_eq!(louds.child_to_parent(LoudsIndex::new(11)), LoudsNodeNum::new(4));
    assert_eq!(louds.child_to_parent(LoudsIndex::new(15)), LoudsNodeNum::new(7));
    assert_eq!(louds.child_to_parent(LoudsIndex::new(17)), LoudsNodeNum::new(8));
    assert_eq!(louds.child_to_parent(LoudsIndex::new(18)), LoudsNodeNum::new(8));
}

#[test]
fn parent_to_children() {
    let louds = LoudsBuilder::from_lbs(LoudsBitString::new("10_1110_10_0_1110_0_0_10_110_0_0_0")).build();
    assert_eq!(louds.parent_to_children(LoudsNodeNum::new(1)), vec!(LoudsIndex::new(2), LoudsIndex::new(3), LoudsIndex::new(4)));
    assert_eq!(louds.parent_to_children(LoudsNodeNum::new(2)), vec!(LoudsIndex::new(6)));
    assert_eq!(louds.parent_to_children(LoudsNodeNum::new(3)), vec!());
    assert_eq!(louds.parent_to_children(LoudsNodeNum::new(4)), vec!(LoudsIndex::new(9), LoudsIndex::new(10), LoudsIndex::new(11)));
    assert_eq!(louds.parent_to_children(LoudsNodeNum::new(5)), vec!());
    assert_eq!(louds.parent_to_children(LoudsNodeNum::new(6)), vec!());
    assert_eq!(louds.parent_to_children(LoudsNodeNum::new(7)), vec!(LoudsIndex::new(15)));
    assert_eq!(louds.parent_to_children(LoudsNodeNum::new(8)), vec!(LoudsIndex::new(17), LoudsIndex::new(18)));
    assert_eq!(louds.parent_to_children(LoudsNodeNum::new(9)), vec!());
    assert_eq!(louds.parent_to_children(LoudsNodeNum::new(10)), vec!());
    assert_eq!(louds.parent_to_children(LoudsNodeNum::new(11)), vec!());
}
}