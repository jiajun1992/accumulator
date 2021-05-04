use accumulator::group::Rsa2048;
use accumulator::Accumulator;
fn main() {
    println!("Hello, world!");
    let acc = Accumulator::<Rsa2048, &'static str>::empty();

    // Accumulate "dog" and "cat". The `add_with_proof` method returns the new accumulator state
    // and a proof that you accumulated "dog" and "cat".
    let (acc, proof) = acc.add_with_proof(&["dog", "cat"]);
    dbg!(&acc);
    // A network participant who sees (acc, proof, and ["dog", "cat"]) can verify that the update
    // was formed correctly ...
    assert!(acc.verify_membership_batch(&["dog", "cat"], &proof));

    // ... and trying to verify something that has not been accumulated will fail.
    assert!(!acc.verify_membership(&"cow", &proof));
}
