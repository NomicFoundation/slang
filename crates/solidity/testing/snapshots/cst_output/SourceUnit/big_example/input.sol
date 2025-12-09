// SPDX-License-Identifier: BUSL-1.1

// OpenZeppelin Contracts (last updated v4.8.0) (access/Ownable2Step.sol)

// OpenZeppelin Contracts (last updated v4.7.0) (access/Ownable.sol)

// OpenZeppelin Contracts v4.4.1 (utils/Context.sol)

/**
 * @dev Provides information about the current execution context, including the
 * sender of the transaction and its data. While these are generally available
 * via msg.sender and msg.data, they should not be accessed in such a direct
 * manner, since when dealing with meta-transactions the account sending and
 * paying for execution may not be the actual sender (as far as an application
 * is concerned).
 *
 * This contract is only required for intermediate, library-like contracts.
 */
abstract contract Context {
    function _msgSender() internal view virtual returns(address) {
        return msg.sender;
    }

    function _msgData() internal view virtual returns(bytes calldata) {
        return msg.data;
    }
}

/**
 * @dev Contract module which provides a basic access control mechanism, where
 * there is an account (an owner) that can be granted exclusive access to
 * specific functions.
 *
 * By default, the owner account will be the one that deploys the contract. This
 * can later be changed with {transferOwnership}.
 *
 * This module is used through inheritance. It will make available the modifier
 * `onlyOwner`, which can be applied to your functions to restrict their use to
 * the owner.
 */
abstract contract Ownable is Context {
    address private _owner;

    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);

    /**
     * @dev Initializes the contract setting the deployer as the initial owner.
     */
    constructor() {
        _transferOwnership(_msgSender());
    }

    /**
     * @dev Throws if called by any account other than the owner.
     */
    modifier onlyOwner() {
        _checkOwner();
        _;
    }

    /**
     * @dev Returns the address of the current owner.
     */
    function owner() public view virtual returns(address) {
        return _owner;
    }

    /**
     * @dev Throws if the sender is not the owner.
     */
    function _checkOwner() internal view virtual {
        require(owner() == _msgSender(), "Ownable: caller is not the owner");
    }

    /**
     * @dev Leaves the contract without owner. It will not be possible to call
     * `onlyOwner` functions anymore. Can only be called by the current owner.
     *
     * NOTE: Renouncing ownership will leave the contract without an owner,
     * thereby removing any functionality that is only available to the owner.
     */
    function renounceOwnership() public virtual onlyOwner {
        _transferOwnership(address(0));
    }

    /**
     * @dev Transfers ownership of the contract to a new account (`newOwner`).
     * Can only be called by the current owner.
     */
    function transferOwnership(address newOwner) public virtual onlyOwner {
        require(newOwner != address(0), "Ownable: new owner is the zero address");
        _transferOwnership(newOwner);
    }

    /**
     * @dev Transfers ownership of the contract to a new account (`newOwner`).
     * Internal function without access restriction.
     */
    function _transferOwnership(address newOwner) internal virtual {
        address oldOwner = _owner;
        _owner = newOwner;
        emit OwnershipTransferred(oldOwner, newOwner);
    }
}

/**
 * @dev Contract module which provides access control mechanism, where
 * there is an account (an owner) that can be granted exclusive access to
 * specific functions.
 *
 * By default, the owner account will be the one that deploys the contract. This
 * can later be changed with {transferOwnership} and {acceptOwnership}.
 *
 * This module is used through inheritance. It will make available all functions
 * from parent (Ownable).
 */
abstract contract Ownable2Step is Ownable {
    address private _pendingOwner;

    event OwnershipTransferStarted(address indexed previousOwner, address indexed newOwner);

    /**
     * @dev Returns the address of the pending owner.
     */
    function pendingOwner() public view virtual returns(address) {
        return _pendingOwner;
    }

    /**
     * @dev Starts the ownership transfer of the contract to a new account. Replaces the pending transfer if there is one.
     * Can only be called by the current owner.
     */
    function transferOwnership(address newOwner) public virtual   onlyOwner {
        _pendingOwner = newOwner;
        emit OwnershipTransferStarted(owner(), newOwner);
    }

    /**
     * @dev Transfers ownership of the contract to a new account (`newOwner`) and deletes any pending owner.
     * Internal function without access restriction.
     */
    function _transferOwnership(address newOwner) internal virtual   {
        delete _pendingOwner;
        super._transferOwnership(newOwner);
    }

    /**
     * @dev The new owner accepts the ownership transfer.
     */
    function acceptOwnership() external {
        address sender = _msgSender();
        require(pendingOwner() == sender, "Ownable2Step: caller is not the new owner");
        _transferOwnership(sender);
    }
}

// OpenZeppelin Contracts (last updated v4.7.0) (interfaces/IERC3156FlashBorrower.sol)

/**
 * @dev Interface of the ERC3156 FlashBorrower, as defined in
 * https://eips.ethereum.org/EIPS/eip-3156[ERC-3156].
 *
 * _Available since v4.1._
 */
interface IERC3156FlashBorrower {
    /**
     * @dev Receive a flash loan.
     * @param initiator The initiator of the loan.
     * @param token The loan currency.
     * @param amount The amount of tokens lent.
     * @param fee The additional amount of tokens to repay.
     * @param data Arbitrary data structure, intended to contain user-defined parameters.
     * @return The keccak256 hash of "IERC3156FlashBorrower.onFlashLoan"
     */
    function onFlashLoan(
    address initiator,
    address token,
    uint256 amount,
    uint256 fee,
    bytes calldata data
) external returns(bytes32);
}

// OpenZeppelin Contracts (last updated v4.6.0) (token/ERC20/IERC20.sol)

/**
 * @dev Interface of the ERC20 standard as defined in the EIP.
 */
interface IERC20 {
    /**
     * @dev Emitted when `value` tokens are moved from one account (`from`) to
     * another (`to`).
     *
     * Note that `value` may be zero.
     */
    event Transfer(address indexed from2, address indexed to, uint256 value);

    /**
     * @dev Emitted when the allowance of a `spender` for an `owner` is set by
     * a call to {approve}. `value` is the new allowance.
     */
    event Approval(address indexed owner, address indexed spender, uint256 value);

/**
 * @dev Returns the amount of tokens in existence.
 */
function totalSupply() external view returns(uint256);

/**
 * @dev Returns the amount of tokens owned by `account`.
 */
function balanceOf(address account) external view returns(uint256);

/**
 * @dev Moves `amount` tokens from the caller's account to `to`.
 *
 * Returns a boolean value indicating whether the operation succeeded.
 *
 * Emits a {Transfer} event.
 */
function transfer(address to, uint256 amount) external returns(bool);

/**
 * @dev Returns the remaining number of tokens that `spender` will be
 * allowed to spend on behalf of `owner` through {transferFrom}. This is
 * zero by default.
 *
 * This value changes when {approve} or {transferFrom} are called.
 */
function allowance(address owner, address spender) external view returns(uint256);

/**
 * @dev Sets `amount` as the allowance of `spender` over the caller's tokens.
 *
 * Returns a boolean value indicating whether the operation succeeded.
 *
 * IMPORTANT: Beware that changing an allowance with this method brings the risk
 * that someone may use both the old and the new allowance by unfortunate
 * transaction ordering. One possible solution to mitigate this race
 * condition is to first reduce the spender's allowance to 0 and set the
 * desired value afterwards:
 * https://github.com/ethereum/EIPs/issues/20#issuecomment-263524729
 *
 * Emits an {Approval} event.
 */
function approve(address spender, uint256 amount) external returns(bool);

/**
 * @dev Moves `amount` tokens from `from` to `to` using the
 * allowance mechanism. `amount` is then deducted from the caller's
 * allowance.
 *
 * Returns a boolean value indicating whether the operation succeeded.
 *
 * Emits a {Transfer} event.
 */
function transferFrom(
    address from2,
    address to,
    uint256 amount
) external returns(bool);
}

// OpenZeppelin Contracts (last updated v4.8.0) (token/ERC20/utils/SafeERC20.sol)

// OpenZeppelin Contracts v4.4.1 (token/ERC20/extensions/draft-IERC20Permit.sol)

/**
 * @dev Interface of the ERC20 Permit extension allowing approvals to be made via signatures, as defined in
 * https://eips.ethereum.org/EIPS/eip-2612[EIP-2612].
 *
 * Adds the {permit} method, which can be used to change an account's ERC20 allowance (see {IERC20-allowance}) by
 * presenting a message signed by the account. By not relying on {IERC20-approve}, the token holder account doesn't
 * need to send a transaction, and thus is not required to hold Ether at all.
 */
interface IERC20Permit {
    /**
     * @dev Sets `value` as the allowance of `spender` over ``owner``'s tokens,
     * given ``owner``'s signed approval.
     *
     * IMPORTANT: The same issues {IERC20-approve} has related to transaction
     * ordering also apply here.
     *
     * Emits an {Approval} event.
     *
     * Requirements:
     *
     * - `spender` cannot be the zero address.
     * - `deadline` must be a timestamp in the future.
     * - `v`, `r` and `s` must be a valid `secp256k1` signature from `owner`
     * over the EIP712-formatted function arguments.
     * - the signature must use ``owner``'s current nonce (see {nonces}).
     *
     * For more information on the signature format, see the
     * https://eips.ethereum.org/EIPS/eip-2612#specification[relevant EIP
     * section].
     */
    function permit(
    address owner,
    address spender,
    uint256 value,
    uint256 deadline,
    uint8 v,
    bytes32 r,
    bytes32 s
) external;

/**
 * @dev Returns the current nonce for `owner`. This value must be
 * included whenever a signature is generated for {permit}.
 *
 * Every successful call to {permit} increases ``owner``'s nonce by one. This
 * prevents a signature from being used multiple times.
 */
function nonces(address owner) external view returns(uint256);

/**
 * @dev Returns the domain separator used in the encoding of the signature for {permit}, as defined by {EIP712}.
 */
// solhint-disable-next-line func-name-mixedcase
function DOMAIN_SEPARATOR() external view returns(bytes32);
}

// OpenZeppelin Contracts (last updated v4.8.0) (utils/Address.sol)

/**
 * @dev Collection of functions related to the address type
 */
library Address {
    /**
     * @dev Returns true if `account` is a contract.
     *
     * [IMPORTANT]
     * ====
     * It is unsafe to assume that an address for which this function returns
     * false is an externally-owned account (EOA) and not a contract.
     *
     * Among others, `isContract` will return false for the following
     * types of addresses:
     *
     *  - an externally-owned account
     *  - a contract in construction
     *  - an address where a contract will be created
     *  - an address where a contract lived, but was destroyed
     * ====
     *
     * [IMPORTANT]
     * ====
     * You shouldn't rely on `isContract` to protect against flash loan attacks!
     *
     * Preventing calls from contracts is highly discouraged. It breaks composability, breaks support for smart wallets
     * like Gnosis Safe, and does not provide security since it can be circumvented by calling from a contract
     * constructor.
     * ====
     */
    function isContract(address account) internal view returns(bool) {
        // This method relies on extcodesize/address.code.length, which returns 0
        // for contracts in construction, since the code is only stored at the end
        // of the constructor execution.

        return account.code.length > 0;
    }

    /**
     * @dev Replacement for Solidity's `transfer`: sends `amount` wei to
     * `recipient`, forwarding all available gas and reverting on errors.
     *
     * https://eips.ethereum.org/EIPS/eip-1884[EIP1884] increases the gas cost
     * of certain opcodes, possibly making contracts go over the 2300 gas limit
     * imposed by `transfer`, making them unable to receive funds via
     * `transfer`. {sendValue} removes this limitation.
     *
     * https://diligence.consensys.net/posts/2019/09/stop-using-soliditys-transfer-now/[Learn more].
     *
     * IMPORTANT: because control is transferred to `recipient`, care must be
     * taken to not create reentrancy vulnerabilities. Consider using
     * {ReentrancyGuard} or the
     * https://solidity.readthedocs.io/en/v0.5.11/security-considerations.html#use-the-checks-effects-interactions-pattern[checks-effects-interactions pattern].
     */
    function sendValue(address payable recipient, uint256 amount) internal {
        require(address(this).balance >= amount, "Address: insufficient balance");

        (bool success, ) = recipient.call ("");
        require(success, "Address: unable to send value, recipient may have reverted");
    }

    /**
     * @dev Performs a Solidity function call using a low level `call`. A
     * plain `call` is an unsafe replacement for a function call: use this
     * function instead.
     *
     * If `target` reverts with a revert reason, it is bubbled up by this
     * function (like regular Solidity function calls).
     *
     * Returns the raw returned data. To convert to the expected return value,
     * use https://solidity.readthedocs.io/en/latest/units-and-global-variables.html?highlight=abi.decode#abi-encoding-and-decoding-functions[`abi.decode`].
     *
     * Requirements:
     *
     * - `target` must be a contract.
     * - calling `target` with `data` must not revert.
     *
     * _Available since v3.1._
     */
    function functionCall(address target, bytes memory data) internal returns(bytes memory) {
        return functionCallWithValue(target, data, 0, "Address: low-level call failed");
    }

    /**
     * @dev Same as {xref-Address-functionCall-address-bytes-}[`functionCall`], but with
     * `errorMessage` as a fallback revert reason when `target` reverts.
     *
     * _Available since v3.1._
     */
    function functionCall(
        address target,
        bytes memory data,
        string memory errorMessage
    ) internal returns(bytes memory) {
        return functionCallWithValue(target, data, 0, errorMessage);
    }

    /**
     * @dev Same as {xref-Address-functionCall-address-bytes-}[`functionCall`],
     * but also transferring `value` wei to `target`.
     *
     * Requirements:
     *
     * - the calling contract must have an ETH balance of at least `value`.
     * - the called Solidity function must be `payable`.
     *
     * _Available since v3.1._
     */
    function functionCallWithValue(
        address target,
        bytes memory data,
        uint256 value
    ) internal returns(bytes memory) {
        return functionCallWithValue(target, data, value, "Address: low-level call with value failed");
    }

    /**
     * @dev Same as {xref-Address-functionCallWithValue-address-bytes-uint256-}[`functionCallWithValue`], but
     * with `errorMessage` as a fallback revert reason when `target` reverts.
     *
     * _Available since v3.1._
     */
    function functionCallWithValue(
        address target,
        bytes memory data,
        uint256 value,
        string memory errorMessage
    ) internal returns(bytes memory) {
        require(address(this).balance >= value, "Address: insufficient balance for call");
        (bool success, bytes memory returndata) = target.call(data);
        return verifyCallResultFromTarget(target, success, returndata, errorMessage);
    }

    /**
     * @dev Same as {xref-Address-functionCall-address-bytes-}[`functionCall`],
     * but performing a static call.
     *
     * _Available since v3.3._
     */
    function functionStaticCall(address target, bytes memory data) internal view returns(bytes memory) {
        return functionStaticCall(target, data, "Address: low-level static call failed");
    }

    /**
     * @dev Same as {xref-Address-functionCall-address-bytes-string-}[`functionCall`],
     * but performing a static call.
     *
     * _Available since v3.3._
     */
    function functionStaticCall(
        address target,
        bytes memory data,
        string memory errorMessage
    ) internal view returns(bytes memory) {
        (bool success, bytes memory returndata) = target.staticcall(data);
        return verifyCallResultFromTarget(target, success, returndata, errorMessage);
    }

    /**
     * @dev Same as {xref-Address-functionCall-address-bytes-}[`functionCall`],
     * but performing a delegate call.
     *
     * _Available since v3.4._
     */
    function functionDelegateCall(address target, bytes memory data) internal returns(bytes memory) {
        return functionDelegateCall(target, data, "Address: low-level delegate call failed");
    }

    /**
     * @dev Same as {xref-Address-functionCall-address-bytes-string-}[`functionCall`],
     * but performing a delegate call.
     *
     * _Available since v3.4._
     */
    function functionDelegateCall(
        address target,
        bytes memory data,
        string memory errorMessage
    ) internal returns(bytes memory) {
        (bool success, bytes memory returndata) = target.delegatecall(data);
        return verifyCallResultFromTarget(target, success, returndata, errorMessage);
    }

    /**
     * @dev Tool to verify that a low level call to smart-contract was successful, and revert (either by bubbling
     * the revert reason or using the provided one) in case of unsuccessful call or if target was not a contract.
     *
     * _Available since v4.8._
     */
    function verifyCallResultFromTarget(
        address target,
        bool success,
        bytes memory returndata,
        string memory errorMessage
    ) internal view returns(bytes memory) {
        if (success) {
            if (returndata.length == 0) {
                // only check isContract if the call was successful and the return data is empty
                // otherwise we already know that it was a contract
                require(isContract(target), "Address: call to non-contract");
            }
            return returndata;
        } else {
            _revert(returndata, errorMessage);
        }
    }

    /**
     * @dev Tool to verify that a low level call was successful, and revert if it wasn't, either by bubbling the
     * revert reason or using the provided one.
     *
     * _Available since v4.3._
     */
    function verifyCallResult(
        bool success,
        bytes memory returndata,
        string memory errorMessage
    ) internal pure returns(bytes memory) {
        if (success) {
            return returndata;
        } else {
            _revert(returndata, errorMessage);
        }
    }

    function _revert(bytes memory returndata, string memory errorMessage) private pure {
        // Look for revert reason and bubble it up if present
        if (returndata.length > 0) {
            // The easiest way to bubble the revert reason is using memory via assembly
            /// @solidity memory-safe-assembly
            assembly {
            }
        } else {
            revert(errorMessage);
        }
    }
}

