//! # Galois Field
//!
//! finite field arithmetic
//!

use core::{
    fmt::{self, Debug, Display, Formatter},
    iter::{Product, Sum},
    ops::{Add, AddAssign, BitXor, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// Functions of A Field
pub trait Field:
    'static
    + Sized
    + Eq
    + PartialEq
    + Copy
    + Clone
    + Send
    + Sync
    + Debug
    + Display
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Div<Output = Self>
    + DivAssign
    + Mul<Output = Self>
    + MulAssign
    + Neg<Output = Self>
    + Product
    + Sum
{
    const ZERO: Self;
    const ONE: Self;

    fn zero() -> Self {
        Self::ZERO
    }

    fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }

    fn one() -> Self {
        Self::ONE
    }

    fn inverse(&self) -> Self;

    fn pow(self, exp: usize) -> Self;

    fn square(&mut self) {
        *self *= *self
    }

    // Convert the inner value to usize
    fn idx(&self) -> usize;
}

/// # The Galois Field Type.
/// Supports all basic Mathematical Functions
///
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GF<T>(pub T);

// impl<T> GF<T> {
//     pub fn new(inner: T) -> Self {
//         Self(inner)
//     }
// }

pub type GFU8 = GF<u8>;
pub type GF256 = GFU8;
// pub type GFU16 = GF<u16>;
// pub type GFU32 = GF<u32>;
// pub type GFU64 = GF<u64>;
// pub type GFU128 = GF<u128>;

// TODO: Generate with build script
const LOGTABLE: [usize; 256] = [
    512, 255, 1, 25, 2, 50, 26, 198, 3, 223, 51, 238, 27, 104, 199, 75, 4, 100, 224, 14, 52, 141,
    239, 129, 28, 193, 105, 248, 200, 8, 76, 113, 5, 138, 101, 47, 225, 36, 15, 33, 53, 147, 142,
    218, 240, 18, 130, 69, 29, 181, 194, 125, 106, 39, 249, 185, 201, 154, 9, 120, 77, 228, 114,
    166, 6, 191, 139, 98, 102, 221, 48, 253, 226, 152, 37, 179, 16, 145, 34, 136, 54, 208, 148,
    206, 143, 150, 219, 189, 241, 210, 19, 92, 131, 56, 70, 64, 30, 66, 182, 163, 195, 72, 126,
    110, 107, 58, 40, 84, 250, 133, 186, 61, 202, 94, 155, 159, 10, 21, 121, 43, 78, 212, 229, 172,
    115, 243, 167, 87, 7, 112, 192, 247, 140, 128, 99, 13, 103, 74, 222, 237, 49, 197, 254, 24,
    227, 165, 153, 119, 38, 184, 180, 124, 17, 68, 146, 217, 35, 32, 137, 46, 55, 63, 209, 91, 149,
    188, 207, 205, 144, 135, 151, 178, 220, 252, 190, 97, 242, 86, 211, 171, 20, 42, 93, 158, 132,
    60, 57, 83, 71, 109, 65, 162, 31, 45, 67, 216, 183, 123, 164, 118, 196, 23, 73, 236, 127, 12,
    111, 246, 108, 161, 59, 82, 41, 157, 85, 170, 251, 96, 134, 177, 187, 204, 62, 90, 203, 89, 95,
    176, 156, 169, 160, 81, 11, 245, 22, 235, 122, 117, 44, 215, 79, 174, 213, 233, 230, 231, 173,
    232, 116, 214, 244, 234, 168, 80, 88, 175,
];

const ALOGTABLE: [u8; 1025] = [
    1, 2, 4, 8, 16, 32, 64, 128, 29, 58, 116, 232, 205, 135, 19, 38, 76, 152, 45, 90, 180, 117,
    234, 201, 143, 3, 6, 12, 24, 48, 96, 192, 157, 39, 78, 156, 37, 74, 148, 53, 106, 212, 181,
    119, 238, 193, 159, 35, 70, 140, 5, 10, 20, 40, 80, 160, 93, 186, 105, 210, 185, 111, 222, 161,
    95, 190, 97, 194, 153, 47, 94, 188, 101, 202, 137, 15, 30, 60, 120, 240, 253, 231, 211, 187,
    107, 214, 177, 127, 254, 225, 223, 163, 91, 182, 113, 226, 217, 175, 67, 134, 17, 34, 68, 136,
    13, 26, 52, 104, 208, 189, 103, 206, 129, 31, 62, 124, 248, 237, 199, 147, 59, 118, 236, 197,
    151, 51, 102, 204, 133, 23, 46, 92, 184, 109, 218, 169, 79, 158, 33, 66, 132, 21, 42, 84, 168,
    77, 154, 41, 82, 164, 85, 170, 73, 146, 57, 114, 228, 213, 183, 115, 230, 209, 191, 99, 198,
    145, 63, 126, 252, 229, 215, 179, 123, 246, 241, 255, 227, 219, 171, 75, 150, 49, 98, 196, 149,
    55, 110, 220, 165, 87, 174, 65, 130, 25, 50, 100, 200, 141, 7, 14, 28, 56, 112, 224, 221, 167,
    83, 166, 81, 162, 89, 178, 121, 242, 249, 239, 195, 155, 43, 86, 172, 69, 138, 9, 18, 36, 72,
    144, 61, 122, 244, 245, 247, 243, 251, 235, 203, 139, 11, 22, 44, 88, 176, 125, 250, 233, 207,
    131, 27, 54, 108, 216, 173, 71, 142, 1, 2, 4, 8, 16, 32, 64, 128, 29, 58, 116, 232, 205, 135,
    19, 38, 76, 152, 45, 90, 180, 117, 234, 201, 143, 3, 6, 12, 24, 48, 96, 192, 157, 39, 78, 156,
    37, 74, 148, 53, 106, 212, 181, 119, 238, 193, 159, 35, 70, 140, 5, 10, 20, 40, 80, 160, 93,
    186, 105, 210, 185, 111, 222, 161, 95, 190, 97, 194, 153, 47, 94, 188, 101, 202, 137, 15, 30,
    60, 120, 240, 253, 231, 211, 187, 107, 214, 177, 127, 254, 225, 223, 163, 91, 182, 113, 226,
    217, 175, 67, 134, 17, 34, 68, 136, 13, 26, 52, 104, 208, 189, 103, 206, 129, 31, 62, 124, 248,
    237, 199, 147, 59, 118, 236, 197, 151, 51, 102, 204, 133, 23, 46, 92, 184, 109, 218, 169, 79,
    158, 33, 66, 132, 21, 42, 84, 168, 77, 154, 41, 82, 164, 85, 170, 73, 146, 57, 114, 228, 213,
    183, 115, 230, 209, 191, 99, 198, 145, 63, 126, 252, 229, 215, 179, 123, 246, 241, 255, 227,
    219, 171, 75, 150, 49, 98, 196, 149, 55, 110, 220, 165, 87, 174, 65, 130, 25, 50, 100, 200,
    141, 7, 14, 28, 56, 112, 224, 221, 167, 83, 166, 81, 162, 89, 178, 121, 242, 249, 239, 195,
    155, 43, 86, 172, 69, 138, 9, 18, 36, 72, 144, 61, 122, 244, 245, 247, 243, 251, 235, 203, 139,
    11, 22, 44, 88, 176, 125, 250, 233, 207, 131, 27, 54, 108, 216, 173, 71, 142, 1, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

impl Field for GF<u8> {
    const ZERO: Self = Self(0);
    const ONE: Self = Self(1);

    fn inverse(&self) -> Self {
        Self(ALOGTABLE[255 - (LOGTABLE[self.idx()] % 255)])
    }
    fn idx(&self) -> usize {
        self.0 as usize
    }

    fn pow(self, pow: usize) -> Self {
        if self.is_zero() && pow != 0 {
            Self::zero()
        } else {
            Self(ALOGTABLE[pow * LOGTABLE[self.idx()] % 255])
        }
    }
}

impl MulAssign for GF<u8> {
    fn mul_assign(&mut self, rhs: GF256) {
        *self = GF(ALOGTABLE[(LOGTABLE[self.idx()] | 0) + (LOGTABLE[rhs.idx()] | 0)] | 0);
    }
}

impl DivAssign for GF<u8> {
    fn div_assign(&mut self, rhs: GF256) {
        *self = GF(ALOGTABLE[LOGTABLE[self.idx()] + 255 - LOGTABLE[rhs.idx()]]);
    }
}

impl<T: Display> Display for GF<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> Sum for GF<T>
where
    Self: Field,
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |a, b| a + b)
    }
}

