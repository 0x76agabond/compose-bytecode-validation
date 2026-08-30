use super::types::{
    ComposeValidationStatus, ObservationVerdict, RawStorageObservation, StorageObservation,
    VirtualStorageLayout, VirtualStorageLayoutRecord, VslSlotMatch,
};
use super::vsl_semantics::{SemanticCompatibility, compare_semantic_types, expected_type_at};

pub(crate) fn hex(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(2 + bytes.len() * 2);
    output.push_str("0x");
    for byte in bytes {
        output.push_str(&format!("{byte:02x}"));
    }
    output
}

pub(crate) fn inferred_width(inferred_type: &str) -> Option<u16> {
    if inferred_type.starts_with("mapping(")
        || inferred_type.ends_with(']')
        || matches!(inferred_type, "bytes" | "string" | "uint256")
    {
        return Some(256);
    }
    if inferred_type == "bool" {
        return Some(8);
    }
    if inferred_type == "address" {
        return Some(160);
    }
    if inferred_type.starts_with("function") {
        return Some(192);
    }
    if let Some(bits) = numeric_suffix(inferred_type, "uint") {
        return Some(bits);
    }
    if let Some(bits) = numeric_suffix(inferred_type, "int") {
        return Some(bits);
    }
    if let Some(size) = numeric_suffix(inferred_type, "bytes") {
        return size.checked_mul(8);
    }
    None
}

pub(crate) fn vsl_match(
    raw: &RawStorageObservation,
    layout: &VirtualStorageLayout,
) -> Option<VslSlotMatch> {
    layout
        .records
        .iter()
        .filter(|record| is_physical_root(record, &layout.records))
        .find_map(|record| {
            let slot = raw.slot.as_ref()?;
            let root = decode_slot(&record.id)?;
            let slot_index = slot_delta(&root, slot, record.slots.len())?;
            let group = record.slots.get(slot_index)?;
            let mut offset = 0_u16;
            let expected_width = group.iter().copied().find(|width| {
                let current_offset = offset / 8;
                offset = offset.saturating_add(*width);
                current_offset == u16::from(raw.offset)
            });

            Some(VslSlotMatch {
                record_id: record.id.clone(),
                virtual_path: record.virtual_path.clone(),
                slot_index,
                expected_offset: expected_width.map(|_| raw.offset),
                expected_width,
                expected_type: expected_type_at(record, &layout.records, slot_index, raw.offset),
            })
        })
}

pub(crate) fn base_observation(
    raw: &RawStorageObservation,
    layout: &VirtualStorageLayout,
) -> StorageObservation {
    let inferred_width = inferred_width(&raw.inferred_type);
    let matched = vsl_match(raw, layout);
    let verdict = match (&matched, inferred_width) {
        (None, _) => ObservationVerdict::Unmapped,
        (Some(_), None) => ObservationVerdict::Uncertain,
        (Some(matched), Some(width)) => match matched.expected_width {
            Some(expected) if expected == width => matched.expected_type.as_ref().map_or(
                ObservationVerdict::Uncertain,
                |expected_type| match compare_semantic_types(&raw.inferred_type, expected_type) {
                    SemanticCompatibility::Compatible | SemanticCompatibility::KeyMismatch => {
                        ObservationVerdict::NoPhysicalContradiction
                    }
                    SemanticCompatibility::Contradiction => ObservationVerdict::Contradiction,
                    SemanticCompatibility::Uncertain => ObservationVerdict::Uncertain,
                },
            ),
            Some(_) | None => ObservationVerdict::Contradiction,
        },
    };

    StorageObservation {
        slot: raw
            .slot
            .as_ref()
            .map_or_else(|| "unresolved".to_owned(), |slot| hex(slot)),
        symbolic_path: raw.symbolic_path.clone(),
        offset: raw.offset,
        inferred_type: raw.inferred_type.clone(),
        candidate_types: raw.candidate_types.clone(),
        inferred_width,
        mask: raw.mask.clone(),
        is_write: raw.is_write,
        reads: raw.reads.iter().map(|selector| hex(selector)).collect(),
        writes: raw.writes.iter().map(|selector| hex(selector)).collect(),
        vsl_match: matched,
        inference_source: super::types::InferenceSource::Bytecode,
        verdict,
    }
}

