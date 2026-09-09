// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

contract CompatibleDiamondDelegateImplementation {
    struct FacetNode {
        address facet;
        bytes4 prevFacetNodeId;
        bytes4 nextFacetNodeId;
    }

    /** @custom:storage-location erc8042:erc8153.diamond */
    struct DiamondStorage {
        mapping(bytes4 functionSelector => FacetNode) facetNodes;
        bytes4 headFacetNodeId;
        bytes4 tailFacetNodeId;
        uint32 facetCount;
        uint32 selectorCount;
    }

    bytes32 private constant STORAGE_POSITION = keccak256("erc8153.diamond");

    function delegateSetFacet(bytes4 selector, address facet) external {
        _storage().facetNodes[selector].facet = facet;
    }

    function _storage() private pure returns (DiamondStorage storage s) {
        bytes32 position = STORAGE_POSITION;
        assembly ("memory-safe") {
            s.slot := position
        }
    }
}
