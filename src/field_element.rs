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

impl Sub<&FieldElement> for &FieldElement {
    type Output = FieldElement;

    fn sub(self, other_field_elem: &FieldElement) -> FieldElement {
        FieldElement::new(self.num.clone() - other_field_elem.num.clone())
    }
}

impl Mul<FieldElement> for FieldElement {
    type Output = Self;

    fn mul(self, other_field_elem: Self) -> Self {
        FieldElement::new(self.num * other_field_elem.num)
    }
}

impl Mul<&FieldElement> for &FieldElement {
    type Output = FieldElement;

    fn mul(self, other_field_elem: &FieldElement) -> FieldElement {
        FieldElement::new(self.num.clone() * other_field_elem.num.clone())
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

impl Div for &FieldElement {
    type Output = FieldElement;

    fn div(self, other: &FieldElement) -> FieldElement {
        if let Ok(inv) = other.num.clone().invert(&PRIME) {
            FieldElement::new(self.num.clone() * &inv)
        } else {
            unreachable!()
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::{felt, felt_str};

    use super::*;
    #[test]
    fn new_field_element() {
        let a = FieldElement::new(PRIME.clone() + 1);
        assert_eq!(a.num, Integer::from(1));

        let b = FieldElement::new(Integer::from(-1));
        assert_eq!(b.num, (PRIME.clone() - 1));

        let c = FieldElement::new(Integer::from(765));
        assert_eq!(c.num, Integer::from(765));

        let d = FieldElement::new(Integer::from(-765));
        assert_eq!(d.num, (PRIME.clone() - 765));
    }

    #[test]
    fn test_add() {
        let half_prime: Integer = PRIME.clone() / 2;
        assert_eq!(
            FieldElement::new(half_prime.clone() + 56) + FieldElement::new(Integer::from(51312316)),
            FieldElement::new(
                Integer::from_str_radix(
                    "57896044618658097711785492504343953926634992332820282019728792003954468648203",
                    10
                )
                .unwrap()
            )
        );
        assert_eq!(
            felt!(half_prime.clone() + 56) + felt!(half_prime + 12364563),
            felt!(12364618)
        );
    }

    #[test]
    fn test_sub() {
        let half_prime: Integer = PRIME.clone() / 2;

        assert_eq!(felt!(109) - felt!(9), felt!(100));
        assert_eq!(
            felt!(67) - felt!(99),
            felt_str!(
                "115792089237316195423570985008687907853269984665640564039457584007908834671631"
            )
        );
        assert_eq!(
            felt!(1) - felt!(half_prime.clone()),
            felt_str!(
                "57896044618658097711785492504343953926634992332820282019728792003954417335833"
            )
        );
        assert_eq!(
            felt!(half_prime.clone()) - felt!(11321312),
            felt_str!(
                "57896044618658097711785492504343953926634992332820282019728792003954406014519"
            )
        );
        assert_eq!(
            felt!(half_prime.clone()) - felt!(half_prime + 123123),
            felt_str!(
                "115792089237316195423570985008687907853269984665640564039457584007908834548540"
            )
        );
    }

    #[test]
    fn test_mul() {
        let half_prime: Integer = PRIME.clone() / 2;

        assert_eq!(felt!(0) * felt!(9), felt!(0));
        assert_eq!(felt!(9) * felt!(9), felt!(81));
        assert_eq!(felt!(67) * felt!(99), felt!(6633));
        assert_eq!(
            felt!(1) * felt!(half_prime.clone()),
            felt!(half_prime.clone())
        );
        assert_eq!(
            felt!(half_prime.clone()) * felt!(11321312),
            felt_str!(
                "115792089237316195423570985008687907853269984665640564039457584007908829011007"
            )
        );
        assert_eq!(
            felt!(half_prime.clone()) * felt!(half_prime + 123123),
            felt_str!(
                "86844066927987146567678238756515930889952488499230423029593188005931625942186"
            )
        );
    }

    #[test]
    fn test_div() {
        let half_prime: Integer = PRIME.clone() / 2;

        assert_eq!(felt!(0) / felt!(9), felt!(0));
        assert_eq!(felt!(9) / felt!(9), felt!(1));
        assert_eq!(
            felt!(67) / felt!(99),
            felt_str!(
                "54972001961150112978867033286952845142461507871566732422772792407795103328972"
            )
        );
        assert_eq!(
            felt!(1) / felt!(half_prime.clone()),
            felt_str!(
                "115792089237316195423570985008687907853269984665640564039457584007908834671661"
            )
        );
        assert_eq!(
            felt!(half_prime.clone()) / felt!(11321312),
            felt_str!(
                "51457518096643157087273004048917440940111732376568828515939739785202853004220"
            )
        );
        assert_eq!(
            felt!(half_prime.clone()) / felt!(half_prime + 123123),
            felt_str!(
                "6715842427206034246540806139796059615262855777760679549276262319238782419057"
            )
        );
    }

    #[test]
    fn test_pow() {
        let half_prime: Integer = PRIME.clone() / 2;

        assert_eq!(felt!(0).pow(&felt!(9)), felt!(0));
        assert_eq!(felt!(9).pow(&felt!(9)), felt!(387420489));
        assert_eq!(
            felt!(67).pow(&felt!(99)),
            felt_str!(
                "72479935217262289367494677167304479626254240006257415715499442926700339045007"
            )
        );
        assert_eq!(
            felt!(12312311).pow(&felt!(7123123)),
            felt_str!(
                "104412294607254308520680165928959998854987918678776379280502318629395142924719"
            )
        );
        assert_eq!(felt!(1).pow(&felt!(half_prime.clone())), felt!(1));
        assert_eq!(
            felt!(half_prime.clone()).pow(&felt!(11321312)),
            felt_str!(
                "27638471586956803017572943480961683981573780220627878941854276272488949807439"
            )
        );
        assert_eq!(
            felt!(half_prime.clone()).pow(&felt!(half_prime + 123123)),
            felt_str!(
                "34200881240540466468744785772900404633747554619317806293093634094790802146351"
            )
        );
    }
}
