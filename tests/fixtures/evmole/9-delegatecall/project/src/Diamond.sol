// SPDX-License-Identifier: MIT
pragma solidity >=0.8.30;

/* Compose
 * https://compose.diamonds
 */

import "@perfect-abstractions/compose/diamond/DiamondMod.sol" as DiamondMod;
import "@perfect-abstractions/compose/token/ERC20/Metadata/ERC20MetadataMod.sol" as ERC20MetadataMod;
import "@perfect-abstractions/compose/interfaceDetection/ERC165/ERC165Mod.sol" as ERC165Mod;
import {IERC20} from "@perfect-abstractions/compose/interfaces/IERC20.sol";

contract Diamond {
    constructor(address[] memory _facets) {
        DiamondMod.addFacets(_facets);

        // TODO: Review the sample token name, symbol, and decimals before deployment.
        ERC20MetadataMod.setMetadata({_name: "ExampleToken", _symbol: "EXT", _decimals: 18});

        // Register ERC-20 interface support.
        ERC165Mod.registerInterface(type(IERC20).interfaceId);
    }

    fallback() external payable {
        DiamondMod.diamondFallback();
    }

    receive() external payable {}
}
