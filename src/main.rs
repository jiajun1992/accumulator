use accumulator::group::Rsa2048;
use accumulator::hash;
use accumulator::Accumulator;

use accumulator::hash::qc_hash;
use std::str::FromStr;

//use rustfft::num_complex::Complex32;
use rug::Float;
use rug::Integer;
use rustfft::num_complex;
use rustfft::num_complex::Complex64;
use rustfft::FftPlannerScalar;
//use complexrug;

//pub type ComplexRug = num_complex::Complex<rug::Float>;

use num_traits::cast;
use num_traits::{Inv, MulAdd, Num, One, Pow, Signed, Zero};
//use rug::Float::big;

// impl Fft num for rug::Float
// rug::Float already implenmented Clone + Sync + Send + Debug + 'static {}
// + NumOps + Neg

struct RugWrapper(rug::Float);

impl num_traits::Inv for RugWrapper {
    type Output = rug::Float;
    fn inv(self) -> rug::Float {
        1.0 / self.0
    }
}

impl<'a> num_traits::Inv for &'a RugWrapper {
    type Output = rug::Float;
    fn inv(self) -> rug::Float {
        1.0 / *self.0
    }
}

impl num_traits::MulAdd for RugWrapper {
    type Output = rug::Float;
    fn mul_add(self, a: rug::Float, b: rug::Float) -> rug::Float {
        self.0.mul_add(a, b)
    }
}

impl<'a> num_traits::MulAdd for &'a RugWrapper {
    type Output = rug::Float;
    fn mul_add(self, a: rug::Float, b: rug::Float) -> rug::Float {
        self.0.mul_add(a, b)
    }
}

//impl Sized for RugWrapper {}

impl num_traits::Zero for RugWrapper {
    fn zero() {
        Float::with_val(53, Float::Special::Zero)
    }

    fn set_zero(&mut self) {
        *self.0 = Zero::zero();
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl num_traits::One for RugWrapper {
    fn one() {
        Float::with_val(53, 1.0)
    }

    fn set_one(&mut self) {
        *self.0 = One::one();
    }

    fn is_one(&self) -> bool {
        *self.0 == Self::one()
    }
}

impl num_traits::Num for RugWrapper {
    type FromStrRadixErr = rug::Float::big::ParseFloatError;
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        self.0.parse_radix(str, radix)
    }
}

impl num_traits::Signed for RugWrapper {
    fn abs(&self) -> rug::Float {
        self.abs()
    }

    fn abs_sub(&self, other: &rug::Float) -> rug::Float {
        let temp = self - other;
        if temp < 0 {
            Float::with_val(53, Float::Special::Zero)
        }
        temp
    }

    fn signum(&self) -> rug::Float {
        self.signum()
    }

    fn is_positive(&self) -> bool {
        self.is_sign_positive()
    }

    fn is_negative(&self) -> bool {
        self.is_sign_negative()
    }
}

impl cast::FromPrimitive for rug::Float {
    fn from_isize(n: isize) -> Option<Self> {
        n.to_i64().and_then(FromPrimitive::from_i64)
    }

    fn from_i8(n: i8) -> Option<Self> {
        FromPrimitive::from_i64(From::from(n))
    }

    fn from_i16(n: i16) -> Option<Self> {
        FromPrimitive::from_i64(From::from(n))
    }

    fn from_i32(n: i32) -> Option<Self> {
        FromPrimitive::from_i64(From::from(n))
    }

    fn from_i64(n: i64) -> Option<Self> {
        Some(rug::Float::with_val(64, n))
    }

    fn from_i128(n: i128) -> Option<Self> {
        n.to_i64().and_then(FromPrimitive::from_i64)
    }

    fn from_usize(n: usize) -> Option<Self> {
        n.to_u64().and_then(FromPrimitive::from_u64)
    }

    fn from_u8(n: u8) -> Option<Self> {
        FromPrimitive::from_u64(From::from(n))
    }

    fn from_u16(n: u16) -> Option<Self> {
        FromPrimitive::from_u64(From::from(n))
    }

