#![no_main]
use libfuzzer_sys::fuzz_target;
use merlin::Transcript;
use rand_core::{OsRng, RngCore};

static DATA: &[u8; 32] = &[0; 32];
static LABEL: &[u8; 32] = &[0; 32];

fuzz_target!(|value: (&[u8], &[u8])| {
    let mut transcript = Transcript::new(DATA);
    transcript.append_message(LABEL, &value.0);
    let transcript_rng = transcript.build_rng().rekey_with_witness_bytes(LABEL, value.1);
    let mut rng = transcript_rng.finalize(&mut OsRng);
    rng.next_u32();
});
