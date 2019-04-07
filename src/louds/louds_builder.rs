use super::{Louds, LoudsBitString, LoudsBuilder};

impl super::LoudsBuilder {
    pub fn from_lbs(lbs: LoudsBitString) -> LoudsBuilder {
        LoudsBuilder {}
    }

    pub fn build(&self) -> Louds {
        Louds {}
    }
}
