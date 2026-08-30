// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

contract Case2Canonical {
    bytes32 constant STORAGE_POSITION = keccak256("compose.validation.case2.constant-key");
    address constant ACCOUNT = address(0x1111111111111111111111111111111111111111);
    address constant SPENDER = address(0x2222222222222222222222222222222222222222);

    struct Storage {
        mapping(address => uint256) balances;
        mapping(address => mapping(address => uint256)) allowances;
        uint8[] smallValues;
    }

    function writeAll(uint256 balance, uint256 allowance, uint8 value) external {
        Storage storage s = _storage();
        s.balances[ACCOUNT] = balance;
        s.allowances[ACCOUNT][SPENDER] = allowance;
        s.smallValues.push(value);
    }

    function _storage() private pure returns (Storage storage s) {
        bytes32 position = STORAGE_POSITION;
        assembly {
            s.slot := position
        }
    }
}
