use std::{fs, path::Path};

use evmole::{ContractInfoArgs, contract_info};

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

fn hex(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(2 + bytes.len() * 2);
    output.push_str("0x");
    for byte in bytes {
        output.push_str(&format!("{byte:02x}"));
    }
    output
}

fn main() {
    let fixture_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/evmole");

    for fixture in FIXTURES {
        let bytecode = decode_hex(
            &fs::read_to_string(fixture_root.join(fixture).join("bytecode.txt"))
                .expect("fixture bytecode"),
        );
        let contract = contract_info(
            ContractInfoArgs::new(&bytecode)
                .with_selectors()
                .with_arguments()
                .with_state_mutability()
                .with_storage(),
        );

        println!("\n[{fixture}]");
        for record in contract.storage.unwrap_or_default() {
            let reads = record
                .reads
                .iter()
                .map(|selector| hex(selector))
                .collect::<Vec<_>>()
                .join(", ");
            let writes = record
                .writes
                .iter()
                .map(|selector| hex(selector))
                .collect::<Vec<_>>()
                .join(", ");

            println!(
                "slot={} offset={} type={} reads=[{}] writes=[{}]",
                hex(&record.slot),
                record.offset,
                record.r#type,
                reads,
                writes,
            );
        }
    }
}
