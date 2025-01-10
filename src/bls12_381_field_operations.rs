use ark_ff::{Field, PrimeField};
use ark_test_curves::bls12_381::Fq;
use ark_std::{One, UniformRand};

fn main() {
    // Creating a field element from a number
    let field_element = Fq::from(42u64);
    // let s = string from:('hello")

    // Printing the field element
    println!("field element: {:?}", field_element);

    // Converting the field element to its underlying representation
    let repr = field_element.into_bigint();
    println!("underlying representation: {:?}", repr);
}
