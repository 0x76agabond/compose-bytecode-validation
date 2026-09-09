// SPDX-License-Identifier: MIT
pragma solidity >=0.8.30;

/* Compose
 * https://compose.diamonds
 */

/*
 * TEST FIXTURE: This facet intentionally assigns an incompatible layout to
 * the storage identifier already used by the ERC-20 facets.
 */

contract ERC20IdentifierCollisionError {
    bytes32 constant STORAGE_POSITION = keccak256("erc20");

    struct Data {
        uint256 value;
        mapping(address => uint256) values;
    }

    function getStorage() internal pure returns (Data storage s) {
        bytes32 position = STORAGE_POSITION;
        assembly {
            s.slot := position
        }
    }

    function getValue() external view returns (uint256) {
        return getStorage().value;
    }

    function getAccountValue(address _account) external view returns (uint256) {
        return getStorage().values[_account];
    }

    function exportSelectors() external pure returns (bytes memory) {
        return bytes.concat(
            this.getValue.selector,
            this.getAccountValue.selector
        );
    }
}
