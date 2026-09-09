// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/* Compose
 * https://compose.diamonds
 */

import {Test} from "forge-std/Test.sol";
import {Diamond} from "../src/Diamond.sol";
import {ERC20DataFacet} from "@perfect-abstractions/compose/token/ERC20/Data/ERC20DataFacet.sol";
import {ERC20ApproveFacet} from "@perfect-abstractions/compose/token/ERC20/Approve/ERC20ApproveFacet.sol";
import {ERC20TransferFacet} from "@perfect-abstractions/compose/token/ERC20/Transfer/ERC20TransferFacet.sol";
import {ERC20BurnFacet} from "@perfect-abstractions/compose/token/ERC20/Burn/ERC20BurnFacet.sol";
import {ERC20MetadataFacet} from "@perfect-abstractions/compose/token/ERC20/Metadata/ERC20MetadataFacet.sol";
import {ERC20PermitFacet} from "@perfect-abstractions/compose/token/ERC20/Permit/ERC20PermitFacet.sol";
import {ERC20BridgeableFacet} from "@perfect-abstractions/compose/token/ERC20/Bridgeable/ERC20BridgeableFacet.sol";
import {DiamondInspectFacet} from "@perfect-abstractions/compose/diamond/DiamondInspectFacet.sol";
import {DiamondUpgradeFacet} from "@perfect-abstractions/compose/diamond/DiamondUpgradeFacet.sol";
import {ERC165Facet} from "@perfect-abstractions/compose/interfaceDetection/ERC165/ERC165Facet.sol";

contract DiamondTest is Test {
    Diamond diamond;

    function setUp() public {
        address[] memory facets = new address[](10);

        /* Base facet generation. */
        facets[0] = address(new ERC20DataFacet());
        facets[1] = address(new ERC20ApproveFacet());
        facets[2] = address(new ERC20TransferFacet());
        facets[3] = address(new ERC20BurnFacet());
        facets[4] = address(new ERC20MetadataFacet());
        facets[5] = address(new ERC20PermitFacet());
        facets[6] = address(new ERC20BridgeableFacet());

        /* Library facet generation. */
        facets[7] = address(new DiamondInspectFacet());
        facets[8] = address(new DiamondUpgradeFacet());
        facets[9] = address(new ERC165Facet());

        diamond = new Diamond(facets);
    }

    function test_inspect_facetAddresses() public view {
        DiamondInspectFacet inspect = DiamondInspectFacet(address(diamond));
        address[] memory addresses = inspect.facetAddresses();
        assertEq(addresses.length, 10);
    }
}
