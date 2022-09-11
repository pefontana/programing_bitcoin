#[macro_export]
macro_rules! felt {
    ($num: expr) => {
        FieldElement::<P>::new($num).unwrap()
    };
    ($prime: expr, $num: expr) => {
        FieldElement::<$prime>::new($num).unwrap()
    };
}

#[macro_export]
macro_rules! point {
    ($x: expr, $y:expr) => {
        Point::<A, B, P>::new_point(felt!($x), felt!($y)).unwrap()
    };
}
