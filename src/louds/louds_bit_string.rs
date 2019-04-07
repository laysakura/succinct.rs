use super::LoudsBitString;
use crate::BitVectorString;

impl super::LoudsBitString {
    pub fn new(s: &str) -> LoudsBitString {
        LoudsBitString {
            bvs: BitVectorString::new(s),
        }
    }
}
