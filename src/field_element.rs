use crate::{bigint, constants::PRIME, felt};
use num_integer::Integer;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};
use num_bigint_dig::{BigUint, ModInverse};
use num_traits::Pow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldElement {
    pub num: BigUint,
}

impl FieldElement {
    pub fn new(num: BigUint) -> Self {
        Self { num }
    }
}

impl Add<FieldElement> for FieldElement {
    type Output = Self;

    fn add(self, other_field_elem: Self) -> Self {
        Self {
            num: (self.num + other_field_elem.num).mod_floor(&PRIME),
        }
    }
}

impl Add<&FieldElement> for FieldElement {
    type Output = FieldElement;

    fn add(self, other_field_elem: &FieldElement) -> FieldElement {
        FieldElement {
            num: (self.num + &other_field_elem.num).mod_floor(&PRIME),
        }
    }
}

impl Sub<FieldElement> for FieldElement {
    type Output = Self;

    fn sub(self, other_field_elem: Self) -> Self {
        Self {
            num: (self.num - other_field_elem.num).mod_floor(&PRIME),
        }
    }
}

impl Sub<&FieldElement> for &FieldElement {
    type Output = FieldElement;

    fn sub(self, other_field_elem: &FieldElement) -> FieldElement {
        FieldElement {
            num: (&self.num - &other_field_elem.num).mod_floor(&PRIME),
        }
    }
}

impl Sub<&FieldElement> for FieldElement {
    type Output = FieldElement;

    fn sub(self, other_field_elem: &FieldElement) -> FieldElement {
        FieldElement {
            num: (self.num - &other_field_elem.num).mod_floor(&PRIME),
        }
    }
}

impl Mul<&FieldElement> for &FieldElement {
    type Output = FieldElement;

    fn mul(self, other_field_elem: &FieldElement) -> FieldElement {
        FieldElement {
            num: (&self.num * &other_field_elem.num).mod_floor(&PRIME),
        }
    }
}

impl Mul<FieldElement> for FieldElement {
    type Output = Self;

    fn mul(self, other_field_elem: Self) -> Self {
        Self {
            num: (self.num * other_field_elem.num).mod_floor(&PRIME),
        }
    }
}
// TODO
//Check perfomance
impl FieldElement {
    // pub fn pow(&self, n: BigUint) -> Self {
        // let exp = bigint!(n).mod_floor(&(&*PRIME - 1_usize));
        // let mut num = self.num.clone();
        // println!("exp :{:?}", &exp);
        // for i in num_iter::range(bigint!(1_usize), exp) {
        //     println!("i :{:?}", &i);
        //     num *= &self.num;
        //     num = num.mod_floor(&PRIME);
        // }
        // Self {
        //     num: num.mod_floor(&PRIME),
        // }

            // Raise the current field element to the given integer power.
    pub fn pow(&self, exponent: BigUint) -> FieldElement {
        println!("USO");
        if let result = self.num.modpow(&exponent, &*PRIME) {
            println!("SALIO");

            FieldElement{
                num: result
            }
        } else {
            unreachable!()
        }
    }
    }
// }

    // Raise the current field element to the given integer power.
    // pub fn pow(&self, exponent: &Integer) -> FieldElement {
    //     if self.field.prime == Integer::from(1) {
    //         return FieldElement::new(Integer::from(0), &self.field.clone());
    //     }

    //     if let Some(result) = self.value.pow_mod_ref(exponent, &self.field.prime) {
    //         FieldElement::new(Integer::from(result), &self.field)
    //     } else {
    //         unreachable!()
    //     }
    // }

// impl Div for FieldElement {
//     type Output = FieldElement;

//     fn div(self, other: FieldElement) -> FieldElement {
//         if let Ok(inv) = other.value.clone().invert(&self.field.prime) {
//             FieldElement::new(
//                 self.value * &inv,
//                 &self.field
//             )
//         } else {
//             unreachable!()
//         }
//     }
// }

impl Div<FieldElement> for FieldElement {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        // self * other_field_elem.pow(&*PRIME - 2)
        if let Some(inv) = other.num.clone().mod_inverse(&*PRIME) {
            FieldElement::new(
                self.num * &inv.to_biguint().unwrap(),
            )
        } else {
            unreachable!()
        }
}
}

impl Div<&FieldElement> for &FieldElement {
    type Output = FieldElement;

    fn div(self, other_field_elem: &FieldElement) -> FieldElement {
        println!("self :{:?}", &self);
        println!("other_field_elem :{:?}", &other_field_elem);
        self * &other_field_elem.pow(&*PRIME - 2_usize)
    }
}

impl AddAssign<FieldElement> for FieldElement {
    fn add_assign(&mut self, other_number: Self) {
        self.num = (&self.num + other_number.num).mod_floor(&PRIME);
    }
}

impl SubAssign<FieldElement> for FieldElement {
    fn sub_assign(&mut self, other_number: Self) {
        self.num = (&self.num - other_number.num).mod_floor(&PRIME);
    }
}

impl MulAssign<FieldElement> for FieldElement {
    fn mul_assign(&mut self, other_number: Self) {
        self.num = (&self.num * other_number.num).mod_floor(&PRIME);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        constants::{G, GX, GY},
        felt,
    };

    use super::*;

    #[test]
    fn test_add_two_finite_field_elements() {
        let first_field_element = felt!(11_usize); //FieldElement::<11>::new(1).unwrap();
        let second_field_element = felt!(11_usize); //FieldElement::<11>::new(20).unwrap();

        assert_eq!(first_field_element + second_field_element, felt!(22_usize));
    }

    #[test]
    fn test_sub_two_finite_field_elements() {
        let first_field_element = felt!(20_usize);
        let second_field_element = felt!(3_usize);

        assert_eq!(first_field_element - second_field_element, felt!(17_usize));
    }

    #[test]
    fn test_gx_and_gy_constants() {
        assert_eq!(GY.pow(bigint!(2_usize)), GX.pow(bigint!(3_usize)) + felt!(7_usize))
    }

    // TODO
    // ADD add and sub tests with num > PRIME

    // #[test]
    // fn test_mul_two_finite_field_elements() {
    //     let first_field_element = FieldElement::<11>::new(1).unwrap();
    //     let second_field_element = FieldElement::<11>::new(20).unwrap();

    //     assert_eq!(
    //         first_field_element * second_field_element,
    //         FieldElement::<11>::new(9).unwrap()
    //     );
    // }

    // #[test]
    // fn test_pow_a_finite_field_with_a_number() {
    //     let first_field_element = FieldElement::<11>::new(3).unwrap();

    //     assert_eq!(
    //         first_field_element.pow(3),
    //         FieldElement::<11>::new(5).unwrap()
    //     );
    // }

    // #[test]
    // fn test_div_two_finite_field_elements() {
    //     let first_field_element = FieldElement::<11>::new(1).unwrap();
    //     let second_field_element = FieldElement::<11>::new(20).unwrap();

    //     assert_eq!(
    //         first_field_element / second_field_element,
    //         FieldElement::<11>::new(5).unwrap()
    //     );
    // }
}