/**
 * @title SafeERC20
 * @dev Wrappers around ERC20 operations that throw on failure (when the token
 * contract returns false). Tokens that return no value (and instead revert or
 * throw on failure) are also supported, non-reverting calls are assumed to be
 * successful.
 * To use this library you can add a `using SafeERC20 for IERC20;` statement to your contract,
 * which allows you to call the safe operations as `token.safeTransfer(...)`, etc.
 */
library SafeERC20 {
    using Address for address;

        function safeTransfer(
            IERC20 token,
            address to,
            uint256 value
        ) internal {
        _callOptionalReturn(token, abi.encodeWithSelector(token.transfer.selector, to, value));
    }

    function safeTransferFrom(
        IERC20 token,
        address from2,
        address to,
        uint256 value
    ) internal {
        _callOptionalReturn(token, abi.encodeWithSelector(token.transferFrom.selector, from2, to, value));
    }

    /**
     * @dev Deprecated. This function has issues similar to the ones found in
     * {IERC20-approve}, and its usage is discouraged.
     *
     * Whenever possible, use {safeIncreaseAllowance} and
     * {safeDecreaseAllowance} instead.
     */
    function safeApprove(
        IERC20 token,
        address spender,
        uint256 value
    ) internal {
        // safeApprove should only be called when setting an initial allowance,
        // or when resetting it to zero. To increase and decrease it, use
        // 'safeIncreaseAllowance' and 'safeDecreaseAllowance'
        require(
            (value == 0) || (token.allowance(address(this), spender) == 0),
            "SafeERC20: approve from non-zero to non-zero allowance"
        );
        _callOptionalReturn(token, abi.encodeWithSelector(token.approve.selector, spender, value));
    }

    function safeIncreaseAllowance(
        IERC20 token,
        address spender,
        uint256 value
    ) internal {
        uint256 newAllowance = token.allowance(address(this), spender) + value;
        _callOptionalReturn(token, abi.encodeWithSelector(token.approve.selector, spender, newAllowance));
    }

    function safeDecreaseAllowance(
        IERC20 token,
        address spender,
        uint256 value
    ) internal {
        unchecked {
            uint256 oldAllowance = token.allowance(address(this), spender);
            require(oldAllowance >= value, "SafeERC20: decreased allowance below zero");
            uint256 newAllowance = oldAllowance - value;
            _callOptionalReturn(token, abi.encodeWithSelector(token.approve.selector, spender, newAllowance));
        }
    }

    function safePermit(
        IERC20Permit token,
        address owner,
        address spender,
        uint256 value,
        uint256 deadline,
        uint8 v,
        bytes32 r,
        bytes32 s
    ) internal {
        uint256 nonceBefore = token.nonces(owner);
        token.permit(owner, spender, value, deadline, v, r, s);
        uint256 nonceAfter = token.nonces(owner);
        require(nonceAfter == nonceBefore + 1, "SafeERC20: permit did not succeed");
    }

    /**
     * @dev Imitates a Solidity high-level call (i.e. a regular function call to a contract), relaxing the requirement
     * on the return value: the return value is optional (but if data is returned, it must not be false).
     * @param token The token targeted by the call.
     * @param data The call data (encoded using abi.encode or one of its variants).
     */
    function _callOptionalReturn(IERC20 token, bytes memory data) private {
        // We need to perform a low level call here, to bypass Solidity's return data size checking mechanism, since
        // we're implementing it ourselves. We use {Address-functionCall} to perform this call, which verifies that
        // the target address contains contract code and also asserts for success in the low-level call.

        bytes memory returndata = address(token).functionCall(data, "SafeERC20: low-level call failed");
        if (returndata.length > 0) {
            // Return data is optional
            require(abi.decode(returndata, (bool)), "SafeERC20: ERC20 operation did not succeed");
        }
    }
}

/// Parameters for ERC20Permit.permit call
struct ERC20PermitSignature {
    IERC20Permit token;
    uint256 value;
    uint256 deadline;
    uint8 v;
    bytes32 r;
    bytes32 s;
}

library PermitHelper {
    function applyPermit(
        ERC20PermitSignature calldata p,
        address owner,
        address spender
    ) internal {
        p.token.permit(owner, spender, p.value, p.deadline, p.v, p.r, p.s);
    }

    function applyPermits(
        ERC20PermitSignature calldata permits,
        address owner,
        address spender
    ) internal {
        for (uint256 i = 0; i < permits.length; i++) {
            applyPermit(permits[i], owner, spender);
        }
    }
}

interface IPositionManagerDependent {
    // --- Errors ---

    /// @dev Position Manager cannot be zero.
    error PositionManagerCannotBeZero();

    /// @dev Caller is not Position Manager.
    error CallerIsNotPositionManager(address caller);

// --- Functions ---

/// @dev Returns address of the PositionManager contract.
function positionManager() external view returns(address);
}

interface IERC20Indexable is IERC20, IPositionManagerDependent {
    // --- Events ---

    /// @dev New token is deployed.
    /// @param positionManager Address of the PositionManager contract that is authorized to mint and burn new tokens.
    event ERC20IndexableDeployed(address positionManager);

    /// @dev New index has been set.
    /// @param newIndex Value of the new index.
    event IndexUpdated(uint256 newIndex);

    // --- Errors ---

    /// @dev Unsupported action for ERC20Indexable contract.
    error NotSupported();

    // --- Functions ---

    /// @return Precision for token index. Represents index that is equal to 1.
    function INDEX_PRECISION() external view returns(uint256);

    /// @return Current index value.
    function currentIndex() external view returns(uint256);

    /// @dev Sets new token index. Callable only by PositionManager contract.
    /// @param backingAmount Amount of backing token that is covered by total supply.
    function setIndex(uint256 backingAmount) external;

    /// @dev Mints new tokens. Callable only by PositionManager contract.
    /// @param to Address that will receive newly minted tokens.
    /// @param amount Amount of tokens to mint.
    function mint(address to, uint256 amount) external;

    /// @dev Mints new tokens. Callable only by PositionManager contract.
    /// @param from Address of user whose tokens are burnt.
    /// @param amount Amount of tokens to burn.
    function burn(address from2, uint256 amount) external;
}

/// @dev Interface to be used by contracts that collect fees. Contains fee recipient that can be changed by owner.
interface IFeeCollector {
    // --- Events ---

    /// @dev Fee Recipient is changed to @param feeRecipient address.
    /// @param feeRecipient New fee recipient address.
    event FeeRecipientChanged(address feeRecipient);

    // --- Errors ---

    /// @dev Invalid fee recipient.
    error InvalidFeeRecipient();

// --- Functions ---

/// @return Address of the current fee recipient.
function feeRecipient() external view returns(address);

/// @dev Sets new fee recipient address
/// @param newFeeRecipient Address of the new fee recipient.
function setFeeRecipient(address newFeeRecipient) external;
}

interface IPriceOracle {
    // --- Errors ---

    /// @dev Contract initialized with an invalid deviation parameter.
    error InvalidDeviation();

    // --- Types ---

    struct PriceOracleResponse {
        bool isBrokenOrFrozen;
        bool priceChangeAboveMax;
        uint256 price;
}

// --- Functions ---

/// @dev Return price oracle response which consists the following information: oracle is broken or frozen, the
/// price change between two rounds is more than max, and the price.
function getPriceOracleResponse() external returns(PriceOracleResponse memory);

/// @dev Maximum time period allowed since oracle latest round data timestamp, beyond which oracle is considered
/// frozen.
function timeout() external view returns(uint256);

/// @dev Used to convert a price answer to an 18-digit precision uint.
function TARGET_DIGITS() external view returns(uint256);

/// @dev price deviation for the oracle in percentage.
function DEVIATION() external view returns(uint256);
}

interface IPriceFeed {
    // --- Events ---

    /// @dev Last good price has been updated.
    event LastGoodPriceUpdated(uint256 lastGoodPrice);

    /// @dev Price difference between oracles has been updated.
    /// @param priceDifferenceBetweenOracles New price difference between oracles.
    event PriceDifferenceBetweenOraclesUpdated(uint256 priceDifferenceBetweenOracles);

    /// @dev Primary oracle has been updated.
    /// @param primaryOracle New primary oracle.
    event PrimaryOracleUpdated(IPriceOracle primaryOracle);

    /// @dev Secondary oracle has been updated.
    /// @param secondaryOracle New secondary oracle.
    event SecondaryOracleUpdated(IPriceOracle secondaryOracle);

    // --- Errors ---

    /// @dev Invalid primary oracle.
    error InvalidPrimaryOracle();

    /// @dev Invalid secondary oracle.
    error InvalidSecondaryOracle();

    /// @dev Primary oracle is broken or frozen or has bad result.
    error PrimaryOracleBrokenOrFrozenOrBadResult();

    /// @dev Invalid price difference between oracles.
    error InvalidPriceDifferenceBetweenOracles();

// --- Functions ---

/// @dev Return primary oracle address.
function primaryOracle() external returns(IPriceOracle);

/// @dev Return secondary oracle address
function secondaryOracle() external returns(IPriceOracle);

/// @dev The last good price seen from an oracle by Raft.
function lastGoodPrice() external returns(uint256);

/// @dev The maximum relative price difference between two oracle responses.
function priceDifferenceBetweenOracles() external returns(uint256);

/// @dev Set primary oracle address.
/// @param newPrimaryOracle Primary oracle address.
function setPrimaryOracle(IPriceOracle newPrimaryOracle) external;

/// @dev Set secondary oracle address.
/// @param newSecondaryOracle Secondary oracle address.
function setSecondaryOracle(IPriceOracle newSecondaryOracle) external;

/// @dev Set the maximum relative price difference between two oracle responses.
/// @param newPriceDifferenceBetweenOracles The maximum relative price difference between two oracle responses.
function setPriceDifferenceBetweenOracles(uint256 newPriceDifferenceBetweenOracles) external;

/// @dev Returns the latest price obtained from the Oracle. Called by Raft functions that require a current price.
///
/// Also callable by anyone externally.
/// Non-view function - it stores the last good price seen by Raft.
///
/// Uses a primary oracle and a fallback oracle in case primary fails. If both fail,
/// it uses the last good price seen by Raft.
///
/// @return currentPrice Returned price.
/// @return deviation Deviation of the reported price in percentage.
/// @notice Actual returned price is in range `currentPrice` +/- `currentPrice * deviation / ONE`
function fetchPrice() external returns(uint256 currentPrice, uint256 deviation);
}

// OpenZeppelin Contracts v4.4.1 (interfaces/IERC3156FlashLender.sol)

/**
 * @dev Interface of the ERC3156 FlashLender, as defined in
 * https://eips.ethereum.org/EIPS/eip-3156[ERC-3156].
 *
 * _Available since v4.1._
 */
