use std::ops::{Add, Div, Mul, Sub};

use rug::{Complete, Integer};

use crate::constants::PRIME;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldElement {
    pub num: Integer,
}

impl FieldElement {
    pub fn new(num: Integer) -> Self {
        if num < 0 || num >= *PRIME {
            let (_, normalized_value) = num.div_rem_euc_ref(&PRIME).complete();
            FieldElement {
                num: normalized_value,
            }
        } else {
            FieldElement { num }
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_field_element() {
        let a = FieldElement::new(PRIME.clone() + 1);
        assert_eq!(a.num, Integer::from(1));

        let b = FieldElement::new(Integer::from(-1));
        assert_eq!(b.num, Integer::from(PRIME.clone() - 1));

        let c = FieldElement::new(Integer::from(765));
        assert_eq!(c.num, Integer::from(765));

        let d = FieldElement::new(Integer::from(-765));
        assert_eq!(d.num, Integer::from(PRIME.clone() - 765));
    }
}
