// use num_traits::cast;
// use num_traits::{Inv, MulAdd, Num, One, Pow, Signed, Zero};
// use rug::Float;
// use rug::Float::big;

// // impl Fft num for rug::Float
// // rug::Float already implenmented Clone + Sync + Send + Debug + 'static {}
// // + NumOps + Neg

// impl num_traits::Inv for rug::Float {
//     type Output = rug::Float;
//     fn inv(self) -> rug::Float {
//         1.0 / self
//     }
// }

// impl<'a> num_traits::Inv for &'a rug::Float {
//     type Output = rug::Float;
//     fn inv(self) -> rug::Float {
//         1.0 / *self
//     }
// }

// impl num_traits::MulAdd for rug::Float {
//     type Output = rug::Float;
//     fn mul_add(self, a: rug::Float, b: rug::Float) -> rug::Float {
//         self.mul_add(a, b)
//     }
// }

// impl<'a> num_traits::MulAdd for &'a rug::Float {
//     type Output = rug::Float;
//     fn mul_add(self, a: rug::Float, b: rug::Float) -> rug::Float {
//         self.mul_add(a, b)
//     }
// }

// impl num_traits::Sized for rug::Float {}

// impl num_traits::Zero for rug::Float {
//     fn zero() {
//         Float::with_val(53, Special::Zero)
//     }

//     fn set_zero(&mut self) {
//         *self = Zero::zero();
//     }

//     fn is_zero(&self) -> bool {
//         self.is_zero()
//     }
// }

// impl num_traits::One for rug::Float {
//     fn one() {
//         Float::with_val(53, 1.0)
//     }

//     fn set_one(&mut self) {
//         *self = One::one();
//     }

//     fn is_one(&self) -> bool {
//         *self == Self::one()
//     }
// }

// impl num_traits::Num for rug::Float {
//     type FromStrRadixErr = big::ParseFloatError;
//     fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
//         self.parse_radix(s, radix)
//     }
// }

// impl num_traits::Signed for rug::Float {
//     fn abs(&self) -> rug::Float {
//         self.abs()
//     }

//     fn abs_sub(&self, other: &rug::Float) -> rug::Float {
//         let temp = self - other;
//         if temp < 0 {
//             Float::with_val(53, Special::Zero)
//         }
//         temp
//     }

//     fn signum(&self) -> rug::Float {
//         self.signum()
//     }

//     fn is_positive(&self) -> bool {
//         self.is_sign_positive()
//     }

//     fn is_negative(&self) -> bool {
//         self.is_sign_negative()
//     }
// }

// impl cast::FromPrimitive for rug::Float {
//     fn from_isize(n: isize) -> Option<Self> {
//         n.to_i64().and_then(FromPrimitive::from_i64)
//     }

//     fn from_i8(n: i8) -> Option<Self> {
//         FromPrimitive::from_i64(From::from(n))
//     }

//     fn from_i16(n: i16) -> Option<Self> {
//         FromPrimitive::from_i64(From::from(n))
//     }

//     fn from_i32(n: i32) -> Option<Self> {
//         FromPrimitive::from_i64(From::from(n))
//     }

//     fn from_i64(n: i64) -> Option<Self> {
//         Some(rug::Float::with_val(64, n))
//     }

//     fn from_i128(n: i128) -> Option<Self> {
//         n.to_i64().and_then(FromPrimitive::from_i64)
//     }

//     fn from_usize(n: usize) -> Option<Self> {
//         n.to_u64().and_then(FromPrimitive::from_u64)
//     }

//     fn from_u8(n: u8) -> Option<Self> {
//         FromPrimitive::from_u64(From::from(n))
//     }

//     fn from_u16(n: u16) -> Option<Self> {
//         FromPrimitive::from_u64(From::from(n))
//     }

//     fn from_u32(n: u32) -> Option<Self> {
//         FromPrimitive::from_u64(From::from(n))
//     }

//     fn from_u64(n: u64) -> Option<Self> {
//         Some(rug::Float::with_val(64, n))
//     }

//     fn from_u128(n: u128) -> Option<Self> {
//         n.to_u64().and_then(FromPrimitive::from_u64)
//     }

//     fn from_f32(n: f32) -> Option<Self> {
//         FromPrimitive::from_f64(From::from(n))
//     }

//     fn from_f64(n: f64) -> Option<Self> {
//         match n.to_i64() {
//             Some(i) => FromPrimitive::from_i64(i),
//             None => n.to_u64().and_then(FromPrimitive::from_u64),
//         }
//     }
// }
