use std::{fs, path::Path};

use evmole::storage_validation::{StorageValidationInput, VirtualStorageLayout, validate};

struct Variant {
    name: &'static str,
    source: &'static str,
    contract: &'static str,
    canonical: bool,
    must_collide: bool,
}

struct Case {
    name: &'static str,
    directory: &'static str,
    variants: &'static [Variant],
}

const CASES: &[Case] = &[
    Case {
        name: "1-normal",
        directory: "1-normal",
        variants: &[
            Variant {
                name: "canonical",
                source: "Canonical.sol",
                contract: "Case1Canonical",
                canonical: true,
                must_collide: false,
            },
            Variant {
                name: "incompatible-packed-width",
                source: "Incompatible.sol",
                contract: "Case1Incompatible",
                canonical: false,
                must_collide: false,
            },
        ],
    },
    Case {
        name: "2-constant-key",
        directory: "2-constant-key",
        variants: &[
            Variant {
                name: "canonical",
                source: "Canonical.sol",
                contract: "Case2Canonical",
                canonical: true,
                must_collide: false,
            },
            Variant {
                name: "incompatible-dynamic-width",
                source: "Incompatible.sol",
                contract: "Case2Incompatible",
                canonical: false,
                must_collide: true,
            },
        ],
    },
    Case {
        name: "3-storage-key",
        directory: "3-storage-key",
        variants: &[
            Variant {
                name: "canonical",
                source: "Canonical.sol",
                contract: "Case3Canonical",
                canonical: true,
                must_collide: false,
            },
            Variant {
                name: "incompatible-dynamic-widths",
                source: "IncompatibleDynamic.sol",
                contract: "Case3IncompatibleDynamic",
                canonical: false,
                must_collide: true,
            },
            Variant {
                name: "incompatible-fixed-widths",
                source: "IncompatibleFixed.sol",
                contract: "Case3IncompatibleFixed",
                canonical: false,
                must_collide: true,
            },
        ],
    },
    Case {
        name: "4-mapping-struct",
        directory: "4-mapping-struct",
        variants: &[
            Variant {
                name: "canonical",
                source: "Canonical.sol",
                contract: "Case4Canonical",
                canonical: true,
                must_collide: false,
            },
            Variant {
                name: "incompatible-member-order",
                source: "Incompatible.sol",
                contract: "Case4Incompatible",
                canonical: false,
                must_collide: false,
            },
        ],
    },
    Case {
        name: "5-array-struct",
        directory: "5-array-struct",
        variants: &[
            Variant {
                name: "canonical",
                source: "Canonical.sol",
                contract: "Case5Canonical",
                canonical: true,
                must_collide: false,
            },
            Variant {
                name: "incompatible-member-order",
                source: "IncompatibleReordered.sol",
                contract: "Case5IncompatibleReordered",
                canonical: false,
                must_collide: false,
            },
            Variant {
                name: "incompatible-address-array",
                source: "IncompatibleAddress.sol",
                contract: "Case5IncompatibleAddress",
                canonical: false,
                must_collide: false,
            },
        ],
    },
    Case {
        name: "6-array-mapping-struct",
        directory: "6-array-mapping-struct",
        variants: &[
            Variant {
                name: "canonical-mapping-address",
                source: "Canonical.sol",
                contract: "Case6Canonical",
                canonical: true,
                must_collide: false,
            },
            Variant {
                name: "incompatible-mapping-array-struct",
                source: "IncompatibleOnlyArray.sol",
                contract: "Case6IncompatibleOnlyArray",
                canonical: false,
                must_collide: false,
            },
            Variant {
                name: "incompatible-mapping-array-and-fields",
                source: "IncompatibleArrayAndFields.sol",
                contract: "Case6IncompatibleArrayAndFields",
                canonical: false,
                must_collide: false,
            },
        ],
    },
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
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let artifact_root = root.join("tests/fixtures/foundry-out");
    let fixture_root = root.join("tests/fixtures/evmole");
    let selected_case = std::env::var("COMPOSE_FIXTURE_CASE").ok();

    for case in CASES {
        if selected_case
            .as_deref()
            .is_some_and(|selected| selected != case.name)
        {
            continue;
        }
        let virtual_storage_layout = serde_json::from_str::<VirtualStorageLayout>(
            &fs::read_to_string(fixture_root.join(case.directory).join("canonical-vsl.json"))
                .expect("canonical fixture VSL"),
        )
        .expect("valid canonical VSL JSON");

        println!("\n================ {} ================", case.name);
        for variant in case.variants {
            let artifact_path = artifact_root
                .join(variant.source)
                .join(format!("{}.json", variant.contract));
            let artifact: serde_json::Value =
                serde_json::from_str(&fs::read_to_string(&artifact_path).unwrap_or_else(|error| {
                    panic!("cannot read {}: {error}", artifact_path.display())
                }))
                .expect("valid Foundry artifact JSON");
            let bytecode = decode_hex(
                artifact["deployedBytecode"]["object"]
                    .as_str()
                    .expect("deployed runtime bytecode"),
            );

            println!("\n--- {} ---", variant.name);
            let report = validate(&StorageValidationInput {
                bytecode,
                virtual_storage_layout: virtual_storage_layout.clone(),
            });
            println!("{report}");

            if variant.canonical {
                assert!(
                    report.collisions.is_empty(),
                    "{} canonical bytecode contradicts its own VSL",
                    case.name
                );
            } else if variant.must_collide {
                assert!(
                    !report.collisions.is_empty(),
                    "{} / {} must produce a proven collision",
                    case.name,
                    variant.name
                );
            } else {
                assert!(
                    !report.collisions.is_empty() || !report.uncertain_scopes.is_empty(),
                    "{} / {} produced no contradiction or scoped uncertainty",
                    case.name,
                    variant.name
                );
            }
        }
    }
}
