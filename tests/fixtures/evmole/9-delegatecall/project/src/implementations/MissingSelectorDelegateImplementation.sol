// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

/// @dev Has runtime code but intentionally omits DelegateCallFacet's selector.
contract MissingSelectorDelegateImplementation {
    function unrelated() external pure returns (uint256) {
        return 1;
    }
}
