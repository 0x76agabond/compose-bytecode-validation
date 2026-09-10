use std::{fs, path::Path};

use evmole::storage_validation::{StorageValidationInput, VirtualStorageLayout, validate};

struct Variant {
    name: &'static str,
    source: &'static str,
    contract: &'static str,
    expectation: Expectation,
}

#[derive(Clone, Copy)]
enum Expectation {
    Canonical,
    Collision,
    CollisionOrUncertain,
    CompatibleEvidence,
    ByteStringUncertainty,
}

struct Case {
    name: &'static str,
    directory: &'static str,
    minimum_canonical_validated: usize,
    maximum_canonical_uncertain: usize,
    variants: &'static [Variant],
}

const CASES: &[Case] = &[
    Case {
        name: "1-normal",
        directory: "1-normal",
        minimum_canonical_validated: 11,
        maximum_canonical_uncertain: 0,
        variants: &[
            Variant {
                name: "canonical",
                source: "Canonical.sol",
                contract: "Case1Canonical",
                expectation: Expectation::Canonical,
            },
            Variant {
                name: "incompatible-packed-width",
                source: "Incompatible.sol",
                contract: "Case1Incompatible",
                expectation: Expectation::CollisionOrUncertain,
            },
        ],
    },
    Case {
        name: "2-constant-key",
        directory: "2-constant-key",
        minimum_canonical_validated: 4,
        maximum_canonical_uncertain: 0,
        variants: &[
            Variant {
                name: "canonical",
                source: "Canonical.sol",
                contract: "Case2Canonical",
                expectation: Expectation::Canonical,
            },
            Variant {
                name: "incompatible-dynamic-width",
                source: "Incompatible.sol",
                contract: "Case2Incompatible",
                expectation: Expectation::Collision,
            },
        ],
    },
    Case {
        name: "3-storage-key",
        directory: "3-storage-key",
        minimum_canonical_validated: 8,
        maximum_canonical_uncertain: 0,
        variants: &[
            Variant {
                name: "canonical",
                source: "Canonical.sol",
                contract: "Case3Canonical",
                expectation: Expectation::Canonical,
            },
            Variant {
                name: "incompatible-dynamic-widths",
                source: "IncompatibleDynamic.sol",
                contract: "Case3IncompatibleDynamic",
                expectation: Expectation::Collision,
            },
            Variant {
                name: "incompatible-fixed-widths",
                source: "IncompatibleFixed.sol",
                contract: "Case3IncompatibleFixed",
                expectation: Expectation::Collision,
            },
        ],
    },
    Case {
        name: "4-mapping-struct",
        directory: "4-mapping-struct",
        minimum_canonical_validated: 5,
        maximum_canonical_uncertain: 0,
        variants: &[
            Variant {
                name: "canonical",
                source: "Canonical.sol",
                contract: "Case4Canonical",
                expectation: Expectation::Canonical,
            },
            Variant {
                name: "incompatible-member-order",
                source: "Incompatible.sol",
                contract: "Case4Incompatible",
                expectation: Expectation::Collision,
            },
        ],
    },
    Case {
        name: "5-array-struct",
        directory: "5-array-struct",
        minimum_canonical_validated: 9,
        maximum_canonical_uncertain: 0,
        variants: &[
            Variant {
                name: "canonical",
                source: "Canonical.sol",
                contract: "Case5Canonical",
                expectation: Expectation::Canonical,
            },
            Variant {
                name: "incompatible-member-order",
                source: "IncompatibleReordered.sol",
                contract: "Case5IncompatibleReordered",
                expectation: Expectation::Collision,
            },
            Variant {
                name: "incompatible-wide-member-order",
                source: "IncompatibleWideReordered.sol",
                contract: "Case5IncompatibleWideReordered",
                expectation: Expectation::Collision,
            },
            Variant {
                name: "incompatible-address-array",
                source: "IncompatibleAddress.sol",
                contract: "Case5IncompatibleAddress",
                expectation: Expectation::CompatibleEvidence,
            },
            Variant {
                name: "adjacent-arrays",
                source: "AdjacentArrays.sol",
                contract: "Case5AdjacentArrays",
                expectation: Expectation::Collision,
            },
        ],
    },
    Case {
        name: "6-array-mapping-struct",
        directory: "6-array-mapping-struct",
        minimum_canonical_validated: 3,
        maximum_canonical_uncertain: 0,
        variants: &[
            Variant {
                name: "canonical-indexed-mapping-array-struct",
                source: "Canonical.sol",
                contract: "Case6Canonical",
                expectation: Expectation::Canonical,
            },
            Variant {
                name: "incompatible-indexed-member-order",
                source: "Incompatible.sol",
                contract: "Case6Incompatible",
                expectation: Expectation::Collision,
            },
        ],
    },
    Case {
        name: "7-array-mapping-struct-push",
        directory: "7-array-mapping-struct-push",
        minimum_canonical_validated: 4,
        maximum_canonical_uncertain: 0,
        variants: &[
            Variant {
                name: "canonical-mapping-array-struct",
                source: "Canonical.sol",
                contract: "Case7Canonical",
                expectation: Expectation::Canonical,
            },
            Variant {
                name: "incompatible-mapping-array-struct",
                source: "IncompatibleOnlyArray.sol",
                contract: "Case7IncompatibleOnlyArray",
                expectation: Expectation::Collision,
            },
            Variant {
                name: "incompatible-mapping-array-and-fields",
                source: "IncompatibleArrayAndFields.sol",
                contract: "Case7IncompatibleArrayAndFields",
                expectation: Expectation::Collision,
            },
        ],
    },
    Case {
        name: "final-full-storage",
        directory: "final-full-storage",
        minimum_canonical_validated: 30,
        maximum_canonical_uncertain: 2,
        variants: &[
            Variant {
                name: "canonical-full-storage",
                source: "Canonical.sol",
                contract: "FullStorageFacet",
                expectation: Expectation::Canonical,
            },
            Variant {
                name: "compatible-prefix-storage",
                source: "Compatible.sol",
                contract: "CompatibleStorageFacet",
                expectation: Expectation::CompatibleEvidence,
            },
            Variant {
                name: "incompatible-full-storage",
                source: "Incompatible.sol",
                contract: "IncompatibleStorageFacet",
                expectation: Expectation::Collision,
            },
        ],
    },
    Case {
        name: "8-bytes-string",
        directory: "8-bytes-string",
        minimum_canonical_validated: 0,
        maximum_canonical_uncertain: usize::MAX,
        variants: &[
            Variant {
                name: "canonical-bytes-string",
                source: "Canonical.sol",
                contract: "Case8Canonical",
                expectation: Expectation::ByteStringUncertainty,
            },
            Variant {
                name: "swapped-bytes-string-types",
                source: "Incompatible.sol",
                contract: "Case8BytesStringVariant",
                expectation: Expectation::ByteStringUncertainty,
            },
            Variant {
                name: "incompatible-container-shape",
                source: "IncompatibleContainer.sol",
                contract: "Case8IncompatibleContainer",
                expectation: Expectation::Collision,
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

            match variant.expectation {
                Expectation::Canonical => {
                    assert!(
                        report.collisions.is_empty(),
                        "{} canonical bytecode contradicts its own VSL",
                        case.name
                    );
                    assert!(
                        report.validated_variables.len() >= case.minimum_canonical_validated,
                        "{} canonical bytecode recovered too few VSL variables",
                        case.name
                    );
                    assert!(
                        report.uncertain_scopes.len() <= case.maximum_canonical_uncertain,
                        "{} canonical bytecode has unexpected scoped uncertainty",
                        case.name
                    );
                }
                Expectation::Collision => assert!(
                    !report.collisions.is_empty(),
                    "{} / {} must produce a proven collision",
                    case.name,
                    variant.name
                ),
                Expectation::CollisionOrUncertain => assert!(
                    !report.collisions.is_empty() || !report.uncertain_scopes.is_empty(),
                    "{} / {} produced no contradiction or scoped uncertainty",
                    case.name,
                    variant.name
                ),
                Expectation::CompatibleEvidence => {
                    assert!(
                        report.collisions.is_empty(),
                        "{} / {} has no contradictory write path",
                        case.name,
                        variant.name
                    );
                    assert!(
                        !report.validated_variables.is_empty(),
                        "{} / {} must retain its compatible observed write",
                        case.name,
                        variant.name
                    );
                }
                Expectation::ByteStringUncertainty => {
                    assert!(
                        report.collisions.is_empty(),
                        "{} / {} must not treat bytes/string payloads as collisions",
                        case.name,
                        variant.name
                    );
                    assert!(
                        !report.uncertain_scopes.is_empty(),
                        "{} / {} must scope touched bytes/string fields as uncertain",
                        case.name,
                        variant.name
                    );
                    assert_eq!(
                        report.uncertain_scopes.len(),
                        4,
                        "{} / {} must report each touched bytes/string field once",
                        case.name,
                        variant.name
                    );
                }
            }
        }
    }
}
