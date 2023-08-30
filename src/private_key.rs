use crate::{
    constants::{G, N},
    point::Point,
    signature::Signature,
};

use rug::rand::{RandGen, RandState};
use rug::Integer;

struct Seed;
impl RandGen for Seed {
    fn gen(&mut self) -> u32 {
        // not really random
        0x8CEF_7310
    }
}

pub struct PrivateKey {
    secret: Integer,
    pub point: Point,
}

impl PrivateKey {
    pub fn new(secret: Integer) -> PrivateKey {
        let point = &*G * &secret;
        PrivateKey { secret, point }
    }

    pub fn sing(&self, z: Integer) -> Signature {
        let mut seed = Seed;
        let mut rand = RandState::new_custom(&mut seed);
        let k: Integer = N.clone().random_below_ref(&mut rand).try_into().unwrap();
        let r = (&*G * &k).get_x().num.clone();
        let a: Integer = N.clone() - 2;
        let k_inv = &k.pow_mod(&a, &N).unwrap();
        let mut s: Integer = ((z + r.clone() * &self.secret) * k_inv) % &*N;
        if (2 * s.clone()) > *N {
            s = &*N - s
        }
        Signature::new(r, s)
    }
}

#[cfg(test)]
mod point_tests {

    use super::*;

    #[test]
    fn sign_private_key() {
        let pk = PrivateKey::new(Integer::from(9931231));
        let msg = Integer::from(132131);
        let signature = pk.sing(msg);
        assert_eq!(
            signature.r,
            Integer::from_str_radix(
                "22710005838364618652811968071825732356514023926510712063679796318149347001383",
                10
            )
            .unwrap()
        );
        assert_eq!(
            signature.s,
            Integer::from_str_radix(
                "23034864746716448693910767004334383267938751097151656510500616870201689359769",
                10
            )
            .unwrap()
        );
    }
}
