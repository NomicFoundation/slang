// SPDX-License-Identifier: GPL-3.0-or-later
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

pragma solidity ^0.7.0;

import "../../lib/math/FixedPoint.sol";
import "../../lib/math/Math.sol";
import "../../lib/helpers/InputHelpers.sol";

/* solhint-disable private-vars-leading-underscore */

contract WeightedMath {
    using FixedPoint for uint256;
    // A minimum normalized weight imposes a maximum weight ratio. We need this due to limitations in the
    // implementation of the power function, as these ratios are often exponents.
    uint256 internal constant _MIN_WEIGHT = 0.01e18;
    // Having a minimum normalized weight imposes a limit on the maximum number of tokens;
    // i.e., the largest possible pool is one where all tokens have exactly the minimum weight.
    uint256 internal constant _MAX_WEIGHTED_TOKENS = 100;

    // Pool limits that arise from limitations in the fixed point power function (and the imposed 1:100 maximum weight
    // ratio).

    // Swap limits: amounts swapped may not be larger than this percentage of total balance.
    uint256 internal constant _MAX_IN_RATIO = 0.3e18;
    uint256 internal constant _MAX_OUT_RATIO = 0.3e18;

    // Invariant growth limit: non-proportional joins cannot cause the invariant to increase by more than this ratio.
    uint256 internal constant _MAX_INVARIANT_RATIO = 3e18;
    // Invariant shrink limit: non-proportional exits cannot cause the invariant to decrease by less than this ratio.
    uint256 internal constant _MIN_INVARIANT_RATIO = 0.7e18;

    // Invariant is used to collect protocol swap fees by comparing its value between two times.
    // So we can round always to the same direction. It is also used to initiate the BPT amount
    // and, because there is a minimum BPT, we round down the invariant.
    function _calculateInvariant(uint256[] memory normalizedWeights, uint256[] memory balances)
        internal
        pure
        returns (uint256 invariant)
    {
        /**********************************************************************************************
        // invariant               _____                                                             //
        // wi = weight index i      | |      wi                                                      //
        // bi = balance index i     | |  bi ^   = i                                                  //
        // i = invariant                                                                             //
        **********************************************************************************************/

        invariant = FixedPoint.ONE;
        for (uint256 i = 0; i < normalizedWeights.length; i++) {
            invariant = invariant.mulDown(balances[i].powDown(normalizedWeights[i]));
        }

        _require(invariant > 0, Errors.ZERO_INVARIANT);
    }

    // Computes how many tokens can be taken out of a pool if `amountIn` are sent, given the
    // current balances and weights.
    function _calcOutGivenIn(
        uint256 balanceIn,
        uint256 weightIn,
        uint256 balanceOut,
        uint256 weightOut,
        uint256 amountIn
    ) internal pure returns (uint256) {
        /**********************************************************************************************
        // outGivenIn                                                                                //
        // aO = amountOut                                                                            //
        // bO = balanceOut                                                                           //
        // bI = balanceIn              /      /            bI             \    (wI / wO) \           //
        // aI = amountIn    aO = bO * |  1 - | --------------------------  | ^            |          //
        // wI = weightIn               \      \       ( bI + aI )         /              /           //
        // wO = weightOut                                                                            //
        **********************************************************************************************/

        // Amount out, so we round down overall.

        // The multiplication rounds down, and the subtrahend (power) rounds up (so the base rounds up too).
        // Because bI / (bI + aI) <= 1, the exponent rounds down.

        // Cannot exceed maximum in ratio
        _require(amountIn <= balanceIn.mulDown(_MAX_IN_RATIO), Errors.MAX_IN_RATIO);

        uint256 denominator = balanceIn.add(amountIn);
        uint256 base = balanceIn.divUp(denominator);
        uint256 exponent = weightIn.divDown(weightOut);
        uint256 power = base.powUp(exponent);

        return balanceOut.mulDown(power.complement());
    }

    // Computes how many tokens must be sent to a pool in order to take `amountOut`, given the
    // current balances and weights.
    function _calcInGivenOut(
        uint256 balanceIn,
        uint256 weightIn,
        uint256 balanceOut,
        uint256 weightOut,
        uint256 amountOut
    ) internal pure returns (uint256) {
        /**********************************************************************************************
        // inGivenOut                                                                                //
        // aO = amountOut                                                                            //
        // bO = balanceOut                                                                           //
        // bI = balanceIn              /  /            bO             \    (wO / wI)      \          //
        // aI = amountIn    aI = bI * |  | --------------------------  | ^            - 1  |         //
        // wI = weightIn               \  \       ( bO - aO )         /                   /          //
        // wO = weightOut                                                                            //
        **********************************************************************************************/

        // Amount in, so we round up overall.

        // The multiplication rounds up, and the power rounds up (so the base rounds up too).
        // Because b0 / (b0 - a0) >= 1, the exponent rounds up.

        // Cannot exceed maximum out ratio
        _require(amountOut <= balanceOut.mulDown(_MAX_OUT_RATIO), Errors.MAX_OUT_RATIO);

        uint256 base = balanceOut.divUp(balanceOut.sub(amountOut));
        uint256 exponent = weightOut.divUp(weightIn);
        uint256 power = base.powUp(exponent);

        // Because the base is larger than one (and the power rounds up), the power should always be larger than one, so
        // the following subtraction should never revert.
        uint256 ratio = power.sub(FixedPoint.ONE);

        return balanceIn.mulUp(ratio);
    }

    function _calcBptOutGivenExactTokensIn(
        uint256[] memory balances,
        uint256[] memory normalizedWeights,
        uint256[] memory amountsIn,
        uint256 bptTotalSupply,
        uint256 swapFee
    ) internal pure returns (uint256) {
        // BPT out, so we round down overall.

        uint256[] memory balanceRatiosWithFee = new uint256[](amountsIn.length);

        uint256 invariantRatioWithFees = 0;
        for (uint256 i = 0; i < balances.length; i++) {
            balanceRatiosWithFee[i] = balances[i].add(amountsIn[i]).divDown(balances[i]);
            invariantRatioWithFees = invariantRatioWithFees.add(balanceRatiosWithFee[i].mulDown(normalizedWeights[i]));
        }

        uint256 invariantRatio = FixedPoint.ONE;
        for (uint256 i = 0; i < balances.length; i++) {
            uint256 amountInWithoutFee;

            if (balanceRatiosWithFee[i] > invariantRatioWithFees) {
                uint256 nonTaxableAmount = balances[i].mulDown(invariantRatioWithFees.sub(FixedPoint.ONE));
                uint256 taxableAmount = amountsIn[i].sub(nonTaxableAmount);
                amountInWithoutFee = nonTaxableAmount.add(taxableAmount.mulDown(FixedPoint.ONE.sub(swapFee)));
            } else {
                amountInWithoutFee = amountsIn[i];
            }

            uint256 balanceRatio = balances[i].add(amountInWithoutFee).divDown(balances[i]);

            invariantRatio = invariantRatio.mulDown(balanceRatio.powDown(normalizedWeights[i]));
        }

        if (invariantRatio >= FixedPoint.ONE) {
            return bptTotalSupply.mulDown(invariantRatio.sub(FixedPoint.ONE));
        } else {
            return 0;
        }
    }

    function _calcTokenInGivenExactBptOut(
        uint256 balance,
        uint256 normalizedWeight,
        uint256 bptAmountOut,
        uint256 bptTotalSupply,
        uint256 swapFee
    ) internal pure returns (uint256) {
        /******************************************************************************************
        // tokenInForExactBPTOut                                                                 //
        // a = amountIn                                                                          //
        // b = balance                      /  /    totalBPT + bptOut      \    (1 / w)       \  //
        // bptOut = bptAmountOut   a = b * |  | --------------------------  | ^          - 1  |  //
        // bpt = totalBPT                   \  \       totalBPT            /                  /  //
        // w = weight                                                                            //
        ******************************************************************************************/

        // Token in, so we round up overall.

        // Calculate the factor by which the invariant will increase after minting BPTAmountOut
        uint256 invariantRatio = bptTotalSupply.add(bptAmountOut).divUp(bptTotalSupply);
        _require(invariantRatio <= _MAX_INVARIANT_RATIO, Errors.MAX_OUT_BPT_FOR_TOKEN_IN);

        // Calculate by how much the token balance has to increase to match the invariantRatio
        uint256 balanceRatio = invariantRatio.powUp(FixedPoint.ONE.divUp(normalizedWeight));

        uint256 amountInWithoutFee = balance.mulUp(balanceRatio.sub(FixedPoint.ONE));

        // We can now compute how much extra balance is being deposited and used in virtual swaps, and charge swap fees
        // accordingly.
        uint256 taxablePercentage = normalizedWeight.complement();
        uint256 taxableAmount = amountInWithoutFee.mulUp(taxablePercentage);
        uint256 nonTaxableAmount = amountInWithoutFee.sub(taxableAmount);

        return nonTaxableAmount.add(taxableAmount.divUp(swapFee.complement()));
    }

    function _calcBptInGivenExactTokensOut(
        uint256[] memory balances,
        uint256[] memory normalizedWeights,
        uint256[] memory amountsOut,
        uint256 bptTotalSupply,
        uint256 swapFee
    ) internal pure returns (uint256) {
        // BPT in, so we round up overall.

        uint256[] memory balanceRatiosWithoutFee = new uint256[](amountsOut.length);
        uint256 invariantRatioWithoutFees = 0;
        for (uint256 i = 0; i < balances.length; i++) {
            balanceRatiosWithoutFee[i] = balances[i].sub(amountsOut[i]).divUp(balances[i]);
            invariantRatioWithoutFees = invariantRatioWithoutFees.add(
                balanceRatiosWithoutFee[i].mulUp(normalizedWeights[i])
            );
        }

        uint256 invariantRatio = FixedPoint.ONE;
        for (uint256 i = 0; i < balances.length; i++) {
            // Swap fees are typically charged on 'token in', but there is no 'token in' here,
            // o we apply it to 'token out'.
            // This results in slightly larger price impact.

            uint256 amountOutWithFee;
            if (invariantRatioWithoutFees > balanceRatiosWithoutFee[i]) {
                uint256 nonTaxableAmount = balances[i].mulDown(invariantRatioWithoutFees.complement());
                uint256 taxableAmount = amountsOut[i].sub(nonTaxableAmount);

                amountOutWithFee = nonTaxableAmount.add(taxableAmount.divUp(swapFee.complement()));
            } else {
                amountOutWithFee = amountsOut[i];
            }

            uint256 balanceRatio = balances[i].sub(amountOutWithFee).divDown(balances[i]);

            invariantRatio = invariantRatio.mulDown(balanceRatio.powDown(normalizedWeights[i]));
        }

        return bptTotalSupply.mulUp(invariantRatio.complement());
    }

    function _calcTokenOutGivenExactBptIn(
        uint256 balance,
        uint256 normalizedWeight,
        uint256 bptAmountIn,
        uint256 bptTotalSupply,
        uint256 swapFee
    ) internal pure returns (uint256) {
        /*****************************************************************************************
        // exactBPTInForTokenOut                                                                //
        // a = amountOut                                                                        //
        // b = balance                     /      /    totalBPT - bptIn       \    (1 / w)  \   //
        // bptIn = bptAmountIn    a = b * |  1 - | --------------------------  | ^           |  //
        // bpt = totalBPT                  \      \       totalBPT            /             /   //
        // w = weight                                                                           //
        *****************************************************************************************/

        // Token out, so we round down overall. The multiplication rounds down, but the power rounds up (so the base
        // rounds up). Because (totalBPT - bptIn) / totalBPT <= 1, the exponent rounds down.

        // Calculate the factor by which the invariant will decrease after burning BPTAmountIn
        uint256 invariantRatio = bptTotalSupply.sub(bptAmountIn).divUp(bptTotalSupply);
        _require(invariantRatio >= _MIN_INVARIANT_RATIO, Errors.MIN_BPT_IN_FOR_TOKEN_OUT);

        // Calculate by how much the token balance has to decrease to match invariantRatio
        uint256 balanceRatio = invariantRatio.powUp(FixedPoint.ONE.divDown(normalizedWeight));

        // Because of rounding up, balanceRatio can be greater than one. Using complement prevents reverts.
        uint256 amountOutWithoutFee = balance.mulDown(balanceRatio.complement());

        // We can now compute how much excess balance is being withdrawn as a result of the virtual swaps, which result
        // in swap fees.
        uint256 taxablePercentage = normalizedWeight.complement();

        // Swap fees are typically charged on 'token in', but there is no 'token in' here, so we apply it
        // to 'token out'. This results in slightly larger price impact. Fees are rounded up.
        uint256 taxableAmount = amountOutWithoutFee.mulUp(taxablePercentage);
        uint256 nonTaxableAmount = amountOutWithoutFee.sub(taxableAmount);

        return nonTaxableAmount.add(taxableAmount.mulDown(swapFee.complement()));
    }

    function _calcTokensOutGivenExactBptIn(
        uint256[] memory balances,
        uint256 bptAmountIn,
        uint256 totalBPT
    ) internal pure returns (uint256[] memory) {
        /**********************************************************************************************
        // exactBPTInForTokensOut                                                                    //
        // (per token)                                                                               //
        // aO = amountOut                  /        bptIn         \                                  //
        // b = balance           a0 = b * | ---------------------  |                                 //
        // bptIn = bptAmountIn             \       totalBPT       /                                  //
        // bpt = totalBPT                                                                            //
        **********************************************************************************************/

        // Since we're computing an amount out, we round down overall. This means rounding down on both the
        // multiplication and division.

        uint256 bptRatio = bptAmountIn.divDown(totalBPT);

        uint256[] memory amountsOut = new uint256[](balances.length);
        for (uint256 i = 0; i < balances.length; i++) {
            amountsOut[i] = balances[i].mulDown(bptRatio);
        }

        return amountsOut;
    }

    function _calcDueTokenProtocolSwapFeeAmount(
        uint256 balance,
        uint256 normalizedWeight,
        uint256 previousInvariant,
        uint256 currentInvariant,
        uint256 protocolSwapFeePercentage
    ) internal pure returns (uint256) {
        /*********************************************************************************
        /*  protocolSwapFeePercentage * balanceToken * ( 1 - (previousInvariant / currentInvariant) ^ (1 / weightToken))
        *********************************************************************************/

        if (currentInvariant <= previousInvariant) {
            // This shouldn't happen outside of rounding errors, but have this safeguard nonetheless to prevent the Pool
            // from entering a locked state in which joins and exits revert while computing accumulated swap fees.
            return 0;
        }

        // We round down to prevent issues in the Pool's accounting, even if it means paying slightly less in protocol
        // fees to the Vault.

        // Fee percentage and balance multiplications round down, while the subtrahend (power) rounds up (as does the
        // base). Because previousInvariant / currentInvariant <= 1, the exponent rounds down.

        uint256 base = previousInvariant.divUp(currentInvariant);
        uint256 exponent = FixedPoint.ONE.divDown(normalizedWeight);

        // Because the exponent is larger than one, the base of the power function has a lower bound. We cap to this
        // value to avoid numeric issues, which means in the extreme case (where the invariant growth is larger than
        // 1 / min exponent) the Pool will pay less in protocol fees than it should.
        base = Math.max(base, FixedPoint.MIN_POW_BASE_FREE_EXPONENT);

        uint256 power = base.powUp(exponent);

        uint256 tokenAccruedFees = balance.mulDown(power.complement());
        return tokenAccruedFees.mulDown(protocolSwapFeePercentage);
    }
}