interface IERC3156FlashLender {
    /**
     * @dev The amount of currency available to be lended.
     * @param token The loan currency.
     * @return The amount of `token` that can be borrowed.
     */
    function maxFlashLoan(address token) external view returns(uint256);

/**
 * @dev The fee to be charged for a given loan.
 * @param token The loan currency.
 * @param amount The amount of tokens lent.
 * @return The amount of `token` to be charged for the loan, on top of the returned principal.
 */
function flashFee(address token, uint256 amount) external view returns(uint256);

/**
 * @dev Initiate a flash loan.
 * @param receiver The receiver of the tokens in the loan, and the receiver of the callback.
 * @param token The loan currency.
 * @param amount The amount of tokens lent.
 * @param data Arbitrary data structure, intended to contain user-defined parameters.
 */
function flashLoan(
    IERC3156FlashBorrower receiver,
    address token,
    uint256 amount,
    bytes calldata data
) external returns(bool);
}

/// @dev Interface of R stablecoin token. Implements some standards like IERC20, IERC20Permit, and IERC3156FlashLender.
/// Raft's specific implementation contains IFeeCollector and IPositionManagerDependent.
/// PositionManager can mint and burn R when particular actions happen with user's position.
interface IRToken is IERC20, IERC20Permit, IERC3156FlashLender, IFeeCollector, IPositionManagerDependent {
    // --- Events ---

    /// @dev New R token is deployed
    /// @param positionManager Address of the PositionManager contract that is authorized to mint and burn new tokens.
    /// @param flashMintFeeRecipient Address of flash mint fee recipient.
    event RDeployed(address positionManager, address flashMintFeeRecipient);

    /// @dev The Flash Mint Fee Percentage has been changed.
    /// @param flashMintFeePercentage The new Flash Mint Fee Percentage value.
    event FlashMintFeePercentageChanged(uint256 flashMintFeePercentage);

    /// --- Errors ---

    /// @dev Proposed flash mint fee percentage is too big.
    /// @param feePercentage Proposed flash mint fee percentage.
    error FlashFeePercentageTooBig(uint256 feePercentage);

    // --- Functions ---

    /// @return Number representing 100 percentage.
    function PERCENTAGE_BASE() external view returns(uint256);

    /// @dev Mints new tokens. Callable only by PositionManager contract.
    /// @param to Address that will receive newly minted tokens.
    /// @param amount Amount of tokens to mint.
    function mint(address to, uint256 amount) external;

    /// @dev Mints new tokens. Callable only by PositionManager contract.
    /// @param from Address of user whose tokens are burnt.
    /// @param amount Amount of tokens to burn.
    function burn(address from2, uint256 amount) external;

    /// @return Maximum flash mint fee percentage that can be set by owner.
    function MAX_FLASH_MINT_FEE_PERCENTAGE() external view returns(uint256);

    /// @return Current flash mint fee percentage.
    function flashMintFeePercentage() external view returns(uint256);

    /// @dev Sets new flash mint fee percentage. Callable only by owner.
    /// @notice The proposed flash mint fee percentage cannot exceed `MAX_FLASH_MINT_FEE_PERCENTAGE`.
    /// @param feePercentage New flash fee percentage.
    function setFlashMintFeePercentage(uint256 feePercentage) external;
}

interface ISplitLiquidationCollateral {
    // --- Functions ---

    /// @dev Returns lowest total debt that will be split.
    function LOW_TOTAL_DEBT() external view returns(uint256);

/// @dev Minimum collateralization ratio for position
function MCR() external view returns(uint256);

/// @dev Splits collateral between protocol and liquidator.
/// @param totalCollateral Amount of collateral to split.
/// @param totalDebt Amount of debt to split.
/// @param price Price of collateral.
/// @param isRedistribution True if this is a redistribution.
/// @return collateralToSendToProtocol Amount of collateral to send to protocol.
/// @return collateralToSentToLiquidator Amount of collateral to send to liquidator.
function split(
    uint256 totalCollateral,
    uint256 totalDebt,
    uint256 price,
    bool isRedistribution
)
external
view
returns(uint256 collateralToSendToProtocol, uint256 collateralToSentToLiquidator);
}

/// @dev Common interface for the Position Manager.
interface IPositionManager is IFeeCollector {
    // --- Types ---

    /// @dev Information for a Raft indexable collateral token.
    /// @param collateralToken The Raft indexable collateral token.
    /// @param debtToken Corresponding Raft indexable debt token.
    /// @param priceFeed The contract that provides a price for the collateral token.
    /// @param splitLiquidation The contract that calculates collateral split in case of liquidation.
    /// @param isEnabled Whether the token can be used as collateral or not.
    /// @param lastFeeOperationTime Timestamp of the last operation for the collateral token.
    /// @param borrowingSpread The current borrowing spread.
    /// @param baseRate The current base rate.
    /// @param redemptionSpread The current redemption spread.
    /// @param redemptionRebate Percentage of the redemption fee returned to redeemed positions.
    struct CollateralTokenInfo {
        IERC20Indexable collateralToken;
        IERC20Indexable debtToken;
        IPriceFeed priceFeed;
        ISplitLiquidationCollateral splitLiquidation;
        bool isEnabled;
        uint256 lastFeeOperationTime;
        uint256 borrowingSpread;
        uint256 baseRate;
        uint256 redemptionSpread;
        uint256 redemptionRebate;
    }

    // --- Events ---

    /// @dev New position manager has been token deployed.
    /// @param rToken The R token used by the position manager.
    /// @param feeRecipient The address of fee recipient.
    event PositionManagerDeployed(IRToken rToken, address feeRecipient);

    /// @dev New collateral token has been added added to the system.
    /// @param collateralToken The token used as collateral.
    /// @param raftCollateralToken The Raft indexable collateral token for the given collateral token.
    /// @param raftDebtToken The Raft indexable debt token for given collateral token.
    /// @param priceFeed The contract that provides price for the collateral token.
    event CollateralTokenAdded(
        IERC20 collateralToken,
        IERC20Indexable raftCollateralToken,
        IERC20Indexable raftDebtToken,
        IPriceFeed priceFeed
    );

    /// @dev Collateral token has been enabled or disabled.
    /// @param collateralToken The token used as collateral.
    /// @param isEnabled True if the token is enabled, false otherwise.
    event CollateralTokenModified(IERC20 collateralToken, bool isEnabled);

    /// @dev A delegate has been whitelisted for a certain position.
    /// @param position The position for which the delegate was whitelisted.
    /// @param delegate The delegate which was whitelisted.
    /// @param whitelisted Specifies whether the delegate whitelisting has been enabled (true) or disabled (false).
    event DelegateWhitelisted(address indexed position, address indexed delegate, bool whitelisted);

    /// @dev New position has been created.
    /// @param position The address of the user opening new position.
    /// @param collateralToken The token used as collateral for the created position.
    event PositionCreated(address indexed position, IERC20 indexed collateralToken);

    /// @dev The position has been closed by either repayment, liquidation, or redemption.
    /// @param position The address of the user whose position is closed.
    event PositionClosed(address indexed position);

    /// @dev Collateral amount for the position has been changed.
    /// @param position The address of the user that has opened the position.
    /// @param collateralToken The address of the collateral token being added to position.
    /// @param collateralAmount The amount of collateral added or removed.
    /// @param isCollateralIncrease Whether the collateral is added to the position or removed from it.
    event CollateralChanged(
        address indexed position, IERC20 indexed collateralToken, uint256 collateralAmount, bool isCollateralIncrease
    );

    /// @dev Debt amount for position has been changed.
    /// @param position The address of the user that has opened the position.
    /// @param collateralToken The address of the collateral token backing the debt.
    /// @param debtAmount The amount of debt added or removed.
    /// @param isDebtIncrease Whether the debt is added to the position or removed from it.
    event DebtChanged(
        address indexed position, IERC20 indexed collateralToken, uint256 debtAmount, bool isDebtIncrease
    );

    /// @dev Borrowing fee has been paid. Emitted only if the actual fee was paid - doesn't happen with no fees are
    /// paid.
    /// @param collateralToken Collateral token used to mint R.
    /// @param position The address of position's owner that triggered the fee payment.
    /// @param feeAmount The amount of tokens paid as the borrowing fee.
    event RBorrowingFeePaid(IERC20 collateralToken, address indexed position, uint256 feeAmount);

    /// @dev Liquidation has been executed.
    /// @param liquidator The liquidator that executed the liquidation.
    /// @param position The address of position's owner whose position was liquidated.
    /// @param collateralToken The collateral token used for the liquidation.
    /// @param debtLiquidated The total debt that was liquidated or redistributed.
    /// @param collateralLiquidated The total collateral liquidated.
    /// @param collateralSentToLiquidator The collateral amount sent to the liquidator.
    /// @param collateralLiquidationFeePaid The total collateral paid as the liquidation fee to the fee recipient.
    /// @param isRedistribution Whether the executed liquidation was redistribution or not.
    event Liquidation(
        address indexed liquidator,
        address indexed position,
        IERC20 indexed collateralToken,
        uint256 debtLiquidated,
        uint256 collateralLiquidated,
        uint256 collateralSentToLiquidator,
        uint256 collateralLiquidationFeePaid,
        bool isRedistribution
    );

    /// @dev Redemption has been executed.
    /// @param redeemer User that redeemed R.
    /// @param amount Amount of R that was redeemed.
    /// @param collateralSent The amount of collateral sent to the redeemer.
    /// @param fee The amount of fee paid to the fee recipient.
    /// @param rebate Redemption rebate amount.
    event Redemption(address indexed redeemer, uint256 amount, uint256 collateralSent, uint256 fee, uint256 rebate);

    /// @dev Borrowing spread has been updated.
    /// @param borrowingSpread The new borrowing spread.
    event BorrowingSpreadUpdated(uint256 borrowingSpread);

    /// @dev Redemption rebate has been updated.
    /// @param redemptionRebate The new redemption rebate.
    event RedemptionRebateUpdated(uint256 redemptionRebate);

    /// @dev Redemption spread has been updated.
    /// @param collateralToken Collateral token that the spread was set for.
    /// @param redemptionSpread The new redemption spread.
    event RedemptionSpreadUpdated(IERC20 collateralToken, uint256 redemptionSpread);

    /// @dev Base rate has been updated.
    /// @param collateralToken Collateral token that the baser rate was updated for.
    /// @param baseRate The new base rate.
    event BaseRateUpdated(IERC20 collateralToken, uint256 baseRate);

    /// @dev Last fee operation time has been updated.
    /// @param collateralToken Collateral token that the baser rate was updated for.
    /// @param lastFeeOpTime The new operation time.
    event LastFeeOpTimeUpdated(IERC20 collateralToken, uint256 lastFeeOpTime);

    /// @dev Split liquidation collateral has been changed.
    /// @param collateralToken Collateral token whose split liquidation collateral contract is set.
    /// @param newSplitLiquidationCollateral New value that was set to be split liquidation collateral.
    event SplitLiquidationCollateralChanged(
        IERC20 collateralToken, ISplitLiquidationCollateral indexed newSplitLiquidationCollateral
    );

    // --- Errors ---

    /// @dev Max fee percentage must be between borrowing spread and 100%.
    error InvalidMaxFeePercentage();

    /// @dev Max fee percentage must be between 0.5% and 100%.
    error MaxFeePercentageOutOfRange();

    /// @dev Amount is zero.
    error AmountIsZero();

    /// @dev Nothing to liquidate.
    error NothingToLiquidate();

    /// @dev Cannot liquidate last position.
    error CannotLiquidateLastPosition();

    /// @dev Cannot redeem collateral below minimum debt threshold.
    /// @param collateralToken Collateral token used to redeem.
    /// @param newTotalDebt New total debt backed by collateral, which is lower than minimum debt.
    error TotalDebtCannotBeLowerThanMinDebt(IERC20 collateralToken, uint256 newTotalDebt);

    /// @dev Cannot redeem collateral
    /// @param collateralToken Collateral token used to redeem.
    /// @param newTotalCollateral New total collateral, which is lower than minimum collateral.
    /// @param minimumCollateral Minimum collateral required to complete redeem
    error TotalCollateralCannotBeLowerThanMinCollateral(
        IERC20 collateralToken, uint256 newTotalCollateral, uint256 minimumCollateral
    );

    /// @dev Fee would eat up all returned collateral.
    error FeeEatsUpAllReturnedCollateral();

    /// @dev Borrowing spread exceeds maximum.
    error BorrowingSpreadExceedsMaximum();

    /// @dev Redemption rebate exceeds maximum.
    error RedemptionRebateExceedsMaximum();

    /// @dev Redemption spread is out of allowed range.
    error RedemptionSpreadOutOfRange();

    /// @dev There must be either a collateral change or a debt change.
    error NoCollateralOrDebtChange();

    /// @dev There is some collateral for position that doesn't have debt.
    error InvalidPosition();

    /// @dev An operation that would result in ICR < MCR is not permitted.
    /// @param newICR Resulting ICR that is bellow MCR.
    error NewICRLowerThanMCR(uint256 newICR);

    /// @dev Position's net debt must be greater than minimum.
    /// @param netDebt Net debt amount that is below minimum.
    error NetDebtBelowMinimum(uint256 netDebt);

    /// @dev The provided delegate address is invalid.
    error InvalidDelegateAddress();

    /// @dev A non-whitelisted delegate cannot adjust positions.
    error DelegateNotWhitelisted();

    /// @dev Fee exceeded provided maximum fee percentage.
    /// @param fee The fee amount.
    /// @param amount The amount of debt or collateral.
    /// @param maxFeePercentage The maximum fee percentage.
    error FeeExceedsMaxFee(uint256 fee, uint256 amount, uint256 maxFeePercentage);

    /// @dev Borrower uses a different collateral token already.
    error PositionCollateralTokenMismatch();

    /// @dev Collateral token address cannot be zero.
    error CollateralTokenAddressCannotBeZero();

    /// @dev Price feed address cannot be zero.
    error PriceFeedAddressCannotBeZero();

    /// @dev Collateral token already added.
    error CollateralTokenAlreadyAdded();

    /// @dev Collateral token is not added.
    error CollateralTokenNotAdded();

    /// @dev Collateral token is not enabled.
    error CollateralTokenDisabled();

    /// @dev Split liquidation collateral cannot be zero.
    error SplitLiquidationCollateralCannotBeZero();

    /// @dev Cannot change collateral in case of repaying the whole debt.
    error WrongCollateralParamsForFullRepayment();

    // --- Functions ---

    /// @return The R token used by position manager.
    function rToken() external view returns(IRToken);

