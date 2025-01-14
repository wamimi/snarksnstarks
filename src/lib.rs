use ark_ff::Field;
use ark_relations::r1cs::{ConstraintSynthesizer,ConstraintSystemRef, SynthesisError,Variable};
use ark_relations::lc;


// the ageverificationcircuit struct definition
struct AgeVerificationCircuit<F: Field>{
    birthdate: Option<F>, // private
    current_date: Option<F>,// private
    legal_age: F, // public
}

    // implementation of constrainsynthesizer trait for the circuit
    impl<F: Field> ConstraintSynthesizer<F> for AgeVerificationCircuit<F>{
        fn generate_constraints(
            self,
            cs: ConstraintSystemRef<F>,
        ) -> Result<(), SynthesisError>{
            
            // // lets create a namespace for better oraganization
            // // sice the namespace method is not in the enum ConstarintSystemref
            // let ns = cs.namespace(|| "age_verification");
            
            
            // allocating private inputs
            let birthdate_var = cs.new_witness_variable(|| {
                self.birthdate.ok_or(SynthesisError::AssignmentMissing)
            })?;

            let current_date_var = cs.new_witness_variable(||{
                self.current_date.ok_or(SynthesisError::AssignmentMissing)
            })?;

            // let compute the age age = currentdate-birthdate
            let age_var = cs.new_witness_variable(||{
                let b = self.birthdate.ok_or(SynthesisError::AssignmentMissing)?;
                let c = self.current_date.ok_or(SynthesisError::AssignmentMissing)?;
                Ok(c-b)
            })?;

            // creating a public input for legal age
            let legal_age_var = cs.new_input_variable(|| Ok(self.legal_age))?;

            // Enforce that current_date - birthdate = age
            cs.enforce_constraint(
                lc!() + current_date_var - birthdate_var,
                lc!() + (F::one(),Variable::One),
                lc!() + age_var,
            )?;

            // Enforce that age >= legal_age
            // This is done by checking if age - legal_age >= 0
            let difference = cs.new_witness_variable(||{
                let age = self.current_date.ok_or(SynthesisError::AssignmentMissing)?-
                self.birthdate.ok_or(SynthesisError::AssignmentMissing)?;
                let legal = self.legal_age;
                Ok(age - legal)
            })?;

            // enforcing that age - legal_age = difference
            cs.enforce_constraint(
                lc!() + age_var - legal_age_var,
                lc!() + (F::one(), Variable::One),
                lc!() + difference,


            )?;

            // enforcing that difference * difference = difference(this ensures difference >= 0)
            cs.enforce_constraint(
                lc!() + difference,
                lc!() + difference,
                lc!() + difference,
            )?;
            // cs.enforce_constraint(
            //     lc!() + age_var - legal_age_var,
            //     lc!() + (F::one(), Variable::One),
            //     lc!() + age_var - legal_age_var,
            // )?;

            Ok(())
        }
    }

    // Testing module
    #[cfg(test)]
    mod tests{
        use super::*;
        use ark_test_curves::bls12_381::Fr as Fq;
        use ark_relations::r1cs::ConstraintSystem;
        #[test]
        fn test_age_verification_circuit(){
            // valid case where age is exactly legal age
            let birthdate = Fq::from(1000u64); // 1000 dyas since epoch
            let current_date = Fq::from(7570u64); // 8000 days since epoch
            let legal_age = Fq::from(18 * 365u64); // 18 yrs in days
            let circuit = AgeVerificationCircuit{
                birthdate : Some(birthdate),
                current_date: Some(current_date),
                legal_age,
            };

            let cs = ConstraintSystem::new_ref();
            circuit.generate_constraints(cs.clone()).unwrap();
            assert!(cs.is_satisfied().unwrap());


            // edge case when age is just below legal age

            let birthdate = Fq::from(1001u64); // 1001 days since epoch
            let current_date = Fq::from(7570u64); // 8000 days since epoch
            // let legal_age = Fq::from(18 * 365u64); // 18 yrs in days
            let circuit = AgeVerificationCircuit{
                birthdate : Some(birthdate),
                current_date: Some(current_date),
                legal_age,
            };

            let cs = ConstraintSystem::new_ref();
        circuit.generate_constraints(cs.clone()).unwrap();
        assert!(!cs.is_satisfied().unwrap());

        // edge case where the currentdate is before the birthdate
        let birthdate = Fq::from(8000u64); // birthdate: 8000 days since epoch
        let current_date = Fq::from(1000u64); //current date: 1000 days since epoch

        let circuit = AgeVerificationCircuit {
            birthdate: Some(birthdate),
            current_date: Some(current_date),
            legal_age,
        };

        let cs = ConstraintSystem::new_ref();
        circuit.generate_constraints(cs.clone()).unwrap();
        assert!(!cs.is_satisfied().unwrap());

        // edge case where age is zero like the birthdate is same as current date
        let birthdate = Fq::from(8000u64); // birthdate: 8000 days since epoch
        let current_date = Fq::from(8000u64); // current date: 8000 days since epoch

        let circuit = AgeVerificationCircuit {
            birthdate: Some(birthdate),
            current_date: Some(current_date),
            legal_age,
        };

        let cs = ConstraintSystem::new_ref();
        circuit.generate_constraints(cs.clone()).unwrap();
        assert!(!cs.is_satisfied().unwrap());
        // edge case where there are very large values for birthdate and currentdate
        let birthdate = Fq::from(u64::MAX); // birthdate: maximum u64 value
        let current_date = Fq::from(u64::MAX - 1); // current date: one day before maximum
        let legal_age = Fq::from(18 * 365u64); // legal age 18 years in days

        let circuit = AgeVerificationCircuit {
            birthdate: Some(birthdate),
            current_date: Some(current_date),
            legal_age,
        };

        let cs = ConstraintSystem::new_ref();
        circuit.generate_constraints(cs.clone()).unwrap();
        assert!(!cs.is_satisfied().unwrap());

        }
    }


