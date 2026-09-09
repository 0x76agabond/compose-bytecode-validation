// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

contract IncompatibleDiamondDelegateImplementation {
    /** @custom:storage-location erc8042:erc8153.diamond */
    struct WrongDiamondStorage {
        mapping(bytes4 functionSelector => uint256) facetNodes;
    }

    bytes32 private constant STORAGE_POSITION = keccak256("erc8153.diamond");

    function delegateOverwriteFacet(bytes4 selector, uint256 value) external {
        _storage().facetNodes[selector] = value;
    }

    function _storage() private pure returns (WrongDiamondStorage storage s) {
        bytes32 position = STORAGE_POSITION;
        assembly ("memory-safe") {
            s.slot := position
        }
    }
}
