use evmole::compose::{
    ComposeEngine, ComposeValidationInput, ComposeValidationStatus, VirtualStorageLayout,
    VirtualStorageLayoutKind, VirtualStorageLayoutRecord, VirtualStorageLayoutSource,
    validate_and_print,
};

fn decode_hex(value: &str) -> Vec<u8> {
    let value = value.trim().trim_start_matches("0x");
    (0..value.len())
        .step_by(2)
        .map(|index| u8::from_str_radix(&value[index..index + 2], 16).unwrap())
        .collect()
}

fn normal_layout() -> VirtualStorageLayoutRecord {
    let layout = [
        "0x2f", "0x53", "0xf4", "0x53", "0xf4", "0x53", "0x17", "0x10", "0xff", "0x13", "0xff",
        "0xf1", "0x03", "0x2f", "0xff", "0xf1", "0x03", "0xf1", "0x03", "0x2f", "0xff", "0xf2",
        "0x10", "0xff",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<Vec<_>>();
    let mut serialized_layout = vec!["0x01".to_owned()];
    serialized_layout.extend(layout.iter().cloned());

    VirtualStorageLayoutRecord {
        id: "0xb4df32537f6767405c9db7d67260e5375218aecdea91f4240ad14000623cbdff".to_owned(),
        virtual_path: "evmole.normal".to_owned(),
        parent_virtual_path: None,
        kind: VirtualStorageLayoutKind::Normal,
        code_width: 1,
        layout,
        serialized_layout,
        slots: vec![
            vec![256],
            vec![32],
            vec![32],
            vec![32, 64, 8],
            vec![32],
            vec![256],
            vec![256],
            vec![256],
        ],
        source: VirtualStorageLayoutSource::SlotAssignment,
        source_name: "tests/fixtures/evmole/1-normal/Normal.sol".to_owned(),
        contract_name: "Normal".to_owned(),
        struct_name: Some("NormalStorage".to_owned()),
        diamond_name: None,
    }
}

#[test]
fn compares_unbiased_and_vsl_biased_inference() {
    let input = ComposeValidationInput {
        bytecode: decode_hex(include_str!("fixtures/evmole/1-normal/bytecode.txt")),
        virtual_storage_layout: VirtualStorageLayout {
            records: vec![normal_layout()],
        },
    };

    let report = validate_and_print(&input);

    assert_eq!(report.compose.engine, ComposeEngine::Compose);
    assert_eq!(
        report.compose_vsl_bias.engine,
        ComposeEngine::ComposeVslBias
    );
    assert_eq!(
        report.compose.status,
        ComposeValidationStatus::NoContradiction
    );
    assert_eq!(
        report.compose_vsl_bias.status,
        ComposeValidationStatus::NoContradiction
    );
    assert_eq!(report.compose.observations.len(), 10);
    assert!(report.compose.assumptions.is_empty());
    assert!(report.compose_vsl_bias.assumptions.is_empty());
    assert!(report.compose.observations.iter().any(|observation| {
        observation.inferred_type == "bytes4"
            && observation.candidate_types == ["bytes4", "uint256"]
    }));
}
