// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

contract IncompatibleStorageFacet {
    enum Status { None, Active, Paused }
    type UserId is uint64;

    struct InlineChild {
        bytes4 tailFacetNodeId;
        uint64 facetCount;
    }

    struct InlineOuter {
        bytes4 headFacetNodeId;
        InlineChild child;
        uint32 selectorCount;
    }

    struct ContainerChild {
        address amount;
        bool active;
        address owner;
    }

    struct NestedContainerChild {
        address amount;
        bool active;
        address owner;
    }

    /**
     * @custom:storage-location erc8042:compose.fixture.virtual-storage
     */
    struct Storage {
        bool flag;
        Status status;
        address owner;
        uint8 smallUint;
        uint16 mediumUint;
        int24 signedValue;
        bytes2 shortBytes;
        UserId userId;

        bytes dynamicBytes;
        string text;

        function(uint256) external returns (uint256) externalFn;

        InlineOuter inlineStruct;

        mapping(address => uint256) balances;
        mapping(uint256 => address[]) mapToDynamicArray;
        address[] dynamicValues;
        uint16[] packedDynamicValues;

        address[5] fixedFive;
        address[300] fixedThreeHundred;
        address[5][10] nestedFixed;

        mapping(address => ContainerChild) childByAddress;
        ContainerChild[] childList;
        ContainerChild[2] fixedChildren;
        mapping(uint256 => ContainerChild[]) nestedChildren;

        mapping(bytes => uint256) bytesKeyed;
        mapping(string => uint256) stringKeyed;

        uint256 internalFn;
    }

    /**
     * @custom:storage-location erc8042:compose.fixture.terminal.v1
     */
    struct TerminalStorage {
        mapping(uint256 => address) nodes;
    }

    /**
     * @custom:storage-location erc8042:compose.fixture.nested.v1
     */
    struct NestedStorage {
        mapping(uint256 => NestedContainerChild[]) nestedChildren;
    }

    /**
     * @custom:storage-location erc8042:compose.fixture.dynamic-keys.v1
     */
    struct DynamicKeyStorage {
        mapping(bytes => address) bytesKeyed;
        mapping(string => address) stringKeyed;
    }

    /**
     * @custom:storage-location erc8042:compose.fixture.dynamic-data.v1
     */
    struct DynamicDataStorage {
        string dynamicBytes;
        bytes text;
    }

    bytes32 private constant STORAGE_POSITION =
        keccak256("compose.fixture.virtual-storage");
    bytes32 private constant TERMINAL_POSITION =
        keccak256("compose.fixture.terminal.v1");
    bytes32 private constant NESTED_POSITION =
        keccak256("compose.fixture.nested.v1");
    bytes32 private constant DYNAMIC_KEYS_POSITION =
        keccak256("compose.fixture.dynamic-keys.v1");
    bytes32 private constant DYNAMIC_DATA_POSITION =
        keccak256("compose.fixture.dynamic-data.v1");

    function writeIncompatibleContainers(
        address account,
        uint256 key,
        address value,
        uint16 packedValue
    ) external {
        Storage storage s = _storage();
        s.mapToDynamicArray[key].push(value);
        s.dynamicValues.push(value);
        s.packedDynamicValues.push(packedValue);
        s.childByAddress[account].amount = value;
    }

    function writeIncompatibleFixed(address value, uint256 index) external {
        Storage storage s = _storage();
        s.fixedFive[index % 5] = value;
        s.fixedThreeHundred[index % 300] = value;
        s.nestedFixed[index % 10][index % 5] = value;
        s.fixedChildren[index % 2].amount = value;
    }

    function writeIncompatibleTerminal(uint256 key, address value) external {
        _terminal().nodes[key] = value;
    }

    function writeIncompatibleNested(
        uint256 key,
        uint256 index,
        address amount,
        bool active
    ) external {
        NestedStorage storage s = _nested();
        s.nestedChildren[key].push();
        NestedContainerChild storage child = s.nestedChildren[key][index];
        child.amount = amount;
        child.active = active;
    }

    function writeIncompatibleDynamicKeys(
        bytes calldata bytesKey,
        string calldata stringKey,
        address value
    ) external {
        DynamicKeyStorage storage s = _dynamicKeys();
        s.bytesKeyed[bytesKey] = value;
        s.stringKeyed[stringKey] = value;
    }

    function writeIncompatibleDynamicData(
        string calldata data,
        bytes calldata value
    ) external {
        DynamicDataStorage storage s = _dynamicData();
        s.dynamicBytes = data;
        s.text = value;
    }

    function _storage() private pure returns (Storage storage s) {
        bytes32 position = STORAGE_POSITION;
        assembly {
            s.slot := position
        }
    }

    function _terminal() private pure returns (TerminalStorage storage s) {
        bytes32 position = TERMINAL_POSITION;
        assembly {
            s.slot := position
        }
    }

    function _nested() private pure returns (NestedStorage storage s) {
        bytes32 position = NESTED_POSITION;
        assembly {
            s.slot := position
        }
    }

    function _dynamicKeys() private pure returns (DynamicKeyStorage storage s) {
        bytes32 position = DYNAMIC_KEYS_POSITION;
        assembly {
            s.slot := position
        }
    }

    function _dynamicData() private pure returns (DynamicDataStorage storage s) {
        bytes32 position = DYNAMIC_DATA_POSITION;
        assembly {
            s.slot := position
        }
    }
}
