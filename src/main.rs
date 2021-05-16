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

//use rug::Float::big;

// impl Fft num for rug::Float
// rug::Float already implenmented Clone + Sync + Send + Debug + 'static {}
// + NumOps + Neg

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
    let zero = Float::with_val(32, 0.0);
    let mut buffer3 = vec![ComplexRug::new(zero, zero); 4];
    buffer3[0] = ComplexRug::new(zero, zero);
    buffer3[1] = ComplexRug::new(zero, zero);
    let buffer4 = vec![ComplexRug::new(zero, zero); 4];
    buffer4[0] = ComplexRug::new(zero, zero);
    buffer4[1] = ComplexRug::new(zero, zero);
    let mut planner3 = FftPlannerScalar::new();
    let fft3 = planner3.plan_fft_forward(4);
    fft3.process(&mut buffer3);
    fft3.process(&mut buffer4);
}
