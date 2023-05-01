use std::ops::{Add, Mul};

use crate::{
    constants::{A, B},
    errors::BitcoinError::{self, PointNotInTheCurve},
    felt,
    field_element::FieldElement,
};
use rug::Integer;

// Point of y**2 = x**2 + a*x + b eliptic curve
#[derive(Clone, Debug, PartialEq)]
pub enum Point {
    Point(FieldElement, FieldElement),
    Infinity,
}

impl Point {
    pub fn new_point(x: FieldElement, y: FieldElement) -> Result<Self, BitcoinError> {
        if y.pow(&felt!(2)) != x.pow(&felt!(3)) + A.clone() * x.clone() + B.clone() {
            return Err(PointNotInTheCurve);
        }

        Ok(Point::Point(x, y))
    }
    pub fn new_infinity() -> Self {
        Point::Infinity
    }

    /// Returns the slope of the tangent line at a given point
    pub fn tangent_slope(&self) -> FieldElement {
        match self {
            Point::Point(x, y) => {
                (felt!(3) * x.pow(&felt!(2)) + A.clone()) / (felt!(2) * y.clone())
            }
            _ => panic!(),
        }
    }

    /// Returns the slope of the line between two points
    pub fn slope(&self, other: &Point) -> FieldElement {
        match (self, other) {
            (Point::Point(x1, y1), Point::Point(x2, y2)) => (y2 - y1) / (x2 - x1),
            _ => panic!(),
        }
    }
}

impl Add<Point> for Point {
    type Output = Self;
    fn add(self, other_point: Point) -> Self {
        match (&self, &other_point) {
            (Self::Infinity, _) => other_point,
            (_, Self::Infinity) => self,
            (Self::Point(x1, y1), Self::Point(x2, _y2)) => {
                if self == other_point {
                    if y1.num == Integer::ZERO {
                        Point::Infinity
                    } else {
                        let slope = self.tangent_slope();
                        let x3 = slope.pow(&felt!(2)) - felt!(2) * x1.clone();
                        let y = slope * (x1 - &x3) - y1.clone();
                        Point::new_point(x3, y).unwrap()
                    }
                } else if x1 == x2 {
                    return Point::Infinity;
                } else {
                    let slope = self.slope(&other_point);
                    let x3 = slope.pow(&felt!(2)) - x1.clone() - x2.clone();
                    let y = slope * (x1 - &x3) - y1.clone();
                    return Point::new_point(x3, y).unwrap();
                }
            }
        }
    }
}

impl Mul<&Integer> for &Point {
    type Output = Point;

    fn mul(self, scalar: &Integer) -> Point {
        assert!(scalar != &Integer::ZERO, "Cant multiply by 0");

        let mut current = self.clone();
        let mut result = Point::new_infinity();
        let mut coef = scalar.clone();
        while coef != Integer::ZERO {
            if (&coef & Integer::from(1)) != Integer::ZERO {
                println!("if 1");
                result = result + current.clone();
                println!("if 2");
            }
            println!("a");
            current = current.clone() + current.clone();
            println!("b");
            coef >>= 1;
        }
        result
    }
}

#[cfg(test)]
mod point_tests {
    use crate::{
        constants::{G, N},
        felt_str,
    };

    use super::*;

    #[test]
    fn test_point_sum() {
        assert_eq!(
            Point::new_point(felt_str!("24398930080362702090489479461985180105578816720137874339735832822062375759460"), felt_str!("54007149621299385755644170254411630654493180623064781550649051280267079509173")).unwrap() + Point::new_point(felt_str!("59864537116326559311346833611109531662167483564301454271272386388321477649300"), felt_str!("75550953355817334437637498591236360658870728572722501436515415412283060275131")).unwrap(),
            Point::new_point(felt_str!("64226399123092535235995845779670929663801771913640521823243056334938852347306"), felt_str!("113687732421995957741893049079948608351368126315416472092696176881410614162809")).unwrap()


        )
    }

    #[test]
    fn test_point_mul() {
        dbg!(&*G);
        dbg!(&*N);
        assert_eq!(&*G * &*N, Point::Infinity)
    }
}
