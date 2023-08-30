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
    pub fn point_from_hex(x: String, y: String) -> Result<Self, BitcoinError> {
        Point::new_point(FieldElement::from_hex(x), FieldElement::from_hex(y))
    }
    pub fn new_infinity() -> Self {
        Point::Infinity
    }

    pub fn get_x(&self) -> &FieldElement {
        match self {
            Point::Point(x, _) => x,
            _ => panic!(),
        }
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
                result = result + current.clone();
            }
            current = current.clone() + current.clone();
            coef >>= 1;
        }
        result
    }
}

#[cfg(test)]
mod point_tests {
    use std::collections::HashMap;

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
    fn test_g_mul_n() {
        dbg!(&*G);
        dbg!(&*N);
        assert_eq!(&*G * &*N, Point::Infinity)
    }

    // Vector tests from https://chuckbatson.wordpress.com/2014/11/26/secp256k1-test-vectors/
    #[test]
    fn test_point_generator() {
        let results: HashMap<Integer, Point> = HashMap::from([
            (Integer::from(1) , Point::point_from_hex("0x79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798".to_owned(),"0x483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8".to_owned()).unwrap()),
            (Integer::from(2) , Point::point_from_hex("0xC6047F9441ED7D6D3045406E95C07CD85C778E4B8CEF3CA7ABAC09B95C709EE5".to_owned(),"0x1AE168FEA63DC339A3C58419466CEAEEF7F632653266D0E1236431A950CFE52A".to_owned()).unwrap()),
            (Integer::from(3) , Point::point_from_hex("0xF9308A019258C31049344F85F89D5229B531C845836F99B08601F113BCE036F9".to_owned(),"0x388F7B0F632DE8140FE337E62A37F3566500A99934C2231B6CB9FD7584B8E672".to_owned()).unwrap()),
            (Integer::from(4) , Point::point_from_hex("0xE493DBF1C10D80F3581E4904930B1404CC6C13900EE0758474FA94ABE8C4CD13".to_owned(),"0x51ED993EA0D455B75642E2098EA51448D967AE33BFBDFE40CFE97BDC47739922".to_owned()).unwrap()),
            (Integer::from(5) , Point::point_from_hex("0x2F8BDE4D1A07209355B4A7250A5C5128E88B84BDDC619AB7CBA8D569B240EFE4".to_owned(),"0xD8AC222636E5E3D6D4DBA9DDA6C9C426F788271BAB0D6840DCA87D3AA6AC62D6".to_owned()).unwrap()),
            (Integer::from(6) , Point::point_from_hex("0xFFF97BD5755EEEA420453A14355235D382F6472F8568A18B2F057A1460297556".to_owned(),"0xAE12777AACFBB620F3BE96017F45C560DE80F0F6518FE4A03C870C36B075F297".to_owned()).unwrap()),
            (Integer::from(7) , Point::point_from_hex("0x5CBDF0646E5DB4EAA398F365F2EA7A0E3D419B7E0330E39CE92BDDEDCAC4F9BC".to_owned(),"0x6AEBCA40BA255960A3178D6D861A54DBA813D0B813FDE7B5A5082628087264DA".to_owned()).unwrap()),
            (Integer::from(8) , Point::point_from_hex("0x2F01E5E15CCA351DAFF3843FB70F3C2F0A1BDD05E5AF888A67784EF3E10A2A01".to_owned(),"0x5C4DA8A741539949293D082A132D13B4C2E213D6BA5B7617B5DA2CB76CBDE904".to_owned()).unwrap()),
            (Integer::from(9) , Point::point_from_hex("0xACD484E2F0C7F65309AD178A9F559ABDE09796974C57E714C35F110DFC27CCBE".to_owned(),"0xCC338921B0A7D9FD64380971763B61E9ADD888A4375F8E0F05CC262AC64F9C37".to_owned()).unwrap()),
            (Integer::from(10) , Point::point_from_hex("0xA0434D9E47F3C86235477C7B1AE6AE5D3442D49B1943C2B752A68E2A47E247C7".to_owned(),"0x893ABA425419BC27A3B6C7E693A24C696F794C2ED877A1593CBEE53B037368D7".to_owned()).unwrap()),
            (Integer::from(11) , Point::point_from_hex("0x774AE7F858A9411E5EF4246B70C65AAC5649980BE5C17891BBEC17895DA008CB".to_owned(),"0xD984A032EB6B5E190243DD56D7B7B365372DB1E2DFF9D6A8301D74C9C953C61B".to_owned()).unwrap()),
            (Integer::from(12) , Point::point_from_hex("0xD01115D548E7561B15C38F004D734633687CF4419620095BC5B0F47070AFE85A".to_owned(),"0xA9F34FFDC815E0D7A8B64537E17BD81579238C5DD9A86D526B051B13F4062327".to_owned()).unwrap()),
            (Integer::from(13) , Point::point_from_hex("0xF28773C2D975288BC7D1D205C3748651B075FBC6610E58CDDEEDDF8F19405AA8".to_owned(),"0x0AB0902E8D880A89758212EB65CDAF473A1A06DA521FA91F29B5CB52DB03ED81".to_owned()).unwrap()),
            (Integer::from(14) , Point::point_from_hex("0x499FDF9E895E719CFD64E67F07D38E3226AA7B63678949E6E49B241A60E823E4".to_owned(),"0xCAC2F6C4B54E855190F044E4A7B3D464464279C27A3F95BCC65F40D403A13F5B".to_owned()).unwrap()),
            (Integer::from(15) , Point::point_from_hex("0xD7924D4F7D43EA965A465AE3095FF41131E5946F3C85F79E44ADBCF8E27E080E".to_owned(),"0x581E2872A86C72A683842EC228CC6DEFEA40AF2BD896D3A5C504DC9FF6A26B58".to_owned()).unwrap()),
            (Integer::from(16) , Point::point_from_hex("0xE60FCE93B59E9EC53011AABC21C23E97B2A31369B87A5AE9C44EE89E2A6DEC0A".to_owned(),"0xF7E3507399E595929DB99F34F57937101296891E44D23F0BE1F32CCE69616821".to_owned()).unwrap()),
            (Integer::from(17) , Point::point_from_hex("0xDEFDEA4CDB677750A420FEE807EACF21EB9898AE79B9768766E4FAA04A2D4A34".to_owned(),"0x4211AB0694635168E997B0EAD2A93DAECED1F4A04A95C0F6CFB199F69E56EB77".to_owned()).unwrap()),
            (Integer::from(18) , Point::point_from_hex("0x5601570CB47F238D2B0286DB4A990FA0F3BA28D1A319F5E7CF55C2A2444DA7CC".to_owned(),"0xC136C1DC0CBEB930E9E298043589351D81D8E0BC736AE2A1F5192E5E8B061D58".to_owned()).unwrap()),
            (Integer::from(19) , Point::point_from_hex("0x2B4EA0A797A443D293EF5CFF444F4979F06ACFEBD7E86D277475656138385B6C".to_owned(),"0x85E89BC037945D93B343083B5A1C86131A01F60C50269763B570C854E5C09B7A".to_owned()).unwrap()),
            (Integer::from(20) , Point::point_from_hex("0x4CE119C96E2FA357200B559B2F7DD5A5F02D5290AFF74B03F3E471B273211C97".to_owned(),"0x12BA26DCB10EC1625DA61FA10A844C676162948271D96967450288EE9233DC3A".to_owned()).unwrap()),
            (Integer::from_str_radix("112233445566778899", 10).unwrap() , Point::point_from_hex("0xA90CC3D3F3E146DAADFC74CA1372207CB4B725AE708CEF713A98EDD73D99EF29".to_owned(),"0x5A79D6B289610C68BC3B47F3D72F9788A26A06868B4D8E433E1E2AD76FB7DC76".to_owned()).unwrap()),
            (Integer::from_str_radix("112233445566778899112233445566778899", 10).unwrap() , Point::point_from_hex("0xE5A2636BCFD412EBF36EC45B19BFB68A1BC5F8632E678132B885F7DF99C5E9B3".to_owned(),"0x736C1CE161AE27B405CAFD2A7520370153C2C861AC51D6C1D5985D9606B45F39".to_owned()).unwrap()),
            (Integer::from_str_radix("28948022309329048855892746252171976963209391069768726095651290785379540373584",10).unwrap() , Point::point_from_hex("0xA6B594B38FB3E77C6EDF78161FADE2041F4E09FD8497DB776E546C41567FEB3C".to_owned(),"0x71444009192228730CD8237A490FEBA2AFE3D27D7CC1136BC97E439D13330D55".to_owned()).unwrap()),
            (Integer::from_str_radix("57896044618658097711785492504343953926418782139537452191302581570759080747168",10).unwrap() , Point::point_from_hex("0x00000000000000000000003B78CE563F89A0ED9414F5AA28AD0D96D6795F9C63".to_owned(),"0x3F3979BF72AE8202983DC989AEC7F2FF2ED91BDD69CE02FC0700CA100E59DDF3".to_owned()).unwrap()),
            (Integer::from_str_radix("86844066927987146567678238756515930889628173209306178286953872356138621120752",10).unwrap() , Point::point_from_hex("0xE24CE4BEEE294AA6350FAA67512B99D388693AE4E7F53D19882A6EA169FC1CE1".to_owned(),"0x8B71E83545FC2B5872589F99D948C03108D36797C4DE363EBD3FF6A9E1A95B10".to_owned()).unwrap()),
            (Integer::from_str_radix("115792089237316195423570985008687907852837564279074904382605163141518161494317",10).unwrap() , Point::point_from_hex("0x4CE119C96E2FA357200B559B2F7DD5A5F02D5290AFF74B03F3E471B273211C97".to_owned(),"0xED45D9234EF13E9DA259E05EF57BB3989E9D6B7D8E269698BAFD77106DCC1FF5".to_owned()).unwrap()),
            (Integer::from_str_radix("115792089237316195423570985008687907852837564279074904382605163141518161494318",10).unwrap() , Point::point_from_hex("0x2B4EA0A797A443D293EF5CFF444F4979F06ACFEBD7E86D277475656138385B6C".to_owned(),"0x7A17643FC86BA26C4CBCF7C4A5E379ECE5FE09F3AFD9689C4A8F37AA1A3F60B5".to_owned()).unwrap()),
            (Integer::from_str_radix("115792089237316195423570985008687907852837564279074904382605163141518161494319",10).unwrap() , Point::point_from_hex("0x5601570CB47F238D2B0286DB4A990FA0F3BA28D1A319F5E7CF55C2A2444DA7CC".to_owned(),"0x3EC93E23F34146CF161D67FBCA76CAE27E271F438C951D5E0AE6D1A074F9DED7".to_owned()).unwrap()),
            (Integer::from_str_radix("115792089237316195423570985008687907852837564279074904382605163141518161494320",10).unwrap() , Point::point_from_hex("0xDEFDEA4CDB677750A420FEE807EACF21EB9898AE79B9768766E4FAA04A2D4A34".to_owned(),"0xBDEE54F96B9CAE9716684F152D56C251312E0B5FB56A3F09304E660861A910B8".to_owned()).unwrap()),
            (Integer::from_str_radix("115792089237316195423570985008687907852837564279074904382605163141518161494321",10).unwrap() , Point::point_from_hex("0xE60FCE93B59E9EC53011AABC21C23E97B2A31369B87A5AE9C44EE89E2A6DEC0A".to_owned(),"0x081CAF8C661A6A6D624660CB0A86C8EFED6976E1BB2DC0F41E0CD330969E940E".to_owned()).unwrap()),
            (Integer::from_str_radix("115792089237316195423570985008687907852837564279074904382605163141518161494322",10).unwrap() , Point::point_from_hex("0xD7924D4F7D43EA965A465AE3095FF41131E5946F3C85F79E44ADBCF8E27E080E".to_owned(),"0xA7E1D78D57938D597C7BD13DD733921015BF50D427692C5A3AFB235F095D90D7".to_owned()).unwrap()),
            (Integer::from_str_radix("115792089237316195423570985008687907852837564279074904382605163141518161494323",10).unwrap() , Point::point_from_hex("0x499FDF9E895E719CFD64E67F07D38E3226AA7B63678949E6E49B241A60E823E4".to_owned(),"0x353D093B4AB17AAE6F0FBB1B584C2B9BB9BD863D85C06A4339A0BF2AFC5EBCD4".to_owned()).unwrap()),
            (Integer::from_str_radix("115792089237316195423570985008687907852837564279074904382605163141518161494324",10).unwrap() , Point::point_from_hex("0xF28773C2D975288BC7D1D205C3748651B075FBC6610E58CDDEEDDF8F19405AA8".to_owned(),"0xF54F6FD17277F5768A7DED149A3250B8C5E5F925ADE056E0D64A34AC24FC0EAE".to_owned()).unwrap()),
            (Integer::from_str_radix("115792089237316195423570985008687907852837564279074904382605163141518161494325",10).unwrap() , Point::point_from_hex("0xD01115D548E7561B15C38F004D734633687CF4419620095BC5B0F47070AFE85A".to_owned(),"0x560CB00237EA1F285749BAC81E8427EA86DC73A2265792AD94FAE4EB0BF9D908".to_owned()).unwrap()),
            (Integer::from_str_radix("115792089237316195423570985008687907852837564279074904382605163141518161494326",10).unwrap() , Point::point_from_hex("0x774AE7F858A9411E5EF4246B70C65AAC5649980BE5C17891BBEC17895DA008CB".to_owned(),"0x267B5FCD1494A1E6FDBC22A928484C9AC8D24E1D20062957CFE28B3536AC3614".to_owned()).unwrap()),
            (Integer::from_str_radix("115792089237316195423570985008687907852837564279074904382605163141518161494327",10).unwrap() , Point::point_from_hex("0xA0434D9E47F3C86235477C7B1AE6AE5D3442D49B1943C2B752A68E2A47E247C7".to_owned(),"0x76C545BDABE643D85C4938196C5DB3969086B3D127885EA6C3411AC3FC8C9358".to_owned()).unwrap()),
            (Integer::from_str_radix("115792089237316195423570985008687907852837564279074904382605163141518161494328",10).unwrap() , Point::point_from_hex("0xACD484E2F0C7F65309AD178A9F559ABDE09796974C57E714C35F110DFC27CCBE".to_owned(),"0x33CC76DE4F5826029BC7F68E89C49E165227775BC8A071F0FA33D9D439B05FF8".to_owned()).unwrap()),
            (Integer::from_str_radix("115792089237316195423570985008687907852837564279074904382605163141518161494329",10).unwrap() , Point::point_from_hex("0x2F01E5E15CCA351DAFF3843FB70F3C2F0A1BDD05E5AF888A67784EF3E10A2A01".to_owned(),"0xA3B25758BEAC66B6D6C2F7D5ECD2EC4B3D1DEC2945A489E84A25D3479342132B".to_owned()).unwrap()),
            (Integer::from_str_radix("115792089237316195423570985008687907852837564279074904382605163141518161494330",10).unwrap() , Point::point_from_hex("0x5CBDF0646E5DB4EAA398F365F2EA7A0E3D419B7E0330E39CE92BDDEDCAC4F9BC".to_owned(),"0x951435BF45DAA69F5CE8729279E5AB2457EC2F47EC02184A5AF7D9D6F78D9755".to_owned()).unwrap()),
            (Integer::from_str_radix("115792089237316195423570985008687907852837564279074904382605163141518161494331",10).unwrap() , Point::point_from_hex("0xFFF97BD5755EEEA420453A14355235D382F6472F8568A18B2F057A1460297556".to_owned(),"0x51ED8885530449DF0C4169FE80BA3A9F217F0F09AE701B5FC378F3C84F8A0998".to_owned()).unwrap()),
            (Integer::from_str_radix("115792089237316195423570985008687907852837564279074904382605163141518161494332",10).unwrap() , Point::point_from_hex("0x2F8BDE4D1A07209355B4A7250A5C5128E88B84BDDC619AB7CBA8D569B240EFE4".to_owned(),"0x2753DDD9C91A1C292B24562259363BD90877D8E454F297BF235782C459539959".to_owned()).unwrap()),
            (Integer::from_str_radix("115792089237316195423570985008687907852837564279074904382605163141518161494333",10).unwrap() , Point::point_from_hex("0xE493DBF1C10D80F3581E4904930B1404CC6C13900EE0758474FA94ABE8C4CD13".to_owned(),"0xAE1266C15F2BAA48A9BD1DF6715AEBB7269851CC404201BF30168422B88C630D".to_owned()).unwrap()),
            (Integer::from_str_radix("115792089237316195423570985008687907852837564279074904382605163141518161494334",10).unwrap() , Point::point_from_hex("0xF9308A019258C31049344F85F89D5229B531C845836F99B08601F113BCE036F9".to_owned(),"0xC77084F09CD217EBF01CC819D5C80CA99AFF5666CB3DDCE4934602897B4715BD".to_owned()).unwrap()),
            (Integer::from_str_radix("115792089237316195423570985008687907852837564279074904382605163141518161494335",10).unwrap() , Point::point_from_hex("0xC6047F9441ED7D6D3045406E95C07CD85C778E4B8CEF3CA7ABAC09B95C709EE5".to_owned(),"0xE51E970159C23CC65C3A7BE6B99315110809CD9ACD992F1EDC9BCE55AF301705".to_owned()).unwrap()),
            (Integer::from_str_radix("115792089237316195423570985008687907852837564279074904382605163141518161494336",10).unwrap() , Point::point_from_hex("0x79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798".to_owned(),"0xB7C52588D95C3B9AA25B0403F1EEF75702E84BB7597AABE663B82F6F04EF2777".to_owned()).unwrap()),
            ]);

        for (n, result) in results {
            assert_eq!(&*G * &n, result);
        }
    }
}
