use crate::bigint;
use lazy_static::lazy_static;
use num_bigint::BigInt;

lazy_static! {
    pub static ref PRIME: BigInt = bigint!(2).pow(256) - bigint!(2).pow(32) - 977;
}