impl<T> Product for GF<T>
where
    Self: Field,
{
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::one(), |a, b| a * b)
    }
}

impl<T> Add for GF<T>
where
    Self: AddAssign,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut result = self;
        result.add_assign(rhs);
        result
    }
}

impl<T: BitXor<Output = T> + Copy> AddAssign for GF<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 ^ rhs.0);
    }
}

impl<T> Sub for GF<T>
where
    Self: SubAssign,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut result = self;
        result.sub_assign(rhs);
        result
    }
}
impl<T: BitXor<Output = T> + Copy> SubAssign for GF<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self(self.0 ^ rhs.0);
    }
}

impl<T> Mul for GF<T>
where
    Self: MulAssign,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut result = self;
        result.mul_assign(rhs);
        result
    }
}

impl<T> Div for GF<T>
where
    Self: DivAssign,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let mut result = self;
        result.div_assign(rhs);
        result
    }
}

impl<T> Neg for GF<T> {
    type Output = Self;

    fn neg(self) -> Self {
        self
    }
}

macro_rules! impl_conv {
    ($type:ty) => {
        impl From<$type> for GF<$type> {
            fn from(u: $type) -> Self {
                Self(u)
            }
        }

        impl Into<$type> for GF<$type> {
            fn into(self) -> $type {
                self.0
            }
        }
    };
}

impl_conv!(u8);
impl_conv!(u16);
impl_conv!(u32);
impl_conv!(u64);
impl_conv!(u128);

impl_conv!(i8);
impl_conv!(i16);
impl_conv!(i32);
impl_conv!(i64);
impl_conv!(i128);

#[cfg(test)]
mod tests {
    use crate::gf::{Field, GF};
    #[test]
    fn add_sub() {
        assert_eq!((GF(5) + GF(60)) - GF(5), GF(60))
    }

    #[test]
    fn mul_div() {
        assert_eq!((GF(5) * GF(60)) / GF(5), GF(60))
    }

    #[test]
    fn pow() {
        assert_eq!(GF(5).pow(0), GF(1))
    }
    #[test]
    fn pow1() {
        assert_eq!(GF(5).pow(1), GF(5))
    }
    #[test]
    fn pow2() {
        assert_eq!(GF(4).pow(2), GF(4) * GF(4))
    }

    #[test]
    fn conv() {
        assert_eq!(GF::from(34u8), GF(34u8));
        let x: u8 = GF(34u8).into();
        assert!(x == 34);
    }
}
