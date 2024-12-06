// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

/// @notice 1:M mode subscription & payment
contract Subscription is Initializable, OwnableUpgradeable {
    /// @notice Miner's task State
    struct State {
        /// @notice latest reported epoch
        uint256 epoch;
        /// @notice latest reported state
        bytes32 state;
    }

    /// @notice PaymentTask struct
    struct Task {
        /// @notice The payer account
        address payer;
        /// @notice The total amount
        uint256 total;
        /// @notice How many miner can join this payment task
        uint256 capacity;
        /// @notice The end time;
        uint256 endAt;
        /// @notice If need public Url
        bool hasUrl;
        /// @notice the joined miners
        address[] miners;
        /// @notice the miners' state
        mapping(address => State) states;
    }

    /// @notice Common Addresses contract
    address addresses;

    /// @notice Next Task id
    uint256 public nextTaskId;

    /// @notice Store all controllers by account
    mapping(uint256 => Task) tasks;

    /// @notice Emit when new Subscription task created
    event CreateSubscription(uint256 id, address payer, uint256 total, uint256 capacity, uint256 endtime);

    event AcceptSubscription(uint256 id, address miner, string url);

    event SettleSubscription(uint256 id, address miner);

    event withdrawSubscription(uint256 id, uint256 amount);

    event DisputeSubscription(uint256 id, address sender);

    event AdjudicateSubscription(uint256 id, address sender, uint256 daoAmount, bool slash);

    /// @notice Initialize
    /// @param _addresses the common Addresses contract
    function initialize(address _addresses) public initializer {
        __Ownable_init(msg.sender);
        addresses = _addresses;
    }

    /// @notice Set common Addresses contract
    function setAddresses(address _addresses) external onlyOwner {
        addresses = _addresses;
    }

    /// @notice payer create a subscription task
    function create(address prover, address payer, uint256 fee, uint256 capacity, uint256 end) external returns(uint256) {
        //
    }

    /// @notice Miner accept the task
    function accept(uint256 tid) external {}

    /// @notice Miner report state & claim rewards
    function settle(uint256 tid, address miner) external {}

    /// @notice Withdraw remain amount of task
    function withdraw(uint256 tid) external {}

    /// @notice Dispute the task, payer & miner can call this, no need deposit
    function dispute(uint256 tid) external {}

    /// @notice Adjudicate the task dispute by DAO
    function adjudicate(uint256 tid, uint256 daoAmount) external {}
}
