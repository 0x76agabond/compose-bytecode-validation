// SPDX-License-Identifier: MIT
pragma solidity >=0.8.30;

contract ConstantKey {
    bytes32 constant STORAGE_POSITION = keccak256("evmole.normal");
    address constant SAMPLE_ACCOUNT = address(0x1111111111111111111111111111111111111111);
    address constant SAMPLE_SPENDER = address(0x2222222222222222222222222222222222222222);
    uint256 constant SAMPLE_INDEX = 1;
    uint8 constant SAMPLE_COUNT2 = 7;

    struct InnerRecord {
        bytes4 tailId;
        uint64 count;
        uint8 count2;
    }

    struct DirectRecord {
        bytes4 headId;
        InnerRecord inner;
        uint32 total;
    }

    struct NormalStorage {
        uint256 totalSupply;
        bytes4 marker;
        DirectRecord record;
        mapping(address account => uint256 value) balances;
        mapping(address account => mapping(address spender => uint256 value)) allowances;
        uint8[] smallValues;
    }

    struct NormalSnapshot {
        uint256 totalSupply;
        bytes4 marker;
        bytes4 headId;
        bytes4 tailId;
        uint64 count;
        uint8 count2;
        uint32 total;
        uint256 balance;
        uint256 allowance;
        uint8 smallValue;
    }

    function getStorage() internal pure returns (NormalStorage storage s) {
        bytes32 position = STORAGE_POSITION;
        assembly {
            s.slot := position
        }
    }

    function readAll() external view returns (NormalSnapshot memory snapshot) {
        NormalStorage storage s = getStorage();
        snapshot.totalSupply = s.totalSupply;
        snapshot.marker = s.marker;
        snapshot.headId = s.record.headId;
        snapshot.tailId = s.record.inner.tailId;
        snapshot.count = s.record.inner.count;
        snapshot.count2 = s.record.inner.count2;
        snapshot.total = s.record.total;
        snapshot.balance = s.balances[SAMPLE_ACCOUNT];
        snapshot.allowance = s.allowances[SAMPLE_ACCOUNT][SAMPLE_SPENDER];
        snapshot.smallValue = s.smallValues[SAMPLE_INDEX];
    }

    function setCount2() external {
        getStorage().record.inner.count2 = SAMPLE_COUNT2;
    }
}
