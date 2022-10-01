#[macro_export]
macro_rules! felt {
    ($num: expr) => {
        FieldElement::new(Into::<BigInt>::into($num))
    };
}
#[macro_export]
macro_rules! bigint {
    ($val : expr) => {
        Into::<BigInt>::into($val)
    };
}

#[macro_export]
macro_rules! bigint_str {
    ($val: expr) => {
        BigInt::parse_bytes($val, 10).unwrap()
    };
}

#[macro_export]
macro_rules! point {
    ($x: expr, $y:expr) => {
        Point::new_point(felt!($x), felt!($y)).unwrap()
    };
}
