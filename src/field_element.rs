use crate::errors::NotPrimeError;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct FieldElement<const P: u128> {
    pub num: i128,
}

fn is_prime(number_to_check: u128) -> bool {
    if number_to_check == 1 {
        return false;
    }

    let mut aux = 2;
    while aux * aux <= number_to_check {
        if number_to_check % aux == 0 {
            return false;
        }
        aux += 1;
    }

    true
}

impl<const P: u128> FieldElement<P> {
    pub fn new(num: i128) -> Result<Self, NotPrimeError> {
        if !is_prime(P) {
            return Err(NotPrimeError);
        }
        Ok(Self { num })
    }
}

impl<const P: u128> Add<FieldElement<P>> for FieldElement<P> {
    type Output = Self;

    fn add(self, other_field_elem: Self) -> Self {
        Self {
            num: (self.num + other_field_elem.num).rem_euclid(P as i128),
        }
    }
}

impl<const P: u128> Sub<FieldElement<P>> for FieldElement<P> {
    type Output = Self;

    fn sub(self, other_field_elem: Self) -> Self {
        Self {
            num: (self.num - other_field_elem.num).rem_euclid(P as i128),
        }
    }
}

impl<const P: u128> Mul<FieldElement<P>> for FieldElement<P> {
    type Output = Self;

    fn mul(self, other_field_elem: Self) -> Self {
        Self {
            num: (self.num * other_field_elem.num).rem_euclid(P as i128),
        }
    }
}

impl<const P: u128> FieldElement<P> {
    pub fn pow(&self, n: i128) -> Self {
        let exp = n.rem_euclid((P - 1_u128) as i128);
        let mut num = self.num;
        for _ in 1..exp {
            num *= self.num;
            num = num.rem_euclid(P as i128);
        }
        Self {
            num: num.rem_euclid(P as i128),
        }
    }
}

impl<const P: u128> Div<FieldElement<P>> for FieldElement<P> {
    type Output = Self;

    fn div(self, other_field_elem: Self) -> Self {
        self * other_field_elem.pow((P - 2_u128) as i128)
    }
}

impl<const P: u128> AddAssign<FieldElement<P>> for FieldElement<P> {
    fn add_assign(&mut self, other_number: Self) {
        self.num = (self.num + other_number.num).rem_euclid(P as i128);
    }
}

impl<const P: u128> SubAssign<FieldElement<P>> for FieldElement<P> {
    fn sub_assign(&mut self, other_number: Self) {
        self.num = (self.num - other_number.num).rem_euclid(P as i128);
    }
}

impl<const P: u128> MulAssign<FieldElement<P>> for FieldElement<P> {
    fn mul_assign(&mut self, other_number: Self) {
        self.num = (self.num * other_number.num).rem_euclid(P as i128);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[should_panic]
    #[test]
    fn test_can_not_create_field_with_no_prime_order() {
        let _ = FieldElement::<10>::new(1).unwrap();
    }

    #[test]
    fn test_add_two_finite_field_elements() {
        let first_field_element = FieldElement::<11>::new(1).unwrap();
        let second_field_element = FieldElement::<11>::new(20).unwrap();

        assert_eq!(
            first_field_element + second_field_element,
            FieldElement::<11>::new(10).unwrap()
        );
    }

    #[test]
    fn test_sub_two_finite_field_elements() {
        let first_field_element = FieldElement::<11>::new(1).unwrap();
        let second_field_element = FieldElement::<11>::new(20).unwrap();

        assert_eq!(
            first_field_element - second_field_element,
            FieldElement::<11>::new(3).unwrap()
        );
    }

    #[test]
    fn test_mul_two_finite_field_elements() {
        let first_field_element = FieldElement::<11>::new(1).unwrap();
        let second_field_element = FieldElement::<11>::new(20).unwrap();

        assert_eq!(
            first_field_element * second_field_element,
            FieldElement::<11>::new(9).unwrap()
        );
    }

    #[test]
    fn test_pow_a_finite_field_with_a_number() {
        let first_field_element = FieldElement::<11>::new(3).unwrap();

        assert_eq!(
            first_field_element.pow(3),
            FieldElement::<11>::new(5).unwrap()
        );
    }

    #[test]
    fn test_div_two_finite_field_elements() {
        let first_field_element = FieldElement::<11>::new(1).unwrap();
        let second_field_element = FieldElement::<11>::new(20).unwrap();

        assert_eq!(
            first_field_element / second_field_element,
            FieldElement::<11>::new(5).unwrap()
        );
    }
}
