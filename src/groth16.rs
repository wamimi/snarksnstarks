use ark_groth16::{generate_random_parameters,create_random_proof,verify_proof,prepare_verifying_key,ProvingKey,VerifyingKey};
use ark_ff::Field;
use ark_bls12_381::{Bls12_381, Fr};
use ark_std::rand::thread_rng;

use crate::AgeVerificationCircuit;

pub struct Groth16AgeVerifier{
    proving_key: ProvingKey<Bls12_381>,
    verifying_key: VerifyingKey<Bls12_381>
}
impl Groth16AgeVerifier{
    pub fn new(legal_age: Fr) -> Result<Self, Box<dyn std::error::Error>>{
        let rng = &mut thread_rng();

        // dummy circuit for key generation
        let circuit = AgeVerificationCircuit{
            birthdate: Some(Fr::from(0u32)),
            current_date: Some(Fr::from(0u32)),
            legal_age,
        };
        //Generate proving and veryfying keys
        let params = generate_random_parameters::<Bls12_381, _, _>(circuit,rng)?;
        Ok(Self{
            proving_key:params.0,
            verifying_key:params.1,
        })
        
    }
    // generating a proof that a person is of legal age
    pub fn generate_proof(
        &self, 
        birthdate: Fr, 
        current_date: Fr,
        legal_age: Fr,) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let rng = &mut thread_rng();
    }
}


