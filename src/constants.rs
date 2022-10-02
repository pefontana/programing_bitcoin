use crate::{bigint, bigint_hex, felt, felt_hex, field_element::FieldElement, point::Point};
use lazy_static::lazy_static;
use num_bigint_dig::BigInt;
use num_traits::Pow;

lazy_static! {
    pub static ref PRIME: BigInt =
        bigint!(2_usize).pow(256_usize) - bigint!(2_usize).pow(32_usize) - 977_usize;
    pub static ref A: FieldElement = felt!(0_usize);
    pub static ref B: FieldElement = felt!(7_usize);
    pub static ref GX: FieldElement =
        felt_hex!(b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798");
    pub static ref GY: FieldElement =
        felt_hex!(b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8");
    pub static ref G: Point = Point::new_point_from_ref(&*GX, &*GY).unwrap();
    pub static ref N: BigInt =
        bigint_hex!(b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141");
}
