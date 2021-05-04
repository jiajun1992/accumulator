use accumulator::group::Rsa2048;
use accumulator::hash;
use accumulator::Accumulator;
use ark_poly::univariate;
use rug::Integer;

use accumulator::hash::qc_hash;
use std::str::FromStr;
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
    let poly3 = poly1 * poly2;
    dbg!(&poly3);
    // not implemented FFT field!
    // A network participant who sees (acc, proof, and ["dog", "cat"]) can verify that the update
    // was formed correctly ...
    assert!(acc.verify_membership_batch(&["dog", "cat"], &proof));

    // ... and trying to verify something that has not been accumulated will fail.
    assert!(!acc.verify_membership(&"cow", &proof));
}
