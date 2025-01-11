
// Example circuit(hope it works)
use ark_ff::{Field, PrimeField};
use ark_relations::r1cs::{ConstraintSynthesizer,ConstraintSystem, SynthesisError};
use ark_relations::{lc, ns};


struct AgeVerificationCircuit<F: Field> {
    birthdate: Option<F>,       // Private input
    current_date: Option<F>,   // Private input
    legal_age: F,              // Public input
}

impl<F: Field> ConstraintSynthesizer<F> for AgeVerificationCircuit<F> {
    fn generate_constraints(
        &self,
        cs: &mut ConstraintSystem<F>,
    ) -> Result<(), SynthesisError> {
        // Allocate private inputs
        let birthdate = cs.alloc_input(|| "birthdate", || self.birthdate.ok_or(SynthesisError::AssignmentMissing))?;
        let current_date = cs.alloc_input(|| "current_date", || self.current_date.ok_or(SynthesisError::AssignmentMissing))?;

        // Computing age
        let age = cs.alloc(|| "age", || {
            let b = self.birthdate.ok_or(SynthesisError::AssignmentMissing)?;
            let c = self.current_date.ok_or(SynthesisError::AssignmentMissing)?;
            Ok(c - b)
        })?;

        // Proving age >= legal_age
        cs.enforce_constraint(
            lc!() + age,
            lc!() + CS::one(),
            lc!() + self.legal_age,
        );

        Ok(())
    }
}

// so first we have a circuit struct `AgeVerificationCircuit` 
// it hold the private inputs and the public output(see comments above)
// then we define a constraint system `synthesize` for the circuit
// then we allocate the private inputs
// compute the age
// then enforce the constraint

// then a range proof in this case `age >=lega_age`
// then we test(fingers crossed)

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::Field;
    use ark_test_curves::bls12_381::Fq;

    #[test]
    fn test_age_verification_circuit() {
        // Defined inputs
        let birthdate = Fq::from(1000u64); // Birthdate: 1000 days since epoch
        let current_date = Fq::from(8000u64); // Current date: 8000 days since epoch
        let legal_age = Fq::from(18 * 365u64); // Legal age: 18 years in days

        // Creating the circuit
        let circuit = AgeVerificationCircuit {
            birthdate: Some(birthdate),
            current_date: Some(current_date),
            legal_age,
        };

        // Tesingt the circuit
        let mut cs = ConstraintSystem::new();
        circuit.synthesize(&mut cs).unwrap();

        // Checking that the constraints are satisfied
        assert!(cs.is_satisfied().is_ok());
    }
}
