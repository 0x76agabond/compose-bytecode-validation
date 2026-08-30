// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

contract Case1Incompatible {
    bytes32 constant STORAGE_POSITION = keccak256("compose.validation.case1.normal");

    struct InnerRecord {
        bytes4 tailId;
        uint64 count;
        uint256 count2;
    }

    struct DirectRecord {
        bytes4 headId;
        InnerRecord inner;
        uint32 total;
    }

    struct Storage {
        uint256 totalSupply;
        bytes4 marker;
        DirectRecord record;
        mapping(address => uint256) balances;
        mapping(address => mapping(address => uint256)) allowances;
        uint8[] smallValues;
    }

    function writeRecord(bytes4 headId, bytes4 tailId, uint64 count, uint256 count2, uint32 total) external {
        Storage storage s = _storage();
        s.record.headId = headId;
        s.record.inner.tailId = tailId;
        s.record.inner.count = count;
        s.record.inner.count2 = count2;
        s.record.total = total;
    }

    function _storage() private pure returns (Storage storage s) {
        bytes32 position = STORAGE_POSITION;
        assembly {
            s.slot := position
        }
    }
}
