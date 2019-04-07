use super::{Louds, LoudsIndex, LoudsNodeNum};

impl Louds {
    /// # Panics
    /// `node_num` does not exist in this LOUDS.
    pub fn node_num_to_index(&self, node_num: LoudsNodeNum) -> LoudsIndex {
        assert!(node_num.value() > 0);

        let index = self.lbs.select(node_num.value()).expect(&format!(
            "NodeNum({}) does not exist in this LOUDS",
            node_num.value(),
        ));
        LoudsIndex::new(index)
    }

    /// # Panics
    /// `index` does not point to any node in this LOUDS.
    pub fn index_to_node_num(&self, index: LoudsIndex) -> LoudsNodeNum {
        assert_eq!(
            self.lbs.access(index.value()),
            true,
            "LBS[index={:?}] must be '1'",
            index,
        );

        let node_num = self.lbs.rank(index.value());
        LoudsNodeNum::new(node_num)
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

#[cfg(test)]
mod node_num_to_index_success_tests {
    use crate::{BitString, LoudsBuilder, LoudsIndex, LoudsNodeNum};

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_s, node_num, expected_index) = $value;
                let bs = BitString::new(in_s);
                let louds = LoudsBuilder::from_bit_string(bs).build();
                let index = louds.node_num_to_index(LoudsNodeNum::new(node_num));
                assert_eq!(index, LoudsIndex::new(expected_index));
            }
        )*
        }
    }

    parameterized_tests! {
        t1_1: ("10_0", 1, 0),

        t2_1: ("10_10_0", 1, 0),
        t2_2: ("10_10_0", 2, 2),

        t3_1: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 1, 0),
        t3_2: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 2, 2),
        t3_3: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 3, 3),
        t3_4: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 4, 4),
        t3_5: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 5, 6),
        t3_6: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 6, 9),
        t3_7: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 7, 10),
        t3_8: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 8, 11),
        t3_9: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 9, 15),
        t3_10: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 10, 17),
        t3_11: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 11, 18),
    }

}

#[cfg(test)]
mod node_num_to_index_failure_tests {
    use crate::{BitString, LoudsBuilder, LoudsNodeNum};

    macro_rules! parameterized_node_not_found_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let (in_s, node_num) = $value;
                let bs = BitString::new(in_s);
                let louds = LoudsBuilder::from_bit_string(bs).build();
                let _ = louds.node_num_to_index(LoudsNodeNum::new(node_num));
            }
        )*
        }
    }

    parameterized_node_not_found_tests! {
        t1_1: ("10_0", 0),
        t1_2: ("10_0", 2),

        t2_1: ("10_10_0", 0),
        t2_2: ("10_10_0", 3),

        t3_1: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 0),
        t3_2: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 12),
    }
}

#[cfg(test)]
mod index_to_node_num_success_tests {
    use crate::{BitString, LoudsBuilder, LoudsIndex, LoudsNodeNum};

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_s, index, expected_node_num) = $value;
                let bs = BitString::new(in_s);
                let louds = LoudsBuilder::from_bit_string(bs).build();
                let node_num = louds.index_to_node_num(LoudsIndex::new(index));
                assert_eq!(node_num, LoudsNodeNum::new(expected_node_num));
            }
        )*
        }
    }

    parameterized_tests! {
        t1_1: ("10_0", 0, 1),

        t2_1: ("10_10_0", 0, 1),
        t2_2: ("10_10_0", 2, 2),

        t3_1: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 0, 1),
        t3_2: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 2, 2),
        t3_3: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 3, 3),
        t3_4: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 4, 4),
        t3_5: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 6, 5),
        t3_6: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 9, 6),
        t3_7: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 10, 7),
        t3_8: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 11, 8),
        t3_9: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 15, 9),
        t3_10: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 17, 10),
        t3_11: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 18, 11),
    }

}

#[cfg(test)]
mod index_to_node_num_failure_tests {
    use crate::{BitString, LoudsBuilder, LoudsIndex};

    macro_rules! parameterized_node_not_found_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let (in_s, index) = $value;
                let bs = BitString::new(in_s);
                let louds = LoudsBuilder::from_bit_string(bs).build();
                let _ = louds.index_to_node_num(LoudsIndex::new(index));
            }
        )*
        }
    }

    parameterized_node_not_found_tests! {
        t1_1: ("10_0", 1),
        t1_2: ("10_0", 3),

        t2_1: ("10_10_0", 1),
        t2_2: ("10_10_0", 3),
        t2_3: ("10_10_0", 4),
        t2_4: ("10_10_0", 5),

        t3_1: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 1),
        t3_2: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 5),
        t3_3: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 7),
        t3_4: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 8),
        t3_5: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 12),
        t3_6: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 13),
        t3_7: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 14),
        t3_8: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 16),
        t3_9: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 19),
        t3_10: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 20),
        t3_11: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 21),
        t3_12: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 22),
        t3_13: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 23),
        t3_14: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 24),
    }
}