    fn from_u32(n: u32) -> Option<Self> {
        FromPrimitive::from_u64(From::from(n))
    }

    fn from_u64(n: u64) -> Option<Self> {
        Some(rug::Float::with_val(64, n))
    }

    fn from_u128(n: u128) -> Option<Self> {
        n.to_u64().and_then(FromPrimitive::from_u64)
    }

    fn from_f32(n: f32) -> Option<Self> {
        FromPrimitive::from_f64(From::from(n))
    }

    fn from_f64(n: f64) -> Option<Self> {
        match n.to_i64() {
            Some(i) => FromPrimitive::from_i64(i),
            None => n.to_u64().and_then(FromPrimitive::from_u64),
        }
    }
}

pub type ComplexRug = num_complex::Complex<rug::Float>;

fn main() {
    println!("Hello, world!");
    let acc = Accumulator::<Rsa2048, &'static str>::empty();

    let offset = Integer::from_str(&hash::OFFSET_2048).unwrap();
    dbg!(&offset);
    // Accumulate "dog" and "cat". The `add_with_proof` method returns the new accumulator state
    // and a proof that you accumulated "dog" and "cat".
    let (acc, proof) = acc.add_with_proof(&["dog", "cat"]);
    //dbg!(&acc);

    let cat_hash = qc_hash("cat");
    let dog_hash = qc_hash("dog");
    let cat1_hash = qc_hash("cat1");
    let dog1_hash = qc_hash("dog1");
    dbg!(&cat_hash);
    dbg!(&dog_hash);

    let mut poly1: Vec<Integer> = Vec::new();
    let mut poly2: Vec<Integer> = Vec::new();
    poly1.push(cat_hash);
    poly1.push(dog_hash);
    poly2.push(cat1_hash);
    poly2.push(dog1_hash);
    dbg!(&poly1);

    //------------------------------------------starts here----------------------------------------
    let mut planner = FftPlannerScalar::new();
    let fft = planner.plan_fft_forward(4);

    let mut buffer = vec![Complex64::new(0.0, 0.0); 4];
    buffer[0] = Complex64::new(1.0, 0.0);
    buffer[1] = Complex64::new(9.0, 0.0);
    let mut buffer2 = vec![Complex64::new(0.0, 0.0); 4];
    buffer2[0] = Complex64::new(2.0, 0.0);
    buffer2[1] = Complex64::new(3.0, 0.0);
    dbg!(&buffer);
    dbg!(&buffer2);
    println!("qqqqqqqqqqqqqqqq");
    fft.process(&mut buffer);
    fft.process(&mut buffer2);
    dbg!(&buffer);
    dbg!(&buffer2);
    let fft2 = planner.plan_fft_inverse(4);
    for i in 0..4 {
        buffer2[i] = buffer[i] * buffer2[i];
    }
    println!("wwwwwwwwwwwwwwwwwwwwwww");
    dbg!(&buffer2);
    //fft2.process(&mut buffer);
    fft2.process(&mut buffer2);
    println!("eeeeeeeeeeeeeeeeeeeeeeeee");
    //dbg!(&buffer);
    dbg!(&buffer2);
    //fft.

    println!("wwwwwwwwwwwwwwwwwwwwwwweeeeeeeeeeeeeeeeeeeeeeeee");

    let mut buffer3 = vec![ComplexRug::new(Float.zero(), Float.zero()); 4];
    buffer3[0] = ComplexRug::new(Zero::zero(), Float.zero());
    buffer3[1] = ComplexRug::new(Float.zero(), Float.zero());
    let buffer4 = vec![ComplexRug::new(Float.zero(), Float.zero()); 4];
    buffer4[0] = ComplexRug::new(Float.zero(), Float.zero());
    buffer4[1] = ComplexRug::new(Float.zero(), Float.zero());
    let mut planner3 = FftPlannerScalar::new();
    let fft3 = planner3.plan_fft_forward(4);
    fft3.process(&mut buffer3);
    fft3.process(&mut buffer4);
}
