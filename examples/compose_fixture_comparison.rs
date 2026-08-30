use std::{fs, path::Path};

use evmole::compose::{ComposeValidationInput, VirtualStorageLayout, validate_and_print};

const FIXTURES: &[&str] = &[
    "1-normal",
    "2-constant-key",
    "3-storage-key",
    "4-mapping-struct",
    "5-array-struct",
    "6-array-mapping-struct",
];

fn decode_hex(value: &str) -> Vec<u8> {
    let value = value.trim().trim_start_matches("0x");
    assert!(
        value.len().is_multiple_of(2),
        "bytecode must have an even length"
    );
    (0..value.len())
        .step_by(2)
        .map(|index| u8::from_str_radix(&value[index..index + 2], 16).expect("valid hex byte"))
        .collect()
}

fn main() {
    let fixture_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/evmole");

    for fixture in FIXTURES {
        let path = fixture_root.join(fixture);
        let bytecode =
            decode_hex(&fs::read_to_string(path.join("bytecode.txt")).expect("fixture bytecode"));
        let virtual_storage_layout = serde_json::from_str::<VirtualStorageLayout>(
            &fs::read_to_string(path.join("vsl.json")).expect("fixture VSL"),
        )
        .expect("valid VSL JSON");

        println!("\n========== {fixture} ==========");
        validate_and_print(&ComposeValidationInput {
            bytecode,
            virtual_storage_layout,
        });
    }
}
