use std::ops::{Add, Div, Mul, Sub};

use rug::Integer;

use crate::constants::PRIME;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldElement {
    pub num: Integer,
}

impl FieldElement {
    pub fn new(num: Integer) -> Self {
        Self {
            num: num % PRIME.clone(),
        }
    }

    pub fn pow(&self, exponent: &FieldElement) -> FieldElement {
        if let Some(result) = self.num.pow_mod_ref(&exponent.num, &PRIME) {
            FieldElement::new(Integer::from(result))
        } else {
            unreachable!()
        }
    }
}

impl Add<FieldElement> for FieldElement {
    type Output = Self;

    fn add(self, other_field_elem: Self) -> Self {
        FieldElement::new(self.num + other_field_elem.num)
    }
}

impl Sub<FieldElement> for FieldElement {
    type Output = Self;

    fn sub(self, other_field_elem: Self) -> Self {
        FieldElement::new(self.num - other_field_elem.num)
    }
}

impl Mul<FieldElement> for FieldElement {
    type Output = Self;

    fn mul(self, other_field_elem: Self) -> Self {
        FieldElement::new(self.num * other_field_elem.num)
    }
}

impl Div for FieldElement {
    type Output = FieldElement;

    fn div(self, other: FieldElement) -> FieldElement {
        if let Ok(inv) = other.num.invert(&PRIME) {
            FieldElement::new(self.num * &inv)
        } else {
            unreachable!()
        }
    }
}
