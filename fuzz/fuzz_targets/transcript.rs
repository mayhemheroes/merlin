#![no_main]
use libfuzzer_sys::fuzz_target;
use merlin::Transcript;

static DATA: &[u8; 32] = &[0; 32];
static LABEL: &[u8; 32] = &[0; 32];

fuzz_target!(|value: &[u8]| {
    let mut transcript = Transcript::new(DATA);
    transcript.append_message(LABEL, &value);
});
