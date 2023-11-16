use methods::{
    PROVER_ELF, PROVER_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv};
use std::error::Error;
use fhe::{
  bfv::{BfvParametersBuilder, SecretKey, Encoding, PublicKey, Plaintext, Ciphertext},
  mbfv::{CommonRandomPoly, PublicKeyShare, AggregateIter, DecryptionShare}
};
use fhe_traits::{Serialize, DeserializeParametrized, FheEncoder, FheDecoder, FheEncrypter};
use rand::{thread_rng, rngs::OsRng, distributions::{Distribution, Uniform}};
use std::sync::Arc;

// Assuming 5 voters in this example
#[derive(serde::Serialize, serde::Deserialize)]
struct Input {
  ciph1: Vec<u8>,
  ciph2: Vec<u8>,
  ciph3: Vec<u8>,
  ciph4: Vec<u8>,
  ciph5: Vec<u8>
}

fn main() -> Result<(), Box<dyn Error>> {
// Voting example from https://github.com/tlepoint/fhe.rs/blob/main/crates/fhe/examples/voting.rs

    let degree = 4096;
    let plaintext_modulus: u64 = 4096;
    let moduli = vec![0xffffee001, 0xffffc4001, 0x1ffffe0001];
    
    // max 1000 in example
    let num_voters = 5;
    // max 10 in example
    let num_parties = 3;
    let mut rng = thread_rng();

    let params = BfvParametersBuilder::new()
      .set_degree(degree)
      .set_plaintext_modulus(plaintext_modulus)
      .set_moduli(&moduli)
      .build_arc()?;

    let crp = CommonRandomPoly::new(&params, &mut rng)?;

    struct Party {
      sk_share: SecretKey,
      pk_share: PublicKeyShare,
    }
    let mut parties = Vec::with_capacity(num_parties);
    for _ in 0..num_parties {
      let sk_share = SecretKey::random(&params, &mut OsRng);
      let pk_share = PublicKeyShare::new(&sk_share, crp.clone(), &mut rng)?;
      parties.push(Party { sk_share, pk_share });
    }

    let pk: PublicKey = parties.iter().map(|p| p.pk_share.clone()).aggregate()?;

    // Vote casting
    let dist = Uniform::new_inclusive(0, 1);
    let votes: Vec<u64> = dist
        .sample_iter(&mut rng)
        .take(num_voters)
        .collect();
    let mut votes_encrypted = Vec::with_capacity(num_voters);
    for _i in 0..num_voters {
      let pt = Plaintext::try_encode(&[votes[_i]], Encoding::poly(), &params)?;
      let ct = pk.try_encrypt(&pt, &mut rng)?;
      votes_encrypted.push(ct);
    }

    let input = Input { 
      ciph1: votes_encrypted[0].to_bytes(), 
      ciph2: votes_encrypted[1].to_bytes(),
      ciph3: votes_encrypted[2].to_bytes(),
      ciph4: votes_encrypted[3].to_bytes(),
      ciph5: votes_encrypted[4].to_bytes()
    };

    let env = ExecutorEnv::builder()
      .write_slice(&bincode::serialize(&input).unwrap())
      .build().unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, PROVER_ELF).unwrap();

    // Deserialize resulting ciphertext
    let result: Vec<u8> = receipt.journal.decode().unwrap();
    let calculated_sum: Ciphertext = Ciphertext::from_bytes(&result, &params).unwrap();
    
    receipt.verify(PROVER_ID).unwrap();
    
    let tally = Arc::new(calculated_sum);

    let mut decryption_shares = Vec::with_capacity(num_parties);
    for _i in 0..num_parties {
        let sh = DecryptionShare::new(&parties[_i].sk_share, &tally, &mut thread_rng())?;
        decryption_shares.push(sh);
    }

    let tally_pt: Plaintext = decryption_shares.into_iter().aggregate()?;
    let tally_vec = Vec::<u64>::try_decode(&tally_pt, Encoding::poly())?;
    let tally_result = tally_vec[0];
    
    // Show vote result
    println!("Vote result = {} / {}", tally_result, num_voters);

    let expected_tally: u64 = votes.iter().sum();
    assert_eq!(tally_result, expected_tally);

    Ok(())
}
