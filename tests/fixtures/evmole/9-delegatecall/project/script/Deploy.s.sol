// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/* Compose
 * https://compose.diamonds
 */

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
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
import {DelegateCallFacet} from "../src/facets/DelegateCallFacet.sol";
import {CompatibleDiamondDelegateImplementation} from "../src/implementations/CompatibleDiamondDelegateImplementation.sol";
import {IncompatibleDiamondDelegateImplementation} from "../src/implementations/IncompatibleDiamondDelegateImplementation.sol";
import {StoredDiamondDelegateImplementation} from "../src/implementations/StoredDiamondDelegateImplementation.sol";
import {FinalDiamondDelegateImplementation} from "../src/implementations/FinalDiamondDelegateImplementation.sol";
import {NestedDelegateImplementation} from "../src/implementations/NestedDelegateImplementation.sol";
import {MissingSelectorDelegateImplementation} from "../src/implementations/MissingSelectorDelegateImplementation.sol";

contract DeployScript is Script {
    function setUp() public {}

    function run() public returns (Diamond diamond) {
        vm.startBroadcast();

        address compatibleImplementation = address(new CompatibleDiamondDelegateImplementation());
        address incompatibleImplementation = address(new IncompatibleDiamondDelegateImplementation());
        address storedImplementation = address(new StoredDiamondDelegateImplementation());
        address finalImplementation = address(new FinalDiamondDelegateImplementation());
        address nestedImplementation = address(new NestedDelegateImplementation(finalImplementation));
        address missingSelectorImplementation = address(new MissingSelectorDelegateImplementation());
        console.log("StoredDiamondDelegateImplementation:", storedImplementation);

        address[] memory facets = new address[](11);

        /* Base facet generation. */
        facets[0] = address(new ERC20DataFacet());
        console.log("ERC20DataFacet:", facets[0]);
        facets[1] = address(new ERC20ApproveFacet());
        console.log("ERC20ApproveFacet:", facets[1]);
        facets[2] = address(new ERC20TransferFacet());
        console.log("ERC20TransferFacet:", facets[2]);
        facets[3] = address(new ERC20BurnFacet());
        console.log("ERC20BurnFacet:", facets[3]);
        facets[4] = address(new ERC20MetadataFacet());
        console.log("ERC20MetadataFacet:", facets[4]);
        facets[5] = address(new ERC20PermitFacet());
        console.log("ERC20PermitFacet:", facets[5]);
        facets[6] = address(new ERC20BridgeableFacet());
        console.log("ERC20BridgeableFacet:", facets[6]);

        /* Library facet generation. */
        facets[7] = address(new DiamondInspectFacet());
        console.log("DiamondInspectFacet:", facets[7]);
        facets[8] = address(new DiamondUpgradeFacet());
        console.log("DiamondUpgradeFacet:", facets[8]);
        facets[9] = address(new ERC165Facet());
        console.log("ERC165Facet:", facets[9]);
        facets[10] = address(
            new DelegateCallFacet(
                compatibleImplementation,
                incompatibleImplementation,
                storedImplementation,
                nestedImplementation,
                missingSelectorImplementation
            )
        );
        console.log("DelegateCallFacet:", facets[10]);

        /* Define diamond proxy. */
        diamond = new Diamond(facets);
        console.log("Diamond:", address(diamond));

        vm.stopBroadcast();
    }
}
