//! Compose-specific calldata behavior for bytecode validation.
//!
//! EVMole intentionally models `CALLDATASIZE` as a large symbolic value.
//! Compose's delegatecall validator needs only a bounded copied prefix to
//! recover forwarded selectors, while preserving that symbolic stack value.

use crate::evm::{
    U256,
    calldata::{CallData, CallDataImpl, CallDataLabel},
    element::Element,
};
use std::error;

const MAX_COPIED_CALLDATA_BYTES: usize = 32768;

pub(crate) struct ComposeCallData<T> {
    inner: CallDataImpl<T>,
    copy_limit: Option<usize>,
}

impl<T> ComposeCallData<T> {
    pub(crate) fn passthrough(selector: [u8; 4], arguments: &[crate::DynSolType]) -> Self {
        Self {
            inner: CallDataImpl::new(selector, arguments),
            copy_limit: None,
        }
    }

    pub(crate) fn bounded_copy(selector: [u8; 4], arguments: &[crate::DynSolType]) -> Self {
        Self {
            inner: CallDataImpl::new(selector, arguments),
            copy_limit: Some(MAX_COPIED_CALLDATA_BYTES),
        }
    }
}

impl<T: CallDataLabel> CallData<T> for ComposeCallData<T> {
    fn load32(&self, offset: U256) -> Element<T> {
        self.inner.load32(offset)
    }

    fn load(
        &self,
        offset: U256,
        size: U256,
    ) -> Result<(Vec<u8>, Option<T>), Box<dyn error::Error>> {
        let size = self.copy_limit.map_or(size, |limit| {
            let requested = usize::try_from(size).unwrap_or(limit);
            U256::from(requested.min(limit))
        });
        self.inner.load(offset, size)
    }

    fn len(&self) -> U256 {
        self.inner.len()
    }

    fn selector(&self) -> [u8; 4] {
        self.inner.selector()
    }
}

#[cfg(test)]
mod tests {
    use super::ComposeCallData;
    use crate::{
        DynSolType,
        evm::{
            U256,
            calldata::{CallData, CallDataLabel, CallDataLabelType},
        },
    };

    #[derive(Clone)]
    struct TestLabel;

    impl CallDataLabel for TestLabel {
        fn label(_: usize, _: &DynSolType, _: CallDataLabelType) -> Option<Self> {
            None
        }
    }

    #[test]
    fn bounds_only_compose_calldata_copies() {
        let calldata = ComposeCallData::<TestLabel>::bounded_copy([0x12, 0x34, 0x56, 0x78], &[]);
        let (data, _) = calldata.load(U256::ZERO, U256::from(131072)).unwrap();

        assert_eq!(data.len(), 32768);
        assert_eq!(&data[..4], &[0x12, 0x34, 0x56, 0x78]);

        let passthrough =
            ComposeCallData::<TestLabel>::passthrough([0; 4], &[DynSolType::Uint(256)]);
        assert!(passthrough.load(U256::ZERO, U256::from(131072)).is_err());
    }
}
