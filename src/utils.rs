#[macro_export]
macro_rules! felt {
    ($val1 : expr) => {
        FieldElement::new(Integer::from($val1))
    };
}

#[macro_export]
macro_rules! felt_str {
    ($val1 : expr) => {
        FieldElement::new(Integer::from_str_radix($val1, 10).unwrap())
    };
}