    /// @dev Retrieves information about certain collateral type.
    /// @param collateralToken The token used as collateral.
    /// @return raftCollateralToken The Raft indexable collateral token.
    /// @return raftDebtToken The Raft indexable debt token.
    /// @return priceFeed The contract that provides a price for the collateral token.
    /// @return splitLiquidation The contract that calculates collateral split in case of liquidation.
    /// @return isEnabled Whether the collateral token can be used as collateral or not.
    /// @return lastFeeOperationTime Timestamp of the last operation for the collateral token.
    /// @return borrowingSpread The current borrowing spread.
    /// @return baseRate The current base rate.
    /// @return redemptionSpread The current redemption spread.
    /// @return redemptionRebate Percentage of the redemption fee returned to redeemed positions.
    function collateralInfo(IERC20 collateralToken)
    external
    view
    returns(
        IERC20Indexable raftCollateralToken,
        IERC20Indexable raftDebtToken,
        IPriceFeed priceFeed,
        ISplitLiquidationCollateral splitLiquidation,
        bool isEnabled,
        uint256 lastFeeOperationTime,
        uint256 borrowingSpread,
        uint256 baseRate,
        uint256 redemptionSpread,
        uint256 redemptionRebate
    );

    /// @param collateralToken Collateral token whose raft collateral indexable token is being queried.
    /// @return Raft collateral token address for given collateral token.
    function raftCollateralToken(IERC20 collateralToken) external view returns(IERC20Indexable);

    /// @param collateralToken Collateral token whose raft collateral indexable token is being queried.
    /// @return Raft debt token address for given collateral token.
    function raftDebtToken(IERC20 collateralToken) external view returns(IERC20Indexable);

    /// @param collateralToken Collateral token whose price feed contract is being queried.
    /// @return Price feed contract address for given collateral token.
    function priceFeed(IERC20 collateralToken) external view returns(IPriceFeed);

    /// @param collateralToken Collateral token whose split liquidation collateral is being queried.
    /// @return Returns address of the split liquidation collateral contract.
    function splitLiquidationCollateral(IERC20 collateralToken) external view returns(ISplitLiquidationCollateral);

    /// @param collateralToken Collateral token whose split liquidation collateral is being queried.
    /// @return Returns whether collateral is enabled or nor.
    function collateralEnabled(IERC20 collateralToken) external view returns(bool);

    /// @param collateralToken Collateral token we query last operation time fee for.
    /// @return The timestamp of the latest fee operation (redemption or new R issuance).
    function lastFeeOperationTime(IERC20 collateralToken) external view returns(uint256);

    /// @param collateralToken Collateral token we query borrowing spread for.
    /// @return The current borrowing spread.
    function borrowingSpread(IERC20 collateralToken) external view returns(uint256);

    /// @param collateralToken Collateral token we query base rate for.
    /// @return rate The base rate.
    function baseRate(IERC20 collateralToken) external view returns(uint256 rate);

    /// @param collateralToken Collateral token we query redemption spread for.
    /// @return The current redemption spread for collateral token.
    function redemptionSpread(IERC20 collateralToken) external view returns(uint256);

    /// @param collateralToken Collateral token we query redemption rebate for.
    /// @return rebate Percentage of the redemption fee returned to redeemed positions.
    function redemptionRebate(IERC20 collateralToken) external view returns(uint256);

    /// @param collateralToken Collateral token we query redemption rate for.
    /// @return rate The current redemption rate for collateral token.
    function getRedemptionRate(IERC20 collateralToken) external view returns(uint256 rate);

    /// @dev Returns the collateral token that a given position used for their position.
    /// @param position The address of the borrower.
    /// @return collateralToken The collateral token of the borrower's position.
    function collateralTokenForPosition(address position) external view returns(IERC20 collateralToken);

    /// @dev Adds a new collateral token to the protocol.
    /// @param collateralToken The new collateral token.
    /// @param priceFeed The price feed for the collateral token.
    /// @param newSplitLiquidationCollateral split liquidation collateral contract address.
    function addCollateralToken(
        IERC20 collateralToken,
        IPriceFeed priceFeed,
        ISplitLiquidationCollateral newSplitLiquidationCollateral
    )
    external;

    /// @dev Enables or disables a collateral token. Reverts if the collateral token has not been added.
    /// @param collateralToken The collateral token.
    /// @param isEnabled Whether the collateral token can be used as collateral or not.
    function setCollateralEnabled(IERC20 collateralToken, bool isEnabled) external;

    /// @dev Sets the new split liquidation collateral contract.
    /// @param collateralToken Collateral token whose split liquidation collateral is being set.
    /// @param newSplitLiquidationCollateral New split liquidation collateral contract address.
    function setSplitLiquidationCollateral(
        IERC20 collateralToken,
        ISplitLiquidationCollateral newSplitLiquidationCollateral
    )
    external;

    /// @dev Liquidates the borrower if its position's ICR is lower than the minimum collateral ratio.
    /// @param position The address of the borrower.
    function liquidate(address position) external;

    /// @dev Redeems the collateral token for a given debt amount. It sends @param debtAmount R to the system and
    /// redeems the corresponding amount of collateral from as many positions as are needed to fill the redemption
    /// request.
    /// @param collateralToken The token used as collateral.
    /// @param debtAmount The amount of debt to be redeemed. Must be greater than zero.
    /// @param maxFeePercentage The maximum fee percentage to pay for the redemption.
    function redeemCollateral(IERC20 collateralToken, uint256 debtAmount, uint256 maxFeePercentage) external;

    /// @dev Manages the position on behalf of a given borrower.
    /// @param collateralToken The token the borrower used as collateral.
    /// @param position The address of the borrower.
    /// @param collateralChange The amount of collateral to add or remove.
    /// @param isCollateralIncrease True if the collateral is being increased, false otherwise.
    /// @param debtChange The amount of R to add or remove. In case of repayment (isDebtIncrease = false)
    /// `type(uint256).max` value can be used to repay the whole outstanding loan.
    /// @param isDebtIncrease True if the debt is being increased, false otherwise.
    /// @param maxFeePercentage The maximum fee percentage to pay for the position management.
    /// @param permitSignature Optional permit signature for tokens that support IERC20Permit interface.
    /// @notice `permitSignature` it is ignored if permit signature is not for `collateralToken`.
    /// @notice In case of full debt repayment, `isCollateralIncrease` is ignored and `collateralChange` must be 0.
    /// These values are set to `false`(collateral decrease), and the whole collateral balance of the user.
    /// @return actualCollateralChange Actual amount of collateral added/removed.
    /// Can be different to `collateralChange` in case of full repayment.
    /// @return actualDebtChange Actual amount of debt added/removed.
    /// Can be different to `debtChange` in case of passing type(uint256).max as `debtChange`.
    function managePosition(
        IERC20 collateralToken,
        address position,
        uint256 collateralChange,
        bool isCollateralIncrease,
        uint256 debtChange,
        bool isDebtIncrease,
        uint256 maxFeePercentage,
        ERC20PermitSignature calldata permitSignature
    )
    external
    returns(uint256 actualCollateralChange, uint256 actualDebtChange);

    /// @return The max borrowing spread.
    function MAX_BORROWING_SPREAD() external view returns(uint256);

    /// @return The max borrowing rate.
    function MAX_BORROWING_RATE() external view returns(uint256);

    /// @dev Sets the new borrowing spread.
    /// @param collateralToken Collateral token we set borrowing spread for.
    /// @param newBorrowingSpread New borrowing spread to be used.
    function setBorrowingSpread(IERC20 collateralToken, uint256 newBorrowingSpread) external;

    /// @param collateralToken Collateral token we query borrowing rate for.
    /// @return The current borrowing rate.
    function getBorrowingRate(IERC20 collateralToken) external view returns(uint256);

    /// @param collateralToken Collateral token we query borrowing rate with decay for.
    /// @return The current borrowing rate with decay.
    function getBorrowingRateWithDecay(IERC20 collateralToken) external view returns(uint256);

    /// @dev Returns the borrowing fee for a given debt amount.
    /// @param collateralToken Collateral token we query borrowing fee for.
    /// @param debtAmount The amount of debt.
    /// @return The borrowing fee.
    function getBorrowingFee(IERC20 collateralToken, uint256 debtAmount) external view returns(uint256);

    /// @dev Sets the new redemption spread.
    /// @param newRedemptionSpread New redemption spread to be used.
    function setRedemptionSpread(IERC20 collateralToken, uint256 newRedemptionSpread) external;

    /// @dev Sets new redemption rebate percentage.
    /// @param newRedemptionRebate Value that is being set as a redemption rebate percentage.
    function setRedemptionRebate(IERC20 collateralToken, uint256 newRedemptionRebate) external;

    /// @param collateralToken Collateral token we query redemption rate with decay for.
    /// @return The current redemption rate with decay.
    function getRedemptionRateWithDecay(IERC20 collateralToken) external view returns(uint256);

    /// @dev Returns the redemption fee for a given collateral amount.
    /// @param collateralToken Collateral token we query redemption fee for.
    /// @param collateralAmount The amount of collateral.
    /// @param priceDeviation Deviation for the reported price by oracle in percentage.
    /// @return The redemption fee.
    function getRedemptionFee(
        IERC20 collateralToken,
        uint256 collateralAmount,
        uint256 priceDeviation
    )
    external
    view
    returns(uint256);

    /// @dev Returns the redemption fee with decay for a given collateral amount.
    /// @param collateralToken Collateral token we query redemption fee with decay for.
    /// @param collateralAmount The amount of collateral.
    /// @return The redemption fee with decay.
    function getRedemptionFeeWithDecay(
        IERC20 collateralToken,
        uint256 collateralAmount
    )
    external
    view
    returns(uint256);

    /// @return Half-life of 12h (720 min).
    /// @dev (1/2) = d^720 => d = (1/2)^(1/720)
    function MINUTE_DECAY_FACTOR() external view returns(uint256);

    /// @dev Returns if a given delegate is whitelisted for a given borrower.
    /// @param position The address of the borrower.
    /// @param delegate The address of the delegate.
    /// @return isWhitelisted True if the delegate is whitelisted for a given borrower, false otherwise.
    function isDelegateWhitelisted(address position, address delegate) external view returns(bool isWhitelisted);

    /// @dev Whitelists a delegate.
    /// @param delegate The address of the delegate.
    /// @param whitelisted True if delegate is being whitelisted, false otherwise.
    function whitelistDelegate(address delegate, bool whitelisted) external;

    /// @return Parameter by which to divide the redeemed fraction, in order to calc the new base rate from a
    /// redemption. Corresponds to (1 / ALPHA) in the white paper.
    function BETA() external view returns(uint256);
}

abstract contract PositionManagerDependent is IPositionManagerDependent {
    // --- Immutable variables ---

    address public positionManager;

    // --- Modifiers ---

    modifier onlyPositionManager() {
        if (msg.sender != positionManager) {
            revert CallerIsNotPositionManager(msg.sender);
        }
        _;
    }

    // --- Constructor ---

    constructor(address positionManager_) {
        if (positionManager_ == address(0)) {
            revert PositionManagerCannotBeZero();
        }
        positionManager = positionManager_;
    }
}

/// @dev Interface that particular AMM integrations need to implement in order to be used in OneStepLeverage.
/// Implementation will be used to swap between R and collateralToken.
interface IAMM {
    /// @dev Thrown when the amount received after a swap is below the provided minimum return parameter.
    /// @param amountReceived The amount of tokens received after the swap.
    /// @param minReturn The provided minimum return.
    error InsufficientAmountReceived(uint256 amountReceived, uint256 minReturn);

    /// @dev Thrown when a swap is only partially filled.
    error SwapPartiallyFilled();

    /// @dev Emitted when a swap between two tokens occurs.
    /// @param tokenIn The address of the input token being swapped.
    /// @param tokenOut The address of the output token being swapped.
    /// @param amountIn The amount of input tokens being swapped.
    /// @param amountOut The amount of output tokens being swapped.
    /// @param minReturn The minimum acceptable return for the swap (expressed in `tokenOut`).
    event Swap(
    address indexed tokenIn, address indexed tokenOut, uint256 amountIn, uint256 amountOut, uint256 minReturn
);

/// @dev Swaps `amountIn` of `tokenIn` for `tokenOut`. Fails if returned amount is smaller than `minReturn`.
/// @param tokenIn Address of the token that is being swapped.
/// @param tokenOut Address of the token to swap for.
/// @param amountIn Amount of `tokenIn` being swapped.
/// @param minReturn Minimum amount of `tokenOut` to get as a result of swap.
/// @param extraData Extra data for particular integration with DEX/Aggregator.
/// @return amountOut Actual amount that was returned from swap. Needs to be >= `minReturn`.
function swap(
    IERC20 tokenIn,
    IERC20 tokenOut,
    uint256 amountIn,
    uint256 minReturn,
    bytes calldata extraData
)
external
returns(uint256 amountOut);
}

/// @dev Interface that OneStepLeverage needs to implement
interface IOneStepLeverage is IERC3156FlashBorrower, IPositionManagerDependent {
    // --- Errors ---

    /// @dev AMM cannot be zero address.
    error AmmCannotBeZero();

    /// @dev Collateral token cannot be zero address.
    error CollateralTokenCannotBeZero();

    /// @dev One step leverage supports only R token flash mints.
    error UnsupportedToken();

    /// @dev Flash mint initiator is not One Step Leverage contract.
    error InvalidInitiator();

    /// @dev The provided debtChange cannot be zero.
    error ZeroDebtChange();

    // --- Events ---

    /// @dev Emitted on leveraged position adjustment.
    /// @param position The adjusted position.
    /// @param principalCollateralChange Principal collateral change (collateral added/removed from/to user wallet).
    /// @param principalCollateralIncrease True if principal collateral is added.
    /// @param debtChange The amount of debt added/removed from the position to facilitate the leverage adjustment.
    /// @param isDebtIncrease True if increasing debt/leverage.
    /// @param leveragedCollateralChange The amount of collateral added/removed from position as a result of the
    /// leverage adjustment.
    event LeveragedPositionAdjusted(
    address indexed position,
    uint256 principalCollateralChange,
    bool principalCollateralIncrease,
    uint256 debtChange,
    bool isDebtIncrease,
    uint256 leveragedCollateralChange
);

    // --- Functions ---

