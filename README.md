# FHE in Risc0 zkVM

Integration of [fhe.rs](https://github.com/tlepoint/fhe.rs/tree/main), a FHE library in Rust, in [Risc0 zkVM](https://github.com/risc0/risc0). 

A small voting example is implemented, where the homomorphic addition of the ciphertexts is done in the zkVM guest. The example comes from the fhe.rs library ([here](https://github.com/tlepoint/fhe.rs/blob/main/crates/fhe/examples/voting.rs)). The original example has a voters max of 1000, in this repo a fixed amount of 5 voters is used to showcase the functionality. 

A fork of the [fhe.rs](https://github.com/tlepoint/fhe.rs/tree/main) library is being used, where the Rust version aligns with the accepted Rust version in the zkVM guest ([here](https://github.com/ewynx/fhe.rs/tree/37f3d22e6633b043026f93bbb809c893606bc361)). 

__Note:__ the current example can run with 
```bash
RISC0_DEV_MODE=1 cargo run
```
Running it in full proving mode probably hits a hardware limit and running in Bonsai probably hits a quota limit. 


## Quick Start

Init and update submodule:
```
git submodule init
git submodule update
```

To build all methods and execute the method within the zkVM, run the following command:

```bash
cargo run
```

To run in developer mode, without generating valid proofs (much faster):

```bash
RISC0_DEV_MODE=1 cargo run
```

### Running proofs remotely on Bonsai

If you have access to the URL and API key to Bonsai you can run your proofs
remotely. To prove in Bonsai mode, invoke `cargo run` with two additional
environment variables:

```bash
BONSAI_API_KEY="YOUR_API_KEY" BONSAI_API_URL="BONSAI_URL" cargo run
```
