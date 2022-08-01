use std::fmt;

#[derive(Debug, Clone)]
pub struct NotPrimeError;

impl fmt::Display for NotPrimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The order of the field must be a prime number")
    }
}
#[derive(Debug, Clone)]
pub struct PointNotInTheCurve;

impl fmt::Display for PointNotInTheCurve {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point is not on the curve")
    }
}
