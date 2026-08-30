// SPDX-License-Identifier: MIT
pragma solidity >=0.8.30;

contract WrongPackedWrite {
    bytes32 constant STORAGE_POSITION = keccak256("evmole.normal");

    function overwritePackedSlot() external {
        bytes32 slot = bytes32(uint256(STORAGE_POSITION) + 3);
        assembly {
            sstore(slot, 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff)
        }
    }
}
