// SPDX-License-Identifier: MIT
pragma solidity 0.8.30;

import {Test} from "forge-std/Test.sol";
import {Diamond} from "../src/Diamond.sol";
import {DelegateCallFacet} from "../src/facets/DelegateCallFacet.sol";
import {CompatibleDiamondDelegateImplementation} from "../src/implementations/CompatibleDiamondDelegateImplementation.sol";
import {IncompatibleDiamondDelegateImplementation} from "../src/implementations/IncompatibleDiamondDelegateImplementation.sol";
import {StoredDiamondDelegateImplementation} from "../src/implementations/StoredDiamondDelegateImplementation.sol";
import {FinalDiamondDelegateImplementation} from "../src/implementations/FinalDiamondDelegateImplementation.sol";
import {NestedDelegateImplementation} from "../src/implementations/NestedDelegateImplementation.sol";
import {MissingSelectorDelegateImplementation} from "../src/implementations/MissingSelectorDelegateImplementation.sol";

contract DelegateCallDiamondTest is Test {
    bytes32 private constant DIAMOND_STORAGE_POSITION = keccak256("erc8153.diamond");
    bytes4 private constant COMPATIBLE_KEY = 0xaabbccdd;
    bytes4 private constant INCOMPATIBLE_KEY = 0x11223344;
    bytes4 private constant STORED_TARGET_KEY = 0xdecafbad;

    Diamond private diamond;

    function setUp() public {
        address compatible = address(new CompatibleDiamondDelegateImplementation());
        address incompatible = address(new IncompatibleDiamondDelegateImplementation());
        address stored = address(new StoredDiamondDelegateImplementation());
        address finalImplementation = address(new FinalDiamondDelegateImplementation());
        address nested = address(new NestedDelegateImplementation(finalImplementation));
        address missingSelector = address(new MissingSelectorDelegateImplementation());
        address[] memory facets = new address[](1);
        facets[0] = address(
            new DelegateCallFacet(compatible, incompatible, stored, nested, missingSelector)
        );
        diamond = new Diamond(facets);
    }

    function test_delegatedCompatibleWriteUsesFacetNodeAddressField() public {
        address expectedFacet = address(0xBEEF);
        DelegateCallFacet(address(diamond)).delegateSetFacet(COMPATIBLE_KEY, expectedFacet);

        bytes32 slot = _facetNodeSlot(COMPATIBLE_KEY);
        assertEq(address(uint160(uint256(vm.load(address(diamond), slot)))), expectedFacet);
    }

    function test_delegatedIncompatibleWriteOverwritesWholeFacetNodeSlot() public {
        uint256 expectedValue = type(uint256).max;
        DelegateCallFacet(address(diamond)).delegateOverwriteFacet(INCOMPATIBLE_KEY, expectedValue);

        bytes32 slot = _facetNodeSlot(INCOMPATIBLE_KEY);
        assertEq(uint256(vm.load(address(diamond), slot)), expectedValue);
    }

    function test_storageResolvedTargetWritesFacetNodeAddressField() public {
        DelegateCallFacet(address(diamond)).configureStoredTarget(STORED_TARGET_KEY);

        address expectedFacet = address(0xCAFE);
        DelegateCallFacet(address(diamond)).delegateStoredSetFacet(COMPATIBLE_KEY, expectedFacet);

        bytes32 slot = _facetNodeSlot(COMPATIBLE_KEY);
        assertEq(address(uint160(uint256(vm.load(address(diamond), slot)))), expectedFacet);
    }

    function test_nestedDelegatecallWritesFacetNodeAddressField() public {
        address expectedFacet = address(0xD00D);
        DelegateCallFacet(address(diamond)).delegateNestedSetFacet(COMPATIBLE_KEY, expectedFacet);

        bytes32 slot = _facetNodeSlot(COMPATIBLE_KEY);
        assertEq(address(uint160(uint256(vm.load(address(diamond), slot)))), expectedFacet);
    }

    function _facetNodeSlot(bytes4 selector) private pure returns (bytes32) {
        return keccak256(abi.encode(selector, DIAMOND_STORAGE_POSITION));
    }
}
