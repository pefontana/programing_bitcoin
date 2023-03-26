use rug::Integer;

use lazy_static::lazy_static;
use rug::ops::Pow;

lazy_static! {
    pub static ref PRIME: Integer = Integer::from(2).pow(256) - 2_u64.pow(32) - 977;
}
