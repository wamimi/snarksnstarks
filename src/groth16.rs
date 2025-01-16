use ark_groth16::{generate_random_parameters,create_random_proof,verify_proof,prepare_verifying_key,ProvingKey,VerifyingKey};
use ark_ff::Field;
use ark_bls12_381::{Bls12_381, Fr};
use ark_std::rand::Rng;
use crate::AgeVerificationCircuit;

pub struct Groth16Verifier{
    proving_key,
    verifying_key,
}
impl Groth16Verifier{
    pub fn new(legal_)
}


