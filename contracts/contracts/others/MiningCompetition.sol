// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

import "../interface/IAddresses.sol";
import "../Stake.sol";

enum Status {
    Waiting,
    Working,
    Stopped
}

/// @notice Phases in the network, simulating "block height" in blockchain,
/// stake and reward are effective and issued according to the epoch
contract MiningCompetition is Initializable, OwnableUpgradeable {
    using SafeERC20 for IERC20;

    /// @notice Common Addresses contract
    address addresses;

    /// @notice Competition status
    Status public status;

    address public initProver;
    uint256 public registerReward;
    uint256 public inviteReward;

    address realToken;
    uint256 decimal;

    /// admin list for register account
    mapping(address => bool) public allowlist;

    /// user => user invited rewards & invite link is user's address
    mapping(address => uint256) public users;

    event Register(address user, address inviter, uint256 inviterTotal);

    event Exchange(address user, uint256 amount, uint256 realAmount);

    /// @notice Initialize
    /// @param _addresses the Addresses contract
    function initialize(address _addresses) public initializer {
        __Ownable_init(msg.sender);
        addresses = _addresses;

        status = Status.Waiting;
    }

    /// @notice Set the Addresses contract
    /// @param _addresses the Addresses contract address
    function setAddresses(address _addresses) external onlyOwner {
        addresses = _addresses;
    }

    function changeStatus(Status _status, address _initProver, uint256 _registerReward, uint256 _inviteReward) external onlyOwner {
        status = _status;
        if (status == Status.Working) {
            initProver = _initProver;
            registerReward = _registerReward;
            inviteReward = _inviteReward;

            IERC20 token = IERC20(IAddresses(addresses).get(Contracts.Token));
            address stake = IAddresses(addresses).get(Contracts.Stake);
            token.approve(stake, token.totalSupply());
        }
    }

    function setExchange(address _realToken, uint256 _decimal) external onlyOwner {
        realToken = _realToken;
        decimal = _decimal;
    }

    function allow(address account, bool _allow) external onlyOwner {
        allowlist[account] = _allow;
    }

    function register(address account, address inviter) external {
        require(allowlist[msg.sender] && status == Status.Working, "MC01");
        require(users[account] == 0, "MC02");

        // send min staking amount to account
        Stake stake = Stake(IAddresses(addresses).get(Contracts.Stake));

        uint256 minerReward = stake.minStakeAmount();
        IERC20(IAddresses(addresses).get(Contracts.Token)).safeTransfer(account, minerReward);
        users[account] = minerReward;

        // stake invite reward
        if (users[inviter] > 0) {
            users[inviter] += inviteReward;
            users[account] += inviteReward;

            stake.playerStakeFor(inviter, inviteReward);
            stake.playerStakeFor(account, inviteReward);
        }

        emit Register(account, inviter, users[inviter]);
    }

    function exchange(address account, uint256 amount) external {
        require(realToken != address(0) && decimal != 0, "MC03");

        IERC20(IAddresses(addresses).get(Contracts.Token)).safeTransferFrom(account, address(this), amount);

        // calc real token amount
        uint256 realAmount = amount / decimal;

        IERC20(realToken).safeTransfer(account, realAmount);

        emit Exchange(account, amount, realAmount);
    }
}

