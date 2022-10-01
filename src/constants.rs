use crate::{bigint, felt, field_element::FieldElement};
use lazy_static::lazy_static;
use num_bigint::BigInt;

lazy_static! {
    pub static ref PRIME: BigInt = bigint!(2).pow(256) - bigint!(2).pow(32) - 977;
    pub static ref A: FieldElement = felt!(0);
    pub static ref B: FieldElement = felt!(7);
}
