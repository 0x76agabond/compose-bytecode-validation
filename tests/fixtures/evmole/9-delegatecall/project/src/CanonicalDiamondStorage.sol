// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

/// @dev VSL source of truth for the Compose router namespace.
contract CanonicalDiamondStorage {
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
}
