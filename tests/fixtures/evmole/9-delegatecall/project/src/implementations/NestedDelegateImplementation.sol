// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

contract NestedDelegateImplementation {
    address private immutable FINAL_IMPLEMENTATION;

    constructor(address finalImplementation_) {
        FINAL_IMPLEMENTATION = finalImplementation_;
    }

    function delegateNestedSetFacet(bytes4, address) external {
        address target = FINAL_IMPLEMENTATION;
        assembly ("memory-safe") {
            calldatacopy(0, 0, calldatasize())
            let success := delegatecall(gas(), target, 0, calldatasize(), 0, 0)
            returndatacopy(0, 0, returndatasize())
            switch success
            case 0 { revert(0, returndatasize()) }
            default { return(0, returndatasize()) }
        }
    }
}