    /// @dev Maximum amount of R tokens to be leftover as dust after managing leveraged position call.
    /// In particular some dust can be left after decreasing leverage because swap is done from collateral to R.
    /// Since we specify amount of collateral to swap it will result in >= (flashMintAmount + flashMintFee).
    /// If the result is larger than flash mint repayment amount we will pay it back to user only if it is greater
    /// than `MAX_LEFTOVER_R`.
    function MAX_LEFTOVER_R() external view returns(uint256);

    /// @dev Address of the contract that handles swaps between collateral token and R.
    function amm() external view returns(IAMM);

    /// @dev Collateral token used for leverage.
    function collateralToken() external view returns(IERC20);

    /// @dev Underlying collateral token used for leverage.
    /// @dev It is different to collateralToken in case of WrappedCollateralToken.
    function underlyingCollateralToken() external view returns(IERC20);

    /// @dev Collateral token used for leverage.
    function raftCollateralToken() external view returns(IERC20Indexable);

    /// @dev Collateral token used for leverage.
    function raftDebtToken() external view returns(IERC20Indexable);

    /// @dev Increases or decreases leverage for a position.
    /// @param debtChange Debt being added or removed.
    /// @param isDebtIncrease True if increasing debt/leverage.
    /// @param principalCollateralChange Principal collateral change (collateral added/removed from/to user wallet).
    /// @param principalCollateralIncrease True if principal collateral is added.
    /// @param ammData Additional data to pass to swap method in amm.
    /// @param minReturnOrAmountToSell Serves for two different purposes:
    /// - leverage increase: it is min amount of collateral token to get from swapping flash minted R.
    /// - leverage decrease: it is amount of collateral to swap that will result with enough R to repay flash mint.
    /// @param maxFeePercentage The maximum fee percentage to pay for the position management.
    /// @notice In case of closing position by decreasing debt to zero principalCollIncrease must be false,
    /// and principalCollChange + minReturnOrAmountToSell should be equal to total collateral balance of user.
    function manageLeveragedPosition(
        uint256 debtChange,
        bool isDebtIncrease,
        uint256 principalCollateralChange,
        bool principalCollateralIncrease,
        bytes calldata ammData,
        uint256 minReturnOrAmountToSell,
        uint256 maxFeePercentage
    )
    external;

    /// @dev Rescues tokens from the contract.
    /// @param token Address of the token to rescue.
    /// @param to Address to send the rescued tokens to.
    function rescueTokens(IERC20 token, address to) external;
}

// OpenZeppelin Contracts (last updated v4.8.0) (token/ERC20/ERC20.sol)

// OpenZeppelin Contracts v4.4.1 (token/ERC20/extensions/IERC20Metadata.sol)

/**
 * @dev Interface for the optional metadata functions from the ERC20 standard.
 *
 * _Available since v4.1._
 */
interface IERC20Metadata is IERC20 {
    /**
     * @dev Returns the name of the token.
     */
    function name() external view returns(string memory);

    /**
     * @dev Returns the symbol of the token.
     */
    function symbol() external view returns(string memory);

    /**
     * @dev Returns the decimals places of the token.
     */
    function decimals() external view returns(uint8);
}

/**
 * @dev Implementation of the {IERC20} interface.
 *
 * This implementation is agnostic to the way tokens are created. This means
 * that a supply mechanism has to be added in a derived contract using {_mint}.
 * For a generic mechanism see {ERC20PresetMinterPauser}.
 *
 * TIP: For a detailed writeup see our guide
 * https://forum.openzeppelin.com/t/how-to-implement-erc20-supply-mechanisms/226[How
 * to implement supply mechanisms].
 *
 * We have followed general OpenZeppelin Contracts guidelines: functions revert
 * instead returning `false` on failure. This behavior is nonetheless
 * conventional and does not conflict with the expectations of ERC20
 * applications.
 *
 * Additionally, an {Approval} event is emitted on calls to {transferFrom}.
 * This allows applications to reconstruct the allowance for all accounts just
 * by listening to said events. Other implementations of the EIP may not emit
 * these events, as it isn't required by the specification.
 *
 * Finally, the non-standard {decreaseAllowance} and {increaseAllowance}
 * functions have been added to mitigate the well-known issues around setting
 * allowances. See {IERC20-approve}.
 */
contract ERC20 is Context, IERC20, IERC20Metadata {
    mapping(address => uint256) private _balances;

    mapping(address => mapping(address => uint256)) private _allowances;

    uint256 private _totalSupply;

    string private _name;
    string private _symbol;

    /**
     * @dev Sets the values for {name} and {symbol}.
     *
     * The default value of {decimals} is 18. To select a different value for
     * {decimals} you should overload it.
     *
     * All two of these values are immutable: they can only be set once during
     * construction.
     */
    constructor(string memory name_, string memory symbol_) {
        _name = name_;
        _symbol = symbol_;
    }

    /**
     * @dev Returns the name of the token.
     */
    function name() public view virtual   returns(string memory) {
        return _name;
    }

    /**
     * @dev Returns the symbol of the token, usually a shorter version of the
     * name.
     */
    function symbol() public view virtual   returns(string memory) {
        return _symbol;
    }

    /**
     * @dev Returns the number of decimals used to get its user representation.
     * For example, if `decimals` equals `2`, a balance of `505` tokens should
     * be displayed to a user as `5.05` (`505 / 10 ** 2`).
     *
     * Tokens usually opt for a value of 18, imitating the relationship between
     * Ether and Wei. This is the value {ERC20} uses, unless this function is
     * overridden;
     *
     * NOTE: This information is only used for _display_ purposes: it in
     * no way affects any of the arithmetic of the contract, including
     * {IERC20-balanceOf} and {IERC20-transfer}.
     */
    function decimals() public view virtual   returns(uint8) {
        return 18;
    }

    /**
     * @dev See {IERC20-totalSupply}.
     */
    function totalSupply() public view virtual   returns(uint256) {
        return _totalSupply;
    }

    /**
     * @dev See {IERC20-balanceOf}.
     */
    function balanceOf(address account) public view virtual   returns(uint256) {
        return _balances[account];
    }

    /**
     * @dev See {IERC20-transfer}.
     *
     * Requirements:
     *
     * - `to` cannot be the zero address.
     * - the caller must have a balance of at least `amount`.
     */
    function transfer(address to, uint256 amount) public virtual   returns(bool) {
        address owner = _msgSender();
        _transfer(owner, to, amount);
        return true;
    }

    /**
     * @dev See {IERC20-allowance}.
     */
    function allowance(address owner, address spender) public view virtual   returns(uint256) {
        return _allowances[owner][spender];
    }

    /**
     * @dev See {IERC20-approve}.
     *
     * NOTE: If `amount` is the maximum `uint256`, the allowance is not updated on
     * `transferFrom`. This is semantically equivalent to an infinite approval.
     *
     * Requirements:
     *
     * - `spender` cannot be the zero address.
     */
    function approve(address spender, uint256 amount) public virtual   returns(bool) {
        address owner = _msgSender();
        _approve(owner, spender, amount);
        return true;
    }

    /**
     * @dev See {IERC20-transferFrom}.
     *
     * Emits an {Approval} event indicating the updated allowance. This is not
     * required by the EIP. See the note at the beginning of {ERC20}.
     *
     * NOTE: Does not update the allowance if the current allowance
     * is the maximum `uint256`.
     *
     * Requirements:
     *
     * - `from` and `to` cannot be the zero address.
     * - `from` must have a balance of at least `amount`.
     * - the caller must have allowance for ``from``'s tokens of at least
     * `amount`.
     */
    function transferFrom(
        address from2,
        address to,
        uint256 amount
    ) public virtual   returns(bool) {
        address spender = _msgSender();
        _spendAllowance(from2, spender, amount);
        _transfer(from2, to, amount);
        return true;
    }

    /**
     * @dev Atomically increases the allowance granted to `spender` by the caller.
     *
     * This is an alternative to {approve} that can be used as a mitigation for
     * problems described in {IERC20-approve}.
     *
     * Emits an {Approval} event indicating the updated allowance.
     *
     * Requirements:
     *
     * - `spender` cannot be the zero address.
     */
    function increaseAllowance(address spender, uint256 addedValue) public virtual returns(bool) {
        address owner = _msgSender();
        _approve(owner, spender, allowance(owner, spender) + addedValue);
        return true;
    }

    /**
     * @dev Atomically decreases the allowance granted to `spender` by the caller.
     *
     * This is an alternative to {approve} that can be used as a mitigation for
     * problems described in {IERC20-approve}.
     *
     * Emits an {Approval} event indicating the updated allowance.
     *
     * Requirements:
     *
     * - `spender` cannot be the zero address.
     * - `spender` must have allowance for the caller of at least
     * `subtractedValue`.
     */
    function decreaseAllowance(address spender, uint256 subtractedValue) public virtual returns(bool) {
        address owner = _msgSender();
        uint256 currentAllowance = allowance(owner, spender);
        require(currentAllowance >= subtractedValue, "ERC20: decreased allowance below zero");
        unchecked {
            _approve(owner, spender, currentAllowance - subtractedValue);
        }

        return true;
    }

    /**
     * @dev Moves `amount` of tokens from `from` to `to`.
     *
     * This internal function is equivalent to {transfer}, and can be used to
     * e.g. implement automatic token fees, slashing mechanisms, etc.
     *
     * Emits a {Transfer} event.
     *
     * Requirements:
     *
     * - `from` cannot be the zero address.
     * - `to` cannot be the zero address.
     * - `from` must have a balance of at least `amount`.
     */
    function _transfer(
        address from2,
        address to,
        uint256 amount
    ) internal virtual {
        require(from2 != address(0), "ERC20: transfer from the zero address");
        require(to != address(0), "ERC20: transfer to the zero address");

        _beforeTokenTransfer(from2, to, amount);

        uint256 fromBalance = _balances[from2];
        require(fromBalance >= amount, "ERC20: transfer amount exceeds balance");
        unchecked {
            _balances[from2] = fromBalance - amount;
            // Overflow not possible: the sum of all balances is capped by totalSupply, and the sum is preserved by
            // decrementing then incrementing.
            _balances[to] += amount;
        }

        emit Transfer(from2, to, amount);

        _afterTokenTransfer(from2, to, amount);
    }

    /** @dev Creates `amount` tokens and assigns them to `account`, increasing
     * the total supply.
     *
     * Emits a {Transfer} event with `from` set to the zero address.
     *
     * Requirements:
     *
     * - `account` cannot be the zero address.
     */
    function _mint(address account, uint256 amount) internal virtual {
        require(account != address(0), "ERC20: mint to the zero address");

        _beforeTokenTransfer(address(0), account, amount);

        _totalSupply += amount;
        unchecked {
            // Overflow not possible: balance + amount is at most totalSupply + amount, which is checked above.
            _balances[account] += amount;
        }
        emit Transfer(address(0), account, amount);

        _afterTokenTransfer(address(0), account, amount);
    }

    /**
     * @dev Destroys `amount` tokens from `account`, reducing the
     * total supply.
     *
     * Emits a {Transfer} event with `to` set to the zero address.
     *
     * Requirements:
     *
     * - `account` cannot be the zero address.
     * - `account` must have at least `amount` tokens.
     */
    function _burn(address account, uint256 amount) internal virtual {
        require(account != address(0), "ERC20: burn from the zero address");

        _beforeTokenTransfer(account, address(0), amount);

        uint256 accountBalance = _balances[account];
        require(accountBalance >= amount, "ERC20: burn amount exceeds balance");
        unchecked {
            _balances[account] = accountBalance - amount;
            // Overflow not possible: amount <= accountBalance <= totalSupply.
            _totalSupply -= amount;
        }

        emit Transfer(account, address(0), amount);

        _afterTokenTransfer(account, address(0), amount);
    }

    /**
     * @dev Sets `amount` as the allowance of `spender` over the `owner` s tokens.
     *
     * This internal function is equivalent to `approve`, and can be used to
     * e.g. set automatic allowances for certain subsystems, etc.
     *
     * Emits an {Approval} event.
     *
     * Requirements:
     *
     * - `owner` cannot be the zero address.
     * - `spender` cannot be the zero address.
     */
    function _approve(
        address owner,
        address spender,
        uint256 amount
    ) internal virtual {
        require(owner != address(0), "ERC20: approve from the zero address");
        require(spender != address(0), "ERC20: approve to the zero address");

        _allowances[owner][spender] = amount;
        emit Approval(owner, spender, amount);
    }

    /**
     * @dev Updates `owner` s allowance for `spender` based on spent `amount`.
     *
     * Does not update the allowance amount in case of infinite allowance.
     * Revert if not enough allowance is available.
     *
     * Might emit an {Approval} event.
     */
    function _spendAllowance(
        address owner,
        address spender,
        uint256 amount
    ) internal virtual {
        uint256 currentAllowance = allowance(owner, spender);
        if (currentAllowance != type(uint256).max) {
            require(currentAllowance >= amount, "ERC20: insufficient allowance");
            unchecked {
                _approve(owner, spender, currentAllowance - amount);
            }
        }
    }

    /**
     * @dev Hook that is called before any transfer of tokens. This includes
     * minting and burning.
     *
     * Calling conditions:
     *
     * - when `from` and `to` are both non-zero, `amount` of ``from``'s tokens
     * will be transferred to `to`.
     * - when `from` is zero, `amount` tokens will be minted for `to`.
     * - when `to` is zero, `amount` of ``from``'s tokens will be burned.
     * - `from` and `to` are never both zero.
     *
     * To learn more about hooks, head to xref:ROOT:extending-contracts.adoc#using-hooks[Using Hooks].
     */
    function _beforeTokenTransfer(
        address from2,
        address to,
        uint256 amount
    ) internal virtual { }

    /**
     * @dev Hook that is called after any transfer of tokens. This includes
     * minting and burning.
     *
     * Calling conditions:
     *
     * - when `from` and `to` are both non-zero, `amount` of ``from``'s tokens
     * has been transferred to `to`.
     * - when `from` is zero, `amount` tokens have been minted for `to`.
     * - when `to` is zero, `amount` of ``from``'s tokens have been burned.
     * - `from` and `to` are never both zero.
     *
     * To learn more about hooks, head to xref:ROOT:extending-contracts.adoc#using-hooks[Using Hooks].
     */
    function _afterTokenTransfer(
        address from2,
        address to,
        uint256 amount
    ) internal virtual { }
}

// OpenZeppelin Contracts (last updated v4.6.0) (token/ERC20/extensions/ERC20Wrapper.sol)

