// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

contract Case8Canonical {
    /**
     * @custom:storage-location erc8042:compose.validation.case8.bytes-string
     */
    struct Storage {
        bytes data;
        string text;
        bytes[] byteArray;
        string[] stringArray;
    }

    bytes32 private constant STORAGE_POSITION =
        keccak256("compose.validation.case8.bytes-string");

    function writeValues(bytes calldata data, string calldata text) external {
        Storage storage s = _storage();
        s.data = data;
        s.text = text;
    }

    function appendValues(bytes calldata data, string calldata text) external {
        Storage storage s = _storage();
        s.byteArray.push(data);
        s.stringArray.push(text);
    }

    function _storage() private pure returns (Storage storage s) {
        bytes32 position = STORAGE_POSITION;
        assembly {
            s.slot := position
        }
    }
}
