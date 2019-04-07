use succinct_rs::{LoudsBuilder, LoudsBitString, LoudsNodeNum, LoudsIndex};

#[test]
fn node_num_to_index() {
    let louds = LoudsBuilder::from_lbs(LoudsBitString::new("10_1110_10_0_1110_0_0_10_110_0_0_0")).build();
    assert_eq!(louds.node_num_to_index(LoudsNodeNum(1)), LoudsIndex(0));
    assert_eq!(louds.node_num_to_index(LoudsNodeNum(2)), LoudsIndex(2));
    assert_eq!(louds.node_num_to_index(LoudsNodeNum(3)), LoudsIndex(3));
    assert_eq!(louds.node_num_to_index(LoudsNodeNum(4)), LoudsIndex(4));
    assert_eq!(louds.node_num_to_index(LoudsNodeNum(5)), LoudsIndex(6));
    assert_eq!(louds.node_num_to_index(LoudsNodeNum(6)), LoudsIndex(9));
    assert_eq!(louds.node_num_to_index(LoudsNodeNum(7)), LoudsIndex(10));
    assert_eq!(louds.node_num_to_index(LoudsNodeNum(8)), LoudsIndex(11));
    assert_eq!(louds.node_num_to_index(LoudsNodeNum(9)), LoudsIndex(15));
    assert_eq!(louds.node_num_to_index(LoudsNodeNum(10)), LoudsIndex(17));
    assert_eq!(louds.node_num_to_index(LoudsNodeNum(11)), LoudsIndex(18));
}

#[test]
fn index_to_node_num() {
    let louds = LoudsBuilder::from_lbs(LoudsBitString::new("10_1110_10_0_1110_0_0_10_110_0_0_0")).build();
    assert_eq!(LoudsIndex(0), louds.node_num_to_index(LoudsNodeNum(1)));
    assert_eq!(LoudsIndex(2), louds.node_num_to_index(LoudsNodeNum(2)));
    assert_eq!(LoudsIndex(3), louds.node_num_to_index(LoudsNodeNum(3)));
    assert_eq!(LoudsIndex(4), louds.node_num_to_index(LoudsNodeNum(4)));
    assert_eq!(LoudsIndex(6), louds.node_num_to_index(LoudsNodeNum(5)));
    assert_eq!(LoudsIndex(9), louds.node_num_to_index(LoudsNodeNum(6)));
    assert_eq!(LoudsIndex(10), louds.node_num_to_index(LoudsNodeNum(7)));
    assert_eq!(LoudsIndex(11), louds.node_num_to_index(LoudsNodeNum(8)));
    assert_eq!(LoudsIndex(15), louds.node_num_to_index(LoudsNodeNum(9)));
    assert_eq!(LoudsIndex(17), louds.node_num_to_index(LoudsNodeNum(10)));
    assert_eq!(LoudsIndex(18), louds.node_num_to_index(LoudsNodeNum(11)));
}

#[test]
fn child_to_parent() {
    let louds = LoudsBuilder::from_lbs(LoudsBitString::new("10_1110_10_0_1110_0_0_10_110_0_0_0")).build();
    assert_eq(louds.child_to_parent(LoudsIndex(2)), LoudsNodeNum(1));
    assert_eq(louds.child_to_parent(LoudsIndex(3)), LoudsNodeNum(1));
    assert_eq(louds.child_to_parent(LoudsIndex(4)), LoudsNodeNum(1));
    assert_eq(louds.child_to_parent(LoudsIndex(6)), LoudsNodeNum(2));
    assert_eq(louds.child_to_parent(LoudsIndex(9)), LoudsNodeNum(4));
    assert_eq(louds.child_to_parent(LoudsIndex(10)), LoudsNodeNum(4));
    assert_eq(louds.child_to_parent(LoudsIndex(11)), LoudsNodeNum(4));
    assert_eq(louds.child_to_parent(LoudsIndex(15)), LoudsNodeNum(7));
    assert_eq(louds.child_to_parent(LoudsIndex(17)), LoudsNodeNum(8));
    assert_eq(louds.child_to_parent(LoudsIndex(18)), LoudsNodeNum(8));
}

#[test]
fn parent_to_children() {
    let louds = LoudsBuilder::from_lbs(LoudsBitString::new("10_1110_10_0_1110_0_0_10_110_0_0_0")).build();
    assert_eq(louds.parent_to_children(LoudsNodeNum(1), vec!(LoudsIndex(2), LoudsIndex(3), LoudsIndex(4));
    assert_eq(louds.parent_to_children(LoudsNodeNum(2), vec!(LoudsIndex(6));
    assert_eq(louds.parent_to_children(LoudsNodeNum(3), vec!();
    assert_eq(louds.parent_to_children(LoudsNodeNum(4), vec!(LoudsIndex(9), LoudsIndex(10), LoudsIndex(11));
    assert_eq(louds.parent_to_children(LoudsNodeNum(5), vec!();
    assert_eq(louds.parent_to_children(LoudsNodeNum(6), vec!();
    assert_eq(louds.parent_to_children(LoudsNodeNum(7), vec!(LoudsIndex(15));
    assert_eq(louds.parent_to_children(LoudsNodeNum(8), vec!(LoudsIndex(17), LoudsIndex(18));
    assert_eq(louds.parent_to_children(LoudsNodeNum(9), vec!();
    assert_eq(louds.parent_to_children(LoudsNodeNum(10), vec!();
    assert_eq(louds.parent_to_children(LoudsNodeNum(11), vec!();
}
