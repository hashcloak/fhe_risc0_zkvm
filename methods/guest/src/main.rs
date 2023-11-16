#![no_main]

use std::io::Read;
use fhe::bfv::{Ciphertext, BfvParametersBuilder};
use fhe_traits::{DeserializeParametrized, Serialize};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

// Assuming 5 voters in this example
#[derive(serde::Serialize, serde::Deserialize)]
struct Input {
  ciph1: Vec<u8>,
  ciph2: Vec<u8>,
  ciph3: Vec<u8>,
  ciph4: Vec<u8>,
  ciph5: Vec<u8>
}

pub fn main() {
    let start: usize = env::get_cycle_count();

    let degree = 4096;
    let plaintext_modulus: u64 = 4096;
    let moduli = vec![0xffffee001, 0xffffc4001, 0x1ffffe0001];
    
    let params = match BfvParametersBuilder::new()
      .set_degree(degree)
      .set_plaintext_modulus(plaintext_modulus)
      .set_moduli(&moduli)
      .build_arc() {
        Ok(params) => params,
        Err(e) => panic!("Failed to build parameters: {:?}", e)
    };

    // read the input from host
    let mut input_bytes = Vec::<u8>::new();
    env::stdin().read_to_end(&mut input_bytes).unwrap();
    let input: Input = bincode::deserialize(&input_bytes).unwrap();
    
    // Deserialize ciphertexts
    let ciph1: Ciphertext = Ciphertext::from_bytes(&input.ciph1, &params).unwrap();
    let ciph2: Ciphertext = Ciphertext::from_bytes(&input.ciph2, &params).unwrap();
    let ciph3: Ciphertext = Ciphertext::from_bytes(&input.ciph3, &params).unwrap();
    let ciph4: Ciphertext = Ciphertext::from_bytes(&input.ciph4, &params).unwrap();
    let ciph5: Ciphertext = Ciphertext::from_bytes(&input.ciph5, &params).unwrap();

    let mut sum = Ciphertext::zero(&params);
    sum += &ciph1;
    sum += &ciph2;
    sum += &ciph3;
    sum += &ciph4;
    sum += &ciph5;
    
    env::commit(&sum.to_bytes());

    let end = env::get_cycle_count();
    // 1.693.547.928, 1.693.544.600
    eprintln!("total cycle count: {}", end - start);
}
