#[macro_export]
macro_rules! felt {
    ($num: expr) => {
        FieldElement::new(Into::<BigUint>::into($num))
    };
}

#[macro_export]
macro_rules! felt_str {
    ($num: expr) => {
        FieldElement::new(BigUint::parse_bytes($num, 10).unwrap())
    };
}

#[macro_export]
macro_rules! felt_hex {
    ($num: expr) => {
        FieldElement::new(BigUint::parse_bytes($num, 16).unwrap())
    };
}

#[macro_export]
macro_rules! bigint {
    ($val : expr) => {
        Into::<BigUint>::into($val)
    };
}

#[macro_export]
macro_rules! bigint_str {
    ($val: expr) => {
        BigUint::parse_bytes($val, 10).unwrap()
    };
}

#[macro_export]
macro_rules! bigint_hex {
    ($val: expr) => {
        BigUint::parse_bytes($val, 16).unwrap()
    };
}

#[macro_export]
macro_rules! point {
    ($x: expr, $y:expr) => {
        Point::new_point(felt!($x), felt!($y)).unwrap()
    };
}
