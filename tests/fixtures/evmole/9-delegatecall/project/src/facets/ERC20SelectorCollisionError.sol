// SPDX-License-Identifier: MIT
pragma solidity >=0.8.30;

/* Compose
 * https://compose.diamonds
 */

/*
 * TEST FIXTURE: This facet intentionally exports balanceOf(address), which is
 * already exported by ERC20DataFacet.
 */

contract ERC20SelectorCollisionError {
    bytes32 constant STORAGE_POSITION = keccak256("erc20.collision");

    struct Data {
        mapping(address => uint256) balances;
    }

    function getStorage() internal pure returns (Data storage s) {
        bytes32 position = STORAGE_POSITION;
        assembly {
            s.slot := position
        }
    }

    function balanceOf(address _account) external view returns (uint256) {
        return getStorage().balances[_account];
    }

    function exportSelectors() external pure returns (bytes memory) {
        return bytes.concat(this.balanceOf.selector);
    }
}