/**
 * @dev Extension of the ERC20 token contract to support token wrapping.
 *
 * Users can deposit and withdraw "underlying tokens" and receive a matching number of "wrapped tokens". This is useful
 * in conjunction with other modules. For example, combining this wrapping mechanism with {ERC20Votes} will allow the
 * wrapping of an existing "basic" ERC20 into a governance token.
 *
 * _Available since v4.2._
 */
abstract contract ERC20Wrapper is ERC20 {
    IERC20 public  underlying;

    constructor(IERC20 underlyingToken) {
        underlying = underlyingToken;
    }

    /**
     * @dev See {ERC20-decimals}.
     */
    function decimals() public view virtual   returns(uint8) {
        try IERC20Metadata(address(underlying)).decimals() returns(uint8 value) {
            return value;
        } catch {
            return super.decimals();
        }
    }

    /**
     * @dev Allow a user to deposit underlying tokens and mint the corresponding number of wrapped tokens.
     */
    function depositFor(address account, uint256 amount) public virtual returns(bool) {
        SafeERC20.safeTransferFrom(underlying, _msgSender(), address(this), amount);
        _mint(account, amount);
        return true;
    }

    /**
     * @dev Allow a user to burn a number of wrapped tokens and withdraw the corresponding number of underlying tokens.
     */
    function withdrawTo(address account, uint256 amount) public virtual returns(bool) {
        _burn(_msgSender(), amount);
        SafeERC20.safeTransfer(underlying, account, amount);
        return true;
    }

    /**
     * @dev Mint wrapped token to cover any underlyingTokens that would have been transferred by mistake. Internal
     * function that can be exposed with access control if desired.
     */
    function _recover(address account) internal virtual returns(uint256) {
        uint256 value = underlying.balanceOf(address(this)) - totalSupply();
        _mint(account, value);
        return value;
    }
}

// OpenZeppelin Contracts (last updated v4.8.0) (token/ERC20/extensions/draft-ERC20Permit.sol)

// OpenZeppelin Contracts (last updated v4.8.0) (utils/cryptography/ECDSA.sol)

// OpenZeppelin Contracts (last updated v4.8.0) (utils/Strings.sol)

// OpenZeppelin Contracts (last updated v4.8.0) (utils/math/Math.sol)

/**
 * @dev Standard math utilities missing in the Solidity language.
 */
library Math {
    enum Rounding {
        Down, // Toward negative infinity
        Up, // Toward infinity
        Zero // Toward zero
    }

    /**
     * @dev Returns the largest of two numbers.
     */
    function max(uint256 a, uint256 b) internal pure returns(uint256) {
        return a > b ? a : b;
    }

    /**
     * @dev Returns the smallest of two numbers.
     */
    function min(uint256 a, uint256 b) internal pure returns(uint256) {
        return a < b ? a : b;
    }

    /**
     * @dev Returns the average of two numbers. The result is rounded towards
     * zero.
     */
    function average(uint256 a, uint256 b) internal pure returns(uint256) {
        // (a + b) / 2 can overflow.
        return (a & b) + (a ^ b) / 2;
    }

    /**
     * @dev Returns the ceiling of the division of two numbers.
     *
     * This differs from standard division with `/` in that it rounds up instead
     * of rounding down.
     */
    function ceilDiv(uint256 a, uint256 b) internal pure returns(uint256) {
        // (a + b - 1) / b can overflow on addition, so we distribute.
        return a == 0 ? 0 : (a - 1) / b + 1;
    }

    /**
     * @notice Calculates floor(x * y / denominator) with full precision. Throws if result overflows a uint256 or denominator == 0
     * @dev Original credit to Remco Bloemen under MIT license (https://xn--2-umb.com/21/muldiv)
     * with further edits by Uniswap Labs also under MIT license.
     */
    function mulDiv(
        uint256 x,
        uint256 y,
        uint256 denominator
    ) internal pure returns(uint256 result) {
        unchecked {
            // 512-bit multiply [prod1 prod0] = x * y. Compute the product mod 2^256 and mod 2^256 - 1, then use
            // use the Chinese Remainder Theorem to reconstruct the 512 bit result. The result is stored in two 256
            // variables such that product = prod1 * 2^256 + prod0.
            uint256 prod0; // Least significant 256 bits of the product
            uint256 prod1; // Most significant 256 bits of the product
            assembly {
            }

            // Handle non-overflow cases, 256 by 256 division.
            if (prod1 == 0) {
                return prod0 / denominator;
            }

            // Make sure the result is less than 2^256. Also prevents denominator == 0.
            require(denominator > prod1);

            ///////////////////////////////////////////////
            // 512 by 256 division.
            ///////////////////////////////////////////////

            // Make division exact by subtracting the remainder from [prod1 prod0].
            uint256 remainder;
            assembly {
            }

            // Factor powers of two out of denominator and compute largest power of two divisor of denominator. Always >= 1.
            // See https://cs.stackexchange.com/q/138556/92363.

            // Does not overflow because the denominator cannot be zero at this stage in the function.
            uint256 twos = denominator & (~denominator + 1);
            assembly {
            }

            // Shift in bits from prod1 into prod0.
            prod0 |= prod1 * twos;

            // Invert denominator mod 2^256. Now that denominator is an odd number, it has an inverse modulo 2^256 such
            // that denominator * inv = 1 mod 2^256. Compute the inverse by starting with a seed that is correct for
            // four bits. That is, denominator * inv = 1 mod 2^4.
            uint256 inverse = (3 * denominator) ^ 2;

            // Use the Newton-Raphson iteration to improve the precision. Thanks to Hensel's lifting lemma, this also works
            // in modular arithmetic, doubling the correct bits in each step.
            inverse *= 2 - denominator * inverse; // inverse mod 2^8
            inverse *= 2 - denominator * inverse; // inverse mod 2^16
            inverse *= 2 - denominator * inverse; // inverse mod 2^32
            inverse *= 2 - denominator * inverse; // inverse mod 2^64
            inverse *= 2 - denominator * inverse; // inverse mod 2^128
            inverse *= 2 - denominator * inverse; // inverse mod 2^256

            // Because the division is now exact we can divide by multiplying with the modular inverse of denominator.
            // This will give us the correct result modulo 2^256. Since the preconditions guarantee that the outcome is
            // less than 2^256, this is the final result. We don't need to compute the high bits of the result and prod1
            // is no longer required.
            result = prod0 * inverse;
            return result;
        }
    }

    /**
     * @notice Calculates x * y / denominator with full precision, following the selected rounding direction.
     */
    function mulDiv(
        uint256 x,
        uint256 y,
        uint256 denominator,
        Rounding rounding
    ) internal pure returns(uint256) {
        uint256 result = mulDiv(x, y, denominator);
        if (rounding == Rounding.Up && mulmod(x, y, denominator) > 0) {
            result += 1;
        }
        return result;
    }

    /**
     * @dev Returns the square root of a number. If the number is not a perfect square, the value is rounded down.
     *
     * Inspired by Henry S. Warren, Jr.'s "Hacker's Delight" (Chapter 11).
     */
    function sqrt(uint256 a) internal pure returns(uint256) {
        if (a == 0) {
            return 0;
        }

        // For our first guess, we get the biggest power of 2 which is smaller than the square root of the target.
        //
        // We know that the "msb" (most significant bit) of our target number `a` is a power of 2 such that we have
        // `msb(a) <= a < 2*msb(a)`. This value can be written `msb(a)=2**k` with `k=log2(a)`.
        //
        // This can be rewritten `2**log2(a) <= a < 2**(log2(a) + 1)`
        // → `sqrt(2**k) <= sqrt(a) < sqrt(2**(k+1))`
        // → `2**(k/2) <= sqrt(a) < 2**((k+1)/2) <= 2**(k/2 + 1)`
        //
        // Consequently, `2**(log2(a) / 2)` is a good first approximation of `sqrt(a)` with at least 1 correct bit.
        uint256 result = 1 << (log2(a) >> 1);

        // At this point `result` is an estimation with one bit of precision. We know the true value is a uint128,
        // since it is the square root of a uint256. Newton's method converges quadratically (precision doubles at
        // every iteration). We thus need at most 7 iteration to turn our partial result with one bit of precision
        // into the expected uint128 result.
        unchecked {
            result = (result + a / result) >> 1;
            result = (result + a / result) >> 1;
            result = (result + a / result) >> 1;
            result = (result + a / result) >> 1;
            result = (result + a / result) >> 1;
            result = (result + a / result) >> 1;
            result = (result + a / result) >> 1;
            return min(result, a / result);
        }
    }

    /**
     * @notice Calculates sqrt(a), following the selected rounding direction.
     */
    function sqrt(uint256 a, Rounding rounding) internal pure returns(uint256) {
        unchecked {
            uint256 result = sqrt(a);
            return result + (rounding == Rounding.Up && result * result < a ? 1 : 0);
        }
    }

    /**
     * @dev Return the log in base 2, rounded down, of a positive value.
     * Returns 0 if given 0.
     */
    function log2(uint256 value) internal pure returns(uint256) {
        uint256 result = 0;
        unchecked {
            if (value >> 128 > 0) {
                value >>= 128;
                result += 128;
            }
            if (value >> 64 > 0) {
                value >>= 64;
                result += 64;
            }
            if (value >> 32 > 0) {
                value >>= 32;
                result += 32;
            }
            if (value >> 16 > 0) {
                value >>= 16;
                result += 16;
            }
            if (value >> 8 > 0) {
                value >>= 8;
                result += 8;
            }
            if (value >> 4 > 0) {
                value >>= 4;
                result += 4;
            }
            if (value >> 2 > 0) {
                value >>= 2;
                result += 2;
            }
            if (value >> 1 > 0) {
                result += 1;
            }
        }
        return result;
    }

    /**
     * @dev Return the log in base 2, following the selected rounding direction, of a positive value.
     * Returns 0 if given 0.
     */
    function log2(uint256 value, Rounding rounding) internal pure returns(uint256) {
        unchecked {
            uint256 result = log2(value);
            return result + (rounding == Rounding.Up && 1 << result < value ? 1 : 0);
        }
    }

    /**
     * @dev Return the log in base 10, rounded down, of a positive value.
     * Returns 0 if given 0.
     */
    function log10(uint256 value) internal pure returns(uint256) {
        uint256 result = 0;
        unchecked {
            if (value >= 10 ** 64) {
                value /= 10 ** 64;
                result += 64;
            }
            if (value >= 10 ** 32) {
                value /= 10 ** 32;
                result += 32;
            }
            if (value >= 10 ** 16) {
                value /= 10 ** 16;
                result += 16;
            }
            if (value >= 10 ** 8) {
                value /= 10 ** 8;
                result += 8;
            }
            if (value >= 10 ** 4) {
                value /= 10 ** 4;
                result += 4;
            }
            if (value >= 10 ** 2) {
                value /= 10 ** 2;
                result += 2;
            }
            if (value >= 10 ** 1) {
                result += 1;
            }
        }
        return result;
    }

    /**
     * @dev Return the log in base 10, following the selected rounding direction, of a positive value.
     * Returns 0 if given 0.
     */
    function log10(uint256 value, Rounding rounding) internal pure returns(uint256) {
        unchecked {
            uint256 result = log10(value);
            return result + (rounding == Rounding.Up && 10 ** result < value ? 1 : 0);
        }
    }

    /**
     * @dev Return the log in base 256, rounded down, of a positive value.
     * Returns 0 if given 0.
     *
     * Adding one to the result gives the number of pairs of hex symbols needed to represent `value` as a hex string.
     */
    function log256(uint256 value) internal pure returns(uint256) {
        uint256 result = 0;
        unchecked {
            if (value >> 128 > 0) {
                value >>= 128;
                result += 16;
            }
            if (value >> 64 > 0) {
                value >>= 64;
                result += 8;
            }
            if (value >> 32 > 0) {
                value >>= 32;
                result += 4;
            }
            if (value >> 16 > 0) {
                value >>= 16;
                result += 2;
            }
            if (value >> 8 > 0) {
                result += 1;
            }
        }
        return result;
    }

    /**
     * @dev Return the log in base 10, following the selected rounding direction, of a positive value.
     * Returns 0 if given 0.
     */
    function log256(uint256 value, Rounding rounding) internal pure returns(uint256) {
        unchecked {
            uint256 result = log256(value);
            return result + (rounding == Rounding.Up && 1 << (result * 8) < value ? 1 : 0);
        }
    }
}

/**
 * @dev String operations.
 */
library Strings {
    bytes16 private  _SYMBOLS = "0123456789abcdef";
    uint8 private  _ADDRESS_LENGTH = 20;

    /**
     * @dev Converts a `uint256` to its ASCII `string` decimal representation.
     */
    function toString(uint256 value) internal pure returns(string memory) {
        unchecked {
            uint256 length = Math.log10(value) + 1;
            string memory buffer = new string(length);
            uint256 ptr;
            /// @solidity memory-safe-assembly
            assembly {
            }
            while (true) {
                ptr--;
                /// @solidity memory-safe-assembly
                assembly {
                }
                value /= 10;
                if (value == 0) break;
            }
            return buffer;
        }
    }

    /**
     * @dev Converts a `uint256` to its ASCII `string` hexadecimal representation.
     */
    function toHexString(uint256 value) internal pure returns(string memory) {
        unchecked {
            return toHexString(value, Math.log256(value) + 1);
        }
    }

    /**
     * @dev Converts a `uint256` to its ASCII `string` hexadecimal representation with fixed length.
     */
    function toHexString(uint256 value, uint256 length) internal pure returns(string memory) {
        bytes memory buffer = new bytes(2 * length + 2);
        buffer[0] = "0";
        buffer[1] = "x";
        for (uint256 i = 2 * length + 1; i > 1; --i) {
            buffer[i] = _SYMBOLS[value & 0xf];
            value >>= 4;
        }
        require(value == 0, "Strings: hex length insufficient");
        return string(buffer);
    }

    /**
     * @dev Converts an `address` with fixed length of 20 bytes to its not checksummed ASCII `string` hexadecimal representation.
     */
    function toHexString(address addr) internal pure returns(string memory) {
        return toHexString(uint256(uint160(addr)), _ADDRESS_LENGTH);
    }
}

/**
 * @dev Elliptic Curve Digital Signature Algorithm (ECDSA) operations.
 *
 * These functions can be used to verify that a message was signed by the holder
 * of the private keys of a given address.
 */