pub(crate) fn report_status(observations: &[StorageObservation]) -> ComposeValidationStatus {
    if observations
        .iter()
        .any(|observation| observation.verdict == ObservationVerdict::Contradiction)
    {
        return ComposeValidationStatus::Contradiction;
    }
    if observations.is_empty()
        || observations.iter().any(|observation| {
            matches!(
                observation.verdict,
                ObservationVerdict::Uncertain | ObservationVerdict::Unmapped
            )
        })
    {
        return ComposeValidationStatus::Uncertain;
    }
    ComposeValidationStatus::NoContradiction
}

fn numeric_suffix(value: &str, prefix: &str) -> Option<u16> {
    let suffix = value.strip_prefix(prefix)?;
    (!suffix.is_empty())
        .then(|| suffix.parse::<u16>().ok())
        .flatten()
}

fn is_physical_root(
    candidate: &VirtualStorageLayoutRecord,
    records: &[VirtualStorageLayoutRecord],
) -> bool {
    let _ = records;
    candidate.parent_virtual_path.is_none()
}

fn decode_slot(value: &str) -> Option<[u8; 32]> {
    let value = value.strip_prefix("0x").unwrap_or(value);
    if value.is_empty() || value.len() > 64 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return None;
    }
    let padded = format!("{value:0>64}");
    let mut slot = [0_u8; 32];
    for (index, byte) in slot.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&padded[index * 2..index * 2 + 2], 16).ok()?;
    }
    Some(slot)
}

fn slot_delta(root: &[u8; 32], slot: &[u8; 32], span: usize) -> Option<usize> {
    let mut current = *root;
    for delta in 0..span {
        if &current == slot {
            return Some(delta);
        }
        increment_slot(&mut current)?;
    }
    None
}

fn increment_slot(slot: &mut [u8; 32]) -> Option<()> {
    for byte in slot.iter_mut().rev() {
        let (next, overflow) = byte.overflowing_add(1);
        *byte = next;
        if !overflow {
            return Some(());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{decode_slot, inferred_width, is_physical_root, slot_delta};
    use crate::compose::{
        VirtualStorageLayoutKind, VirtualStorageLayoutRecord, VirtualStorageLayoutSource,
    };

    fn record(path: &str, parent_virtual_path: Option<&str>) -> VirtualStorageLayoutRecord {
        VirtualStorageLayoutRecord {
            id: "0x01".to_owned(),
            virtual_path: path.to_owned(),
            parent_virtual_path: parent_virtual_path.map(str::to_owned),
            kind: VirtualStorageLayoutKind::Normal,
            code_width: 1,
            layout: Vec::new(),
            serialized_layout: Vec::new(),
            slots: Vec::new(),
            source: VirtualStorageLayoutSource::SlotAssignment,
            source_name: "Fixture.sol".to_owned(),
            contract_name: "Fixture".to_owned(),
            struct_name: None,
            diamond_name: None,
        }
    }

    #[test]
    fn parses_common_inferred_widths() {
        assert_eq!(inferred_width("bool"), Some(8));
        assert_eq!(inferred_width("address"), Some(160));
        assert_eq!(inferred_width("uint64"), Some(64));
        assert_eq!(inferred_width("mapping(address => uint256)"), Some(256));
    }

    #[test]
    fn finds_small_slot_deltas() {
        let root = decode_slot("0xff").unwrap();
        let slot = decode_slot("0x101").unwrap();
        assert_eq!(slot_delta(&root, &slot, 4), Some(2));
    }

    #[test]
    fn keeps_numeric_suffix_roots_physical() {
        let root = record("fixture.root", None);
        let numeric_root = record("fixture.root.2", None);
        let child = record("fixture.root.0", Some("fixture.root"));
        let records = vec![root.clone(), numeric_root.clone(), child.clone()];

        assert!(is_physical_root(&root, &records));
        assert!(is_physical_root(&numeric_root, &records));
        assert!(!is_physical_root(&child, &records));
    }
}
