use std::ops::{Add, Mul};

use crate::errors::PointNotInTheCurve;
use crate::field_element::FieldElement;

// Point of y**2 = x**2 + a*x + b eliptic curve
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Point<const A: i128, const B: i128, const P: u128> {
    Point(FieldElement<P>, FieldElement<P>),
    Infinity,
}

impl<const A: i128, const B: i128, const P: u128> Point<A, B, P> {
    pub fn new_point(x: FieldElement<P>, y: FieldElement<P>) -> Result<Self, PointNotInTheCurve> {
        if y.pow(2)
            != x.pow(3)
                + FieldElement::<P>::new(A).unwrap() * x
                + FieldElement::<P>::new(B).unwrap()
        {
            return Err(PointNotInTheCurve);
        }

        Ok(Point::<A, B, P>::Point(x, y))
    }
    pub fn new_infinity() -> Self {
        Point::<A, B, P>::Infinity
    }
}

impl<const A: i128, const B: i128, const P: u128> Add<Point<A, B, P>> for Point<A, B, P> {
    type Output = Self;

    fn add(self, other_point: Point<A, B, P>) -> Self {
        match (self, other_point) {
            (Self::Infinity, _) => other_point,
            (_, Self::Infinity) => self,
            (Self::Point(x1, y1), Self::Point(x2, y2)) if x1 == x2 && y1 != y2 => {
                Self::new_infinity()
            }
            (Self::Point(x1, y1), Self::Point(x2, y2))
                if x1 == x2 && y1 == y2 && y1 == FieldElement::<P>::new(0).unwrap() =>
            {
                Self::Infinity
            }
            (Self::Point(x1, y1), Self::Point(x2, y2)) if x1 != x2 => {
                let slope = (y2 - y1) / (x2 - x1);
                let x3 = slope.pow(2) - x1 - x2;
                let y3 = slope * (x1 - x3) - y1;
                Self::Point(x3, y3)
            }
            (Self::Point(x1, y1), Self::Point(x2, y2)) if x1 == x2 && y1 == y2 => {
                let slope = (FieldElement::<P>::new(3).unwrap() * x1.pow(2)
                    + FieldElement::<P>::new(A).unwrap())
                    / (FieldElement::<P>::new(2).unwrap() * y1);
                let x3 = slope.pow(2) - FieldElement::<P>::new(2).unwrap() * x1;
                let y3 = slope * (x1 - x3) - y1;
                Self::Point(x3, y3)
            }

            _ => panic!("Can not handle field element addition"),
        }
    }
}

impl<const A: i128, const B: i128, const P: u128> Mul<usize> for Point<A, B, P> {
    type Output = Self;

    fn mul(self, scalar: usize) -> Self {
        if scalar == 0 {
            panic!("Cant multiply by 0")
        }

        // Naive implementation
        // for _ in 1..scalar  {
        //     result = result + self;
        //     println!("mult result: {:?}", result);
        // }

        let mut current = self;
        let mut result = Point::<A, B, P>::new_infinity();
        let mut coef = scalar;
        while coef != 0 {
            println!("coef: {:?}", coef);
            if coef & 1 != 0 {
                result = result + current;
            }
            current = current + current;
            coef >>= 1;
        }
        result
    }
}

#[cfg(test)]
mod point_tests {
    use crate::{felt, point};

    use super::*;
    #[test]
    fn point_addition() {
        const A: i128 = 0;
        const B: i128 = 7;
        const P: u128 = 223;

        let point1 = point!(170, 142);
        let point2 = point!(60, 139);
        assert_eq!(point1 + point2, point!(220, 181));

        let point3 = point!(47, 71);
        let point4 = point!(17, 56);
        assert_eq!(point3 + point4, point!(215, 68));

        let point5 = point!(143, 98);
        let point6 = point!(76, 66);
        assert_eq!(point5 + point6, point!(47, 71));
    }

    #[test]
    fn point_scalar_multiplication() {
        const A: i128 = 0;
        const B: i128 = 7;
        const P: u128 = 223;

        let point1 = point!(192, 105);
        assert_eq!(point1 * 2, point!(49, 71));

        let point2 = point!(143, 98);
        assert_eq!(point2 * 2, point!(64, 168));

        let point3 = point!(47, 71);
        assert_eq!(point3 * 2, point!(36, 111));

        assert_eq!(point3 * 4, point!(194, 51));

        assert_eq!(point3 * 8, point!(116, 55));

        assert_eq!(point3 * 21, Point::<A, B, P>::new_infinity());
    }
}
