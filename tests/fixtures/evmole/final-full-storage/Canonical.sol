// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

contract FullStorageFacet {
    enum Status { None, Active, Paused }
    type UserId is uint64;

    struct InlineChild {
        bytes4 tailFacetNodeId;
        uint32 facetCount;
    }

    struct InlineOuter {
        bytes4 headFacetNodeId;
        InlineChild child;
        uint32 selectorCount;
    }

    struct ContainerChild {
        uint256 amount;
        bool active;
        address owner;
    }

    struct Node {
        uint256 amount;
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
        mapping(uint256 => uint256[]) mapToDynamicArray;
        uint256[] dynamicValues;
        uint8[] packedDynamicValues;

        uint256[5] fixedFive;
        uint256[300] fixedThreeHundred;
        uint256[5][10] nestedFixed;

        mapping(address => ContainerChild) childByAddress;
        ContainerChild[] childList;
        ContainerChild[2] fixedChildren;
        mapping(uint256 => ContainerChild[]) nestedChildren;

        mapping(bytes => uint256) bytesKeyed;
        mapping(string => uint256) stringKeyed;

        function(uint256) internal returns (uint256) internalFn;
    }

    /**
     * @custom:storage-location erc8042:compose.fixture.terminal.v1
     */
    struct TerminalStorage {
        mapping(uint256 => Node) nodes;
    }

    /**
     * @custom:storage-location erc8042:compose.fixture.nested.v1
     */
    struct NestedStorage {
        mapping(uint256 => ContainerChild[]) nestedChildren;
    }

    /**
     * @custom:storage-location erc8042:compose.fixture.dynamic-keys.v1
     */
    struct DynamicKeyStorage {
        mapping(bytes => uint256) bytesKeyed;
        mapping(string => uint256) stringKeyed;
    }

    /**
     * @custom:storage-location erc8042:compose.fixture.dynamic-data.v1
     */
    struct DynamicDataStorage {
        bytes dynamicBytes;
        string text;
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

    function writePacked(
        bool flag,
        address owner,
        uint8 smallUint,
        uint16 mediumUint,
        int24 signedValue,
        bytes2 shortBytes,
        uint64 userId
    ) external {
        Storage storage s = _storage();
        s.flag = flag;
        s.owner = owner;
        s.smallUint = smallUint;
        s.mediumUint = mediumUint;
        s.signedValue = signedValue;
        s.shortBytes = shortBytes;
        s.userId = UserId.wrap(userId);
    }

    function writeInline(
        bytes4 headFacetNodeId,
        bytes4 tailFacetNodeId,
        uint32 facetCount,
        uint32 selectorCount
    ) external {
        InlineOuter storage value = _storage().inlineStruct;
        value.headFacetNodeId = headFacetNodeId;
        value.child.tailFacetNodeId = tailFacetNodeId;
        value.child.facetCount = facetCount;
        value.selectorCount = selectorCount;
    }

    function writeContainers(
        address account,
        uint256 key,
        uint256 amount,
        bool active,
        address owner,
        uint8 packedValue
    ) external {
        Storage storage s = _storage();
        s.balances[account] = amount;
        s.mapToDynamicArray[key].push(amount);
        s.dynamicValues.push(amount);
        s.packedDynamicValues.push(packedValue);

        ContainerChild storage mapped = s.childByAddress[account];
        mapped.amount = amount;
        mapped.active = active;
        mapped.owner = owner;
    }

    function writeFixed(
        uint256 outer,
        uint256 inner,
        uint256 value
    ) external {
        Storage storage s = _storage();
        s.fixedFive[inner % 5] = value;
        s.fixedThreeHundred[outer % 300] = value;
        s.nestedFixed[outer % 10][inner % 5] = value;
    }

    function writeTerminalAmount(uint256 key, uint256 amount) external {
        _terminal().nodes[key].amount = amount;
    }

    function writeNestedChildren(
        uint256 key,
        uint256 index,
        uint256 amount,
        bool active
    ) external {
        NestedStorage storage s = _nested();
        s.nestedChildren[key].push();
        ContainerChild storage child = s.nestedChildren[key][index];
        child.amount = amount;
        child.active = active;
    }

    function writeDynamicKeys(
        bytes calldata bytesKey,
        string calldata stringKey,
        uint256 value
    ) external {
        DynamicKeyStorage storage s = _dynamicKeys();
        s.bytesKeyed[bytesKey] = value;
        s.stringKeyed[stringKey] = value;
    }

    function writeDynamicData(bytes calldata data, string calldata value) external {
        DynamicDataStorage storage s = _dynamicData();
        s.dynamicBytes = data;
        s.text = value;
    }

    function exportSelectors() external pure returns (bytes4[] memory selectors) {
        selectors = new bytes4[](1);
        selectors[0] = this.writePacked.selector;
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
