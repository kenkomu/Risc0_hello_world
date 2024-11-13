use methods::{HELLO_GUEST_ELF, HELLO_GUEST_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};

fn main() {
    let input: u32 = 15 * u32::pow(2, 27) + 1;
    let env = ExecutorEnv::builder().write(&input).unwrap().build().unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, HELLO_GUEST_ELF).unwrap().receipt;

    // Extract journal of receipt
    let output: u32 = receipt.journal.decode().unwrap();

    // Print, notice, after committing to a journal, the private input became public
    println!("Hello, world! I generated a proof of guest execution! {} is a public output from journal ", output);
}