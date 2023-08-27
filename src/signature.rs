use rug::Integer;

use crate::{
    constants::{G, N},
    point::Point,
};

pub struct Signature {
    pub r: Integer,
    pub s: Integer,
}

impl Signature {
    pub fn new(r: Integer, s: Integer) -> Self {
        Self { r, s }
    }

    pub fn new_from_hex(r: String, s: String) -> Self {
        Self::new(
            Integer::from_str_radix(r.strip_prefix("0x").unwrap(), 16).unwrap(),
            Integer::from_str_radix(s.strip_prefix("0x").unwrap(), 16).unwrap(),
        )
    }

    pub fn verify(&self, z: &Integer, point: &Point) -> bool {
        let s_inv = &self
            .s
            .clone()
            .pow_mod(&(&*N - Integer::from(2)), &N)
            .unwrap();
        let u = (z * s_inv.clone()) % &*N;
        let v = (&self.r * s_inv.clone()) % &*N;
        if let Point::Point(x, _y) = &*G * &u + point * &v {
            return x.num == self.r;
        }
        false
    }
}

#[cfg(test)]
mod point_tests {

    use super::*;

    #[test]
    fn test_signatures() {
        let point = Point::point_from_hex(
            "0x887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c".to_string(),
            "0x61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34".to_string(),
        )
        .unwrap();

        let signature_1 = Signature::new_from_hex(
            "0xac8d1c87e51d0d441be8b3dd5b05c8795b48875dffe00b7ffcfac23010d3a395".to_string(),
            "0x68342ceff8935ededd102dd876ffd6ba72d6a427a3edb13d26eb0781cb423c4".to_string(),
        );

        let z_1 = Integer::from_str_radix(
            "ec208baa0fc1c19f708a9ca96fdeff3ac3f230bb4a7ba4aede4942ad003c0f60",
            16,
        )
        .unwrap();

        assert!(signature_1.verify(&z_1, &point));

        let signature_2 = Signature::new_from_hex(
            "0xeff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c".to_string(),
            "0xc7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6".to_string(),
        );
        let z_2 = Integer::from_str_radix(
            "7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d",
            16,
        )
        .unwrap();
    
        assert!(signature_2.verify(&z_2, &point));
    }
}
