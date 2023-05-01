use rug::Integer;

use lazy_static::lazy_static;
use rug::ops::Pow;

use crate::{field_element::FieldElement, point::Point};

lazy_static! {
    pub static ref PRIME: Integer = Integer::from(2).pow(256) - 2_u64.pow(32) - 977;
    pub static ref A: FieldElement = FieldElement::new(Integer::from(0));
    pub static ref B: FieldElement = FieldElement::new(Integer::from(7));
    pub static ref GX: FieldElement = FieldElement::new(
        Integer::from_str_radix(
            "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
            16
        )
        .unwrap()
    );
    pub static ref GY: FieldElement = FieldElement::new(
        Integer::from_str_radix(
            "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
            16
        )
        .unwrap()
    );
    pub static ref G: Point = Point::new_point(GX.clone(), GY.clone()).unwrap();
    pub static ref N: Integer = Integer::from_str_radix(
        "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
        16
    )
    .unwrap();
}
