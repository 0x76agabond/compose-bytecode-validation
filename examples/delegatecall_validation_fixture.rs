use std::{env, fs, path::Path};

use evmole::storage_validation::{
    DelegateCallValidationContext, HttpRpcCodeSource, RuntimeCodeSource, StorageValidationInput,
    VirtualStorageLayout, validate_with_delegate_calls,
};

fn parse_address(value: &str) -> [u8; 20] {
    let value = value.trim().strip_prefix("0x").unwrap_or(value.trim());
    assert_eq!(value.len(), 40, "address must contain 20 bytes");
    let bytes = alloy_primitives::hex::decode(value).expect("valid hexadecimal address");
    bytes.try_into().expect("20-byte address")
}

fn main() {
    let rpc_url = env::var("COMPOSE_RPC_URL").expect("COMPOSE_RPC_URL is required");
    let caller_address = parse_address(
        &env::var("COMPOSE_CALLER_ADDRESS").expect("COMPOSE_CALLER_ADDRESS is required"),
    );
    let storage_address = env::var("COMPOSE_STORAGE_ADDRESS")
        .map(|address| parse_address(&address))
        .unwrap_or(caller_address);
    let vsl_path = env::var("COMPOSE_VSL").expect("COMPOSE_VSL is required");
    let expected_collisions = env::var("COMPOSE_EXPECT_COLLISIONS")
        .expect("COMPOSE_EXPECT_COLLISIONS is required")
        .parse::<usize>()
        .expect("COMPOSE_EXPECT_COLLISIONS must be an integer");
    let expected_delegatecall_warnings = env::var("COMPOSE_EXPECT_DELEGATECALL_WARNINGS")
        .unwrap_or_else(|_| "0".to_owned())
        .parse::<usize>()
        .expect("COMPOSE_EXPECT_DELEGATECALL_WARNINGS must be an integer");

    let source = HttpRpcCodeSource::new(rpc_url, "latest");
    let bytecode = source
        .code_at(caller_address)
        .expect("caller runtime bytecode");
    let virtual_storage_layout = serde_json::from_str::<VirtualStorageLayout>(
        &fs::read_to_string(Path::new(&vsl_path)).expect("canonical VSL"),
    )
    .expect("valid canonical VSL JSON");

    let report = validate_with_delegate_calls(
        &StorageValidationInput {
            bytecode,
            virtual_storage_layout,
        },
        &source,
        DelegateCallValidationContext::new(storage_address),
    );
    println!("{report}");
    assert_eq!(
        report.collisions.len(),
        expected_collisions,
        "delegatecall fixture produced an unexpected collision count"
    );
    assert_eq!(
        report.delegatecall_warnings.len(),
        expected_delegatecall_warnings,
        "delegatecall fixture produced an unexpected warning count: {:?}",
        report.delegatecall_warnings
    );
}