library ECDSA {
    enum RecoverError {
        NoError,
        InvalidSignature,
        InvalidSignatureLength,
        InvalidSignatureS,
        InvalidSignatureV // Deprecated in v4.8
    }

    function _throwError(RecoverError error2) private pure {
        if (error2 == RecoverError.NoError) {
            return; // no error2: do nothing
        } else if (error2 == RecoverError.InvalidSignature) {
            revert("ECDSA: invalid signature");
        } else if (error2 == RecoverError.InvalidSignatureLength) {
            revert("ECDSA: invalid signature length");
        } else if (error2 == RecoverError.InvalidSignatureS) {
            revert("ECDSA: invalid signature 's' value");
        }
    }

    /**
     * @dev Returns the address that signed a hashed message (`hash`) with
     * `signature` or error string. This address can then be used for verification purposes.
     *
     * The `ecrecover` EVM opcode allows for malleable (non-unique) signatures:
     * this function rejects them by requiring the `s` value to be in the lower
     * half order, and the `v` value to be either 27 or 28.
     *
     * IMPORTANT: `hash` _must_ be the result of a hash operation for the
     * verification to be secure: it is possible to craft signatures that
     * recover to arbitrary addresses for non-hashed data. A safe way to ensure
     * this is by receiving a hash of the original message (which may otherwise
     * be too long), and then calling {toEthSignedMessageHash} on it.
     *
     * Documentation for signature generation:
     * - with https://web3js.readthedocs.io/en/v1.3.4/web3-eth-accounts.html#sign[Web3.js]
     * - with https://docs.ethers.io/v5/api/signer/#Signer-signMessage[ethers]
     *
     * _Available since v4.3._
     */
    function tryRecover(bytes32 hash, bytes memory signature) internal pure returns(address, RecoverError) {
        if (signature.length == 65) {
            bytes32 r;
            bytes32 s;
            uint8 v;
            // ecrecover takes the signature parameters, and the only way to get them
            // currently is to use assembly.
            /// @solidity memory-safe-assembly
            assembly {
            }
            return tryRecover(hash, v, r, s);
        } else {
            return (address(0), RecoverError.InvalidSignatureLength);
        }
    }

    /**
     * @dev Returns the address that signed a hashed message (`hash`) with
     * `signature`. This address can then be used for verification purposes.
     *
     * The `ecrecover` EVM opcode allows for malleable (non-unique) signatures:
     * this function rejects them by requiring the `s` value to be in the lower
     * half order, and the `v` value to be either 27 or 28.
     *
     * IMPORTANT: `hash` _must_ be the result of a hash operation for the
     * verification to be secure: it is possible to craft signatures that
     * recover to arbitrary addresses for non-hashed data. A safe way to ensure
     * this is by receiving a hash of the original message (which may otherwise
     * be too long), and then calling {toEthSignedMessageHash} on it.
     */
    function recover(bytes32 hash, bytes memory signature) internal pure returns(address) {
        (address recovered, RecoverError error2) = tryRecover(hash, signature);
        _throwError(error2);
        return recovered;
    }

    /**
     * @dev Overload of {ECDSA-tryRecover} that receives the `r` and `vs` short-signature fields separately.
     *
     * See https://eips.ethereum.org/EIPS/eip-2098[EIP-2098 short signatures]
     *
     * _Available since v4.3._
     */
    function tryRecover(
        bytes32 hash,
        bytes32 r,
        bytes32 vs
    ) internal pure returns(address, RecoverError) {
        bytes32 s = vs & bytes32(0x7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff);
        uint8 v = uint8((uint256(vs) >> 255) + 27);
        return tryRecover(hash, v, r, s);
    }

    /**
     * @dev Overload of {ECDSA-recover} that receives the `r and `vs` short-signature fields separately.
     *
     * _Available since v4.2._
     */
    function recover(
        bytes32 hash,
        bytes32 r,
        bytes32 vs
    ) internal pure returns(address) {
        (address recovered, RecoverError error2) = tryRecover(hash, r, vs);
        _throwError(error2);
        return recovered;
    }

    /**
     * @dev Overload of {ECDSA-tryRecover} that receives the `v`,
     * `r` and `s` signature fields separately.
     *
     * _Available since v4.3._
     */
    function tryRecover(
        bytes32 hash,
        uint8 v,
        bytes32 r,
        bytes32 s
    ) internal pure returns(address, RecoverError) {
        // EIP-2 still allows signature malleability for ecrecover(). Remove this possibility and make the signature
        // unique. Appendix F in the Ethereum Yellow paper (https://ethereum.github.io/yellowpaper/paper.pdf), defines
        // the valid range for s in (301): 0 < s < secp256k1n ÷ 2 + 1, and for v in (302): v ∈ {27, 28}. Most
        // signatures from current libraries generate a unique signature with an s-value in the lower half order.
        //
        // If your library generates malleable signatures, such as s-values in the upper range, calculate a new s-value
        // with 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141 - s1 and flip v from 27 to 28 or
        // vice versa. If your library also generates signatures with 0/1 for v instead 27/28, add 27 to v to accept
        // these malleable signatures as well.
        if (uint256(s) > 0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF5D576E7357A4501DDFE92F46681B20A0) {
            return (address(0), RecoverError.InvalidSignatureS);
        }

        // If the signature is valid (and not malleable), return the signer address
        address signer = ecrecover(hash, v, r, s);
        if (signer == address(0)) {
            return (address(0), RecoverError.InvalidSignature);
        }

        return (signer, RecoverError.NoError);
    }

    /**
     * @dev Overload of {ECDSA-recover} that receives the `v`,
     * `r` and `s` signature fields separately.
     */
    function recover(
        bytes32 hash,
        uint8 v,
        bytes32 r,
        bytes32 s
    ) internal pure returns(address) {
        (address recovered, RecoverError error2) = tryRecover(hash, v, r, s);
        _throwError(error2);
        return recovered;
    }

    /**
     * @dev Returns an Ethereum Signed Message, created from a `hash`. This
     * produces hash corresponding to the one signed with the
     * https://eth.wiki/json-rpc/API#eth_sign[`eth_sign`]
     * JSON-RPC method as part of EIP-191.
     *
     * See {recover}.
     */
    function toEthSignedMessageHash(bytes32 hash) internal pure returns(bytes32) {
        // 32 is the length in bytes of hash,
        // enforced by the type signature above
        return keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", hash));
    }

    /**
     * @dev Returns an Ethereum Signed Message, created from `s`. This
     * produces hash corresponding to the one signed with the
     * https://eth.wiki/json-rpc/API#eth_sign[`eth_sign`]
     * JSON-RPC method as part of EIP-191.
     *
     * See {recover}.
     */
    function toEthSignedMessageHash(bytes memory s) internal pure returns(bytes32) {
        return keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n", Strings.toString(s.length), s));
    }

    /**
     * @dev Returns an Ethereum Signed Typed Data, created from a
     * `domainSeparator` and a `structHash`. This produces hash corresponding
     * to the one signed with the
     * https://eips.ethereum.org/EIPS/eip-712[`eth_signTypedData`]
     * JSON-RPC method as part of EIP-712.
     *
     * See {recover}.
     */
    function toTypedDataHash(bytes32 domainSeparator, bytes32 structHash) internal pure returns(bytes32) {
        return keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));
    }
}

// OpenZeppelin Contracts (last updated v4.8.0) (utils/cryptography/EIP712.sol)

/**
 * @dev https://eips.ethereum.org/EIPS/eip-712[EIP 712] is a standard for hashing and signing of typed structured data.
 *
 * The encoding specified in the EIP is very generic, and such a generic implementation in Solidity is not feasible,
 * thus this contract does not implement the encoding itself. Protocols need to implement the type-specific encoding
 * they need in their contracts using a combination of `abi.encode` and `keccak256`.
 *
 * This contract implements the EIP 712 domain separator ({_domainSeparatorV4}) that is used as part of the encoding
 * scheme, and the final step of the encoding to obtain the message digest that is then signed via ECDSA
 * ({_hashTypedDataV4}).
 *
 * The implementation of the domain separator was designed to be as efficient as possible while still properly updating
 * the chain id to protect against replay attacks on an eventual fork of the chain.
 *
 * NOTE: This contract implements the version of the encoding known as "v4", as implemented by the JSON RPC method
 * https://docs.metamask.io/guide/signing-data.html[`eth_signTypedDataV4` in MetaMask].
 *
 * _Available since v3.4._
 */
abstract contract EIP712 {
    /* solhint-disable var-name-mixedcase */
    // Cache the domain separator as an immutable value, but also store the chain id that it corresponds to, in order to
    // invalidate the cached domain separator if the chain id changes.
    bytes32 private  _CACHED_DOMAIN_SEPARATOR;
    uint256 private  _CACHED_CHAIN_ID;
    address private  _CACHED_THIS;

    bytes32 private  _HASHED_NAME;
    bytes32 private  _HASHED_VERSION;
    bytes32 private  _TYPE_HASH;

    /* solhint-enable var-name-mixedcase */

    /**
     * @dev Initializes the domain separator and parameter caches.
     *
     * The meaning of `name` and `version` is specified in
     * https://eips.ethereum.org/EIPS/eip-712#definition-of-domainseparator[EIP 712]:
     *
     * - `name`: the user readable name of the signing domain, i.e. the name of the DApp or the protocol.
     * - `version`: the current major version of the signing domain.
     *
     * NOTE: These parameters cannot be changed except through a xref:learn::upgrading-smart-contracts.adoc[smart
     * contract upgrade].
     */
    constructor(string memory name, string memory version) {
        bytes32 hashedName = keccak256(bytes(name));
        bytes32 hashedVersion = keccak256(bytes(version));
        bytes32 typeHash = keccak256(
        "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"
    );
        _HASHED_NAME = hashedName;
        _HASHED_VERSION = hashedVersion;
        _CACHED_CHAIN_ID = block.chainid;
        _CACHED_DOMAIN_SEPARATOR = _buildDomainSeparator(typeHash, hashedName, hashedVersion);
        _CACHED_THIS = address(this);
        _TYPE_HASH = typeHash;
    }

    /**
     * @dev Returns the domain separator for the current chain.
     */
    function _domainSeparatorV4() internal view returns(bytes32) {
        if (address(this) == _CACHED_THIS && block.chainid == _CACHED_CHAIN_ID) {
            return _CACHED_DOMAIN_SEPARATOR;
        } else {
            return _buildDomainSeparator(_TYPE_HASH, _HASHED_NAME, _HASHED_VERSION);
        }
    }

    function _buildDomainSeparator(
        bytes32 typeHash,
        bytes32 nameHash,
        bytes32 versionHash
    ) private view returns(bytes32) {
        return keccak256(abi.encode(typeHash, nameHash, versionHash, block.chainid, address(this)));
    }

    /**
     * @dev Given an already https://eips.ethereum.org/EIPS/eip-712#definition-of-hashstruct[hashed struct], this
     * function returns the hash of the fully encoded EIP712 message for this domain.
     *
     * This hash can be used together with {ECDSA-recover} to obtain the signer of a message. For example:
     *
     * ```solidity
     * bytes32 digest = _hashTypedDataV4(keccak256(abi.encode(
     *     keccak256("Mail(address to,string contents)"),
     *     mailTo,
     *     keccak256(bytes(mailContents))
     * )));
     * address signer = ECDSA.recover(digest, signature);
     * ```
     */
    function _hashTypedDataV4(bytes32 structHash) internal view virtual returns(bytes32) {
        return ECDSA.toTypedDataHash(_domainSeparatorV4(), structHash);
    }
}

// OpenZeppelin Contracts v4.4.1 (utils/Counters.sol)

/**
 * @title Counters
 * @author Matt Condon (@shrugs)
 * @dev Provides counters that can only be incremented, decremented or reset. This can be used e.g. to track the number
 * of elements in a mapping, issuing ERC721 ids, or counting request ids.
 *
 * Include with `using Counters for Counters.Counter;`
 */
library Counters {
    struct Counter {
        // This variable should never be directly accessed by users of the library: interactions must be restricted to
        // the library's function. As of Solidity v0.5.2, this cannot be enforced, though there is a proposal to add
        // this feature: see https://github.com/ethereum/solidity/issues/4637
        uint256 _value; // default: 0
    }

    function current(Counter storage counter) internal view returns(uint256) {
        return counter._value;
    }

    function increment(Counter storage counter) internal {
        unchecked {
            counter._value += 1;
        }
    }

    function decrement(Counter storage counter) internal {
        uint256 value = counter._value;
        require(value > 0, "Counter: decrement overflow");
        unchecked {
            counter._value = value - 1;
        }
    }

    function reset(Counter storage counter) internal {
        counter._value = 0;
    }
}

/**
 * @dev Implementation of the ERC20 Permit extension allowing approvals to be made via signatures, as defined in
 * https://eips.ethereum.org/EIPS/eip-2612[EIP-2612].
 *
 * Adds the {permit} method, which can be used to change an account's ERC20 allowance (see {IERC20-allowance}) by
 * presenting a message signed by the account. By not relying on `{IERC20-approve}`, the token holder account doesn't
 * need to send a transaction, and thus is not required to hold Ether at all.
 *
 * _Available since v3.4._
 */
