use std::{fs, path::Path};

use evmole::storage_validation::{StorageValidationInput, VirtualStorageLayout, validate};

const FIXTURES: &[&str] = &[
    "FullStorageFacet",
    "CompatibleStorageFacet",
    "IncompatibleStorageFacet",
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
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/foundry-out");
    for fixture in FIXTURES {
        let artifact_path = root.join(format!("{fixture}.sol/{fixture}.json"));
        let artifact: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&artifact_path).expect("Foundry artifact"))
                .expect("valid Foundry artifact JSON");
        let bytecode = decode_hex(
            artifact["deployedBytecode"]["object"]
                .as_str()
                .expect("deployed runtime bytecode"),
        );
        let virtual_storage_layout = serde_json::from_str::<VirtualStorageLayout>(
            &fs::read_to_string(root.join("FullStorageFacet.vsl.json"))
                .expect("canonical FullStorageFacet VSL"),
        )
        .expect("valid canonical VSL JSON");

        println!("\n========== {fixture} ==========");
        println!(
            "{}",
            validate(&StorageValidationInput {
                bytecode,
                virtual_storage_layout,
            })
        );
    }
}
