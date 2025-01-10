### Zero-Knowledge Age Verification

With this, I aim to build a simple **zero-knowledge proof (ZKP) system** for age verification. The goal is to prove that someone is of legal age (e.g., over 18) without spilling the beans on their birthdate or exact age. The ultimate "trust me, bro" but with math and cryptography backing it up

## Why Zero-Knowledge Proofs?

Zero-knowledge proofs allow one party (the prover) to prove to another party (the verifier) that a statement is true without revealing any additional information. In this case, the statement is:  
**"I am over the legal age."**

By using ZKPs, we can ensure that:
- The prover’s birthdate and exact age remain private.
- The verifier can trust the proof without needing to see sensitive information.

---

## Step 1: Understanding Finite Field Operations

Before building ZKP circuit, it’s essential to understand **finite field operations**, as they form the mathematical foundation of zero-knowledge proofs. Finite fields are used extensively in cryptography to perform arithmetic operations in a way that ensures security and correctness.

### What happened
In this first step, I implemented basic finite field operations using the `arkworks` library in Rust. Specifically:
1. Created a field element from a number.
2. Printed the field element.
3. Converted the field element to its underlying representation (a `BigInteger`).


### Why This Was Important
Understanding finite field operations is important because:

- Foundational Knowledge: Finite fields are the building blocks of cryptographic protocols, including ZKPs.

- Writing and running this code helped with getting familiar with the arkworks library, which I'll use to implement the ZKP circuit.

- Debugging and Testing: By working with simple field operations, I can debug and test our understanding before moving on to more complex tasks.

## Why `arkworks` Was Essential
#### Abstracting Low-Level Math:

Thanks to `arkworks`, I didn’t have to dive into low-level math like field operations and curve point calculations.
`arkworks` handles all the gnarly math (like modular arithmetic and field inversion) so I don’t have to. 🙌

I can focus on the fun stuff instead of reinventing the wheel.(no need)

#### Predefined Curves and Fields:

`arkworks` provides predefined curves (e.g., BLS12-381) and fields, which are essential for building ZKPs. This allowed me to focus on the high-level logic of the circuit instead of worrying about the underlying math.(again, no need of reinventing the wheel)

#### Efficient and Secure:

The library is optimized for performance and security, ensuring that the cryptographic operations are both fast and reliable.


## Next Steps
Now that we’ve got the basics down, here’s what’s next on the agenda:

#### 1. Define the ZKP Circuit
Create an arithmetic circuit that encodes the logic for age verification.

The circuit will take private inputs (birthdate, current_date) and a public input (legal_age).

It will compute `age = current_date - birthdate` and prove that `age >= legal_age`

#### 2. Implement the Circuit
Use the `arkworks` library to implement the circuit in Rust.

Define the constraints for the arithmetic operations (e.g., subtraction and comparison).

#### 3. Generate and Verify Proofs
Use the circuit to generate proofs for valid inputs.

Verify the proofs without revealing private information.

#### 4. Test the Circuit
Write unit tests to ensure the circuit behaves as expected.

Test edge cases, such as when `birthdate == current_date or age == legal_age.`

#### 5. Optimize the Circuit
Reduce the number of constraints to improve performance.

Use efficient field operations and range proofs.

#### 6. Integrate with a ZKP System
We’ll hook the circuit up to a ZKP system (like `Groth16` or `Marlin`) and make it production-ready. 

#### How to Run the Code
#### Clone the Repository:

`git clone https://github.com/wamimi/snarksnstarks.git`

`cd zk-age-verification`

#### Install Dependencies:
Make sure you have Rust installed. Then, install the project dependencies:

`cargo build`

Run the Finite Field Operations Example:


`cargo run`

You should see output like:

`field element: 42`

`underlying representation: 42`

#### Contributing
If you’re a ZK nerd (or just curious), feel free to contribute! 