abstract contract ERC20Permit is ERC20, IERC20Permit, EIP712 {
    using Counters for Counters.Counter;

        mapping(address => Counters.Counter) private _nonces;

    // solhint-disable-next-line var-name-mixedcase
    bytes32 private  _PERMIT_TYPEHASH =
        keccak256("Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)");
    /**
     * @dev In previous versions `_PERMIT_TYPEHASH` was declared as `immutable`.
     * However, to ensure consistency with the upgradeable transpiler, we will continue
     * to reserve a slot.
     * @custom:oz-renamed-from _PERMIT_TYPEHASH
     */
    // solhint-disable-next-line var-name-mixedcase
    bytes32 private _PERMIT_TYPEHASH_DEPRECATED_SLOT;

    /**
     * @dev Initializes the {EIP712} domain separator using the `name` parameter, and setting `version` to `"1"`.
     *
     * It's a good idea to use the same `name` that is defined as the ERC20 token name.
     */
    constructor(string memory name) EIP712(name, "1") { }

    /**
     * @dev See {IERC20Permit-permit}.
     */
    function permit(
        address owner,
        address spender,
        uint256 value,
        uint256 deadline,
        uint8 v,
        bytes32 r,
        bytes32 s
    ) public virtual  {
        require(block.timestamp <= deadline, "ERC20Permit: expired deadline");

        bytes32 structHash = keccak256(abi.encode(_PERMIT_TYPEHASH, owner, spender, value, _useNonce(owner), deadline));

        bytes32 hash = _hashTypedDataV4(structHash);

        address signer = ECDSA.recover(hash, v, r, s);
        require(signer == owner, "ERC20Permit: invalid signature");

        _approve(owner, spender, value);
    }

    /**
     * @dev See {IERC20Permit-nonces}.
     */
    function nonces(address owner) public view virtual  returns(uint256) {
        return _nonces[owner].current();
    }

    /**
     * @dev See {IERC20Permit-DOMAIN_SEPARATOR}.
     */
    // solhint-disable-next-line func-name-mixedcase
    function DOMAIN_SEPARATOR() external view  returns(bytes32) {
        return _domainSeparatorV4();
    }

    /**
     * @dev "Consume a nonce": return the current value and increment.
     *
     * _Available since v4.1._
     */
    function _useNonce(address owner) internal virtual returns(uint256 current) {
        Counters.Counter storage nonce = _nonces[owner];
        current = nonce.current();
        nonce.increment();
    }
}

interface IWrappedCollateralToken is IERC20, IERC20Permit {
    /// @dev User's balance cannot be larger than `maxBalance`.
    error ExceedsMaxBalance();

    /// @dev Total supply of the token cannot exceed `cap`.
    error ExceedsCap();

    /// @dev Invalid address for whitelist.
    error InvalidWhitelistAddress();

    /// @dev Address is not whitelisted.
    error AddressIsNotWhitelisted(address);

    /// @dev Method not supported.
    error Unsupported();

    /// @dev New `maxBalance` value is set.
    /// @param maxBalance_ Maximum balance of single user.
    event MaxBalanceSet(uint256 maxBalance_);

    /// @dev New `cap` value is set.
    /// @param cap_ Maximum total supply of the token.
    event CapSet(uint256 cap_);

    /// @dev Address is whitelisted or unwhitelisted.
    /// @param addressForWhitelist Address to whitelist or unwhitelist.
    /// @param whitelisted True if address is whitelisted, false if unwhitelisted.
    event AddressWhitelisted(address indexed addressForWhitelist, bool whitelisted);

    /// @dev Maximum balance of single user.
    function maxBalance() external view returns(uint256);

    /// @dev Maximum total supply of the token.
    function cap() external view returns(uint256);

    /// @dev Checks if address is whitelisted.
    /// @param _addressToCheck Address to check is whitelisted.
    function isWhitelisted(address _addressToCheck) external view returns(bool);

    /// @dev Sets new maximum balance of a user.
    function setMaxBalance(uint256 newMaxBalance) external;

    /// @dev Sets new cap for the token.
    function setCap(uint256 newCap) external;

    /// @dev Whitelist or unwhitelist address.
    /// @param addressForWhitelist Address to whitelist or unwhitelist.
    /// @param whitelisted True if address is whitelisted, false if unwhitelisted.
    function whitelistAddress(address addressForWhitelist, bool whitelisted) external;

    /// @dev Mint wrapped token to cover any underlyingTokens that would have been transferred by mistake.
    /// @param account Address to mint wrapped tokens to.
    function recover(address account) external returns(uint256);

    /// @dev Deposits underlying tokens on behalf of user.
    /// @param to Address to receive minted wrapped tokens.
    /// @param accountToCheck Address of the user which token balances we need to check.
    /// @param amount Amount of underlying being deposited.
    function depositForWithAccountCheck(address to, address accountToCheck, uint256 amount) external returns(bool);
}

contract WrappedCollateralToken is
IWrappedCollateralToken,
    ERC20Wrapper,
    ERC20Permit,
    Ownable2Step,
    PositionManagerDependent
{
    // --- Variables ---

    uint256 public  maxBalance;

    uint256 public  cap;

    mapping(address whitelistAddress => bool isWhitelisted) public  isWhitelisted;

    constructor(
        IERC20 underlying_,
        string memory name_,
        string memory symbol_,
        uint256 maxBalance_,
        uint256 cap_,
        address positionManager_
    )
    ERC20(name_, symbol_)
    ERC20Wrapper(underlying_)
    ERC20Permit(name_)
    PositionManagerDependent(positionManager_)
    {
        setMaxBalance(maxBalance_);
        setCap(cap_);
    }

    modifier checkLimits(address accountToCheck, uint256 amount) {
        if (totalSupply() + amount > cap) {
            revert ExceedsCap();
        }
        uint256 raftColBalance = IPositionManager(positionManager).raftCollateralToken(this).balanceOf(accountToCheck);
        if ((balanceOf(accountToCheck) + raftColBalance + amount) > maxBalance) {
            revert ExceedsMaxBalance();
        }

        _;
    }

    modifier checkWhitelist() {
        if (!isWhitelisted[msg.sender]) {
            revert AddressIsNotWhitelisted(msg.sender);
        }
        _;
    }

    function setMaxBalance(uint256 newMaxBalance) public   onlyOwner {
        maxBalance = newMaxBalance;
        emit MaxBalanceSet(newMaxBalance);
    }

    function setCap(uint256 newCap) public   onlyOwner {
        cap = newCap;
        emit CapSet(newCap);
    }

    function whitelistAddress(address addressForWhitelist, bool whitelisted) external   onlyOwner {
        if (addressForWhitelist == address(0)) {
            revert InvalidWhitelistAddress();
        }
        isWhitelisted[addressForWhitelist] = whitelisted;

        emit AddressWhitelisted(addressForWhitelist, whitelisted);
    }

    function decimals() public view virtual returns(uint8) {
        return ERC20Wrapper.decimals();
    }

    function recover(address account) external   onlyOwner returns(uint256) {
        return _recover(account);
    }

    function depositFor(address, uint256) public virtual   returns(bool) {
        revert Unsupported();
    }

    function depositForWithAccountCheck(
        address to,
        address accountToCheck,
        uint256 amount
    )
    external
     
    checkWhitelist
    checkLimits(accountToCheck, amount)
    returns(bool)
    {
        return super.depositFor(to, amount);
    }

    function _transfer(address from2, address to, uint256 amount) internal   onlyPositionManager {
        super._transfer(from2, to, amount);
    }
}

contract OneStepLeverage is IOneStepLeverage, PositionManagerDependent, Ownable2Step {
    using SafeERC20 for IERC20;

        IAMM public    amm;
    IERC20 public    collateralToken;
    IERC20 public    underlyingCollateralToken;
    IERC20Indexable public    raftDebtToken;
    IERC20Indexable public    raftCollateralToken;

    uint256 public   MAX_LEFTOVER_R = 1e18;

    constructor(
        IPositionManager positionManager_,
        IAMM amm_,
        IERC20 collateralToken_,
        bool isWrappedCollateralToken
    )
    PositionManagerDependent(address(positionManager_))
    {
        if (address(amm_) == address(0)) {
            revert AmmCannotBeZero();
        }
        if (address(collateralToken_) == address(0)) {
            revert CollateralTokenCannotBeZero();
        }

        amm = amm_;
        collateralToken = collateralToken_;
        underlyingCollateralToken = isWrappedCollateralToken
            ? WrappedCollateralToken(address(collateralToken_)).underlying()
            : collateralToken_;
        raftCollateralToken = positionManager_.raftCollateralToken(collateralToken);
        raftDebtToken = positionManager_.raftDebtToken(collateralToken);

        // We approve tokens here so we do not need to do approvals in particular actions.
        // Approved contracts are known, so this should be considered as safe.

        // No need to use safeApprove, IRToken is known token and is safe.
        positionManager_.rToken().approve(address(amm), type(uint256).max);
        positionManager_.rToken().approve(address(positionManager_.rToken()), type(uint256).max);
        underlyingCollateralToken.safeApprove(address(amm), type(uint256).max);
        collateralToken_.safeApprove(address(positionManager_), type(uint256).max);
        if (isWrappedCollateralToken) {
            underlyingCollateralToken.safeApprove(address(collateralToken_), type(uint256).max);
        }
    }

    function manageLeveragedPosition(
        uint256 debtChange,
        bool isDebtIncrease,
        uint256 principalCollateralChange,
        bool principalCollateralIncrease,
        bytes calldata ammData,
        uint256 minReturnOrAmountToSell,
        uint256 maxFeePercentage
    )
    external
     
    {
        if (principalCollateralIncrease && principalCollateralChange > 0) {
            underlyingCollateralToken.safeTransferFrom(msg.sender, address(this), principalCollateralChange);
        }

        _manageLeveragedPosition(
            debtChange,
            isDebtIncrease,
            principalCollateralChange,
            principalCollateralIncrease,
            ammData,
            minReturnOrAmountToSell,
            maxFeePercentage,
            true
        );
    }

    function _manageLeveragedPosition(
        uint256 debtChange,
        bool isDebtIncrease,
        uint256 principalCollateralChange,
        bool principalCollateralIncrease,
        bytes calldata ammData,
        uint256 minReturnOrAmountToSell,
        uint256 maxFeePercentage,
        bool releasePrincipals
    )
    internal
    returns(uint256 actualCollateralChange)
    {
        if (debtChange == 0) {
            revert ZeroDebtChange();
        }

        bool fullRepayment;
        if (!isDebtIncrease) {
            uint256 positionDebt = raftDebtToken.balanceOf(msg.sender);
            if (debtChange == type(uint256).max) {
                debtChange = positionDebt;
            }
            fullRepayment = (debtChange == positionDebt);

            actualCollateralChange =
                fullRepayment ? raftCollateralToken.balanceOf(msg.sender) : principalCollateralChange;
        } else {
            actualCollateralChange = principalCollateralChange;
        }

        bytes memory data = abi.encode(
            msg.sender,
            principalCollateralChange,
            principalCollateralIncrease,
            isDebtIncrease,
            ammData,
            minReturnOrAmountToSell,
            maxFeePercentage,
            releasePrincipals,
            fullRepayment,
            actualCollateralChange
        );

        IRToken rToken = IPositionManager(positionManager).rToken();
        rToken.flashLoan(this, address(rToken), debtChange, data);
    }

    function onFlashLoan(
        address initiator,
        address,
        uint256 amount,
        uint256 fee,
        bytes calldata data
    )
    external
     
    returns(bytes32)
    {
        IERC20 rToken = IPositionManager(positionManager).rToken();
        if (msg.sender != address(rToken)) {
            revert UnsupportedToken();
        }
        if (initiator != address(this)) {
            revert InvalidInitiator();
        }

        (
            address user,
                uint256 principalCollateralChange,
                    bool principalCollateralIncrease,
                        bool isDebtIncrease,
                            bytes memory ammData,
                                uint256 minReturnOrAmountToSell,
                                    uint256 maxFeePercentage,
                                        bool releasePrincipals,
                                            bool fullRepayment,
                                                uint256 actualCollateralChange
        ) = abi.decode(data, (address, uint256, bool, bool, bytes, uint256, uint256, bool, bool, uint256));

        uint256 leveragedCollateralChange = isDebtIncrease
            ? amm.swap(rToken, underlyingCollateralToken, amount, minReturnOrAmountToSell, ammData)
            : minReturnOrAmountToSell;

        uint256 collateralChange;
        bool increaseCollateral;
        if (principalCollateralIncrease != isDebtIncrease) {
            collateralChange = principalCollateralChange > leveragedCollateralChange
                ? principalCollateralChange - leveragedCollateralChange
                : leveragedCollateralChange - principalCollateralChange;

            increaseCollateral = principalCollateralIncrease && !isDebtIncrease
                ? principalCollateralChange > leveragedCollateralChange
                : leveragedCollateralChange > principalCollateralChange;
        } else {
            increaseCollateral = principalCollateralIncrease;
            collateralChange = principalCollateralChange + (fullRepayment ? 0 : leveragedCollateralChange);
        }

        if (increaseCollateral) {
            _wrapCollateralTokens(user, collateralChange);
        }

        ERC20PermitSignature memory emptySignature;
        IPositionManager(positionManager).managePosition(
            collateralToken,
            user,
            collateralChange,
            increaseCollateral,
            amount + (isDebtIncrease ? fee : 0),
            isDebtIncrease,
            maxFeePercentage,
            emptySignature
        );

        if (releasePrincipals && !principalCollateralIncrease && actualCollateralChange > 0) {
            _transferCollateralOut(user, actualCollateralChange - (fullRepayment ? minReturnOrAmountToSell : 0));
        }
        if (!isDebtIncrease) {
            uint256 repayAmount = amount + fee;
            _unwrapCollateralTokens(leveragedCollateralChange);
            uint256 amountOut =
                amm.swap(underlyingCollateralToken, rToken, leveragedCollateralChange, repayAmount, ammData);
            if (amountOut > repayAmount + MAX_LEFTOVER_R) {
                // No need to use safeTransfer as rToken is known
                rToken.transfer(user, amountOut - repayAmount);
            }
        }

        emit LeveragedPositionAdjusted(
            user,
            principalCollateralChange,
            principalCollateralIncrease,
            amount,
            isDebtIncrease,
            leveragedCollateralChange
        );
        return keccak256("ERC3156FlashBorrower.onFlashLoan");
    }

    function rescueTokens(IERC20 token, address to) external   onlyOwner {
        if (token == collateralToken) {
            uint256 balance = token.balanceOf(address(this));
            if (balance > 0) {
                _unwrapCollateralTokens(balance);
            }
            token = underlyingCollateralToken;
        }
        token.safeTransfer(to, token.balanceOf(address(this)));
    }

    function _wrapCollateralTokens(address user, uint256 amount) internal {
        if (collateralToken != underlyingCollateralToken) {
            WrappedCollateralToken(address(collateralToken)).depositForWithAccountCheck(address(this), user, amount);
        }
    }

    function _unwrapCollateralTokens(uint256 amount) internal {
        if (collateralToken != underlyingCollateralToken) {
            WrappedCollateralToken(address(collateralToken)).withdrawTo(address(this), amount);
        }
    }

    function _transferCollateralOut(address user, uint256 amount) internal {
        if (collateralToken != underlyingCollateralToken) {
            WrappedCollateralToken(address(collateralToken)).withdrawTo(user, amount);
        } else {
            collateralToken.safeTransfer(user, amount);
        }
    }
}
