// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

/// @notice Token lock status and unlock period
contract L2Vesting is Initializable, OwnableUpgradeable {
    using SafeERC20 for IERC20;

    /// @notice Vesting plan
    struct Plan {
        /// inital unlock amount
        uint256 initial;
        /// vesting period
        uint256 period;
        /// vesting start date
        uint256 start;
    }

    /// @notice the token address
    address token;

    /// @notice vesting plans
    Plan[] public plans;

    /// @notice store the allocations for user by planId: planId => user => amount
    mapping(uint256 => mapping(address => uint256)) public allocations;

    /// @notice store claimed amount for user by planId: planId => user => amount
    mapping(uint256 => mapping(address => uint256)) public claimed;

    /// @notice Emitted when new plan was created or updated
    event AddPlan(uint256 planId, uint256 initial, uint256 period, uint256 start);

    /// @notice Emitted when new allocation was added to a user by planId
    event Allocated(uint256 planId, address user, uint256 allocation);

    /// @notice Emitted when user claimed vested token
    event Claimed(uint256 planId, address user, uint256 amount);

    /// @notice Initialize
    /// @param _token the Token contract
    function initialize(address _token) public initializer {
        __Ownable_init(msg.sender);
        token = _token;
    }

    /// @notice Set the Token contract
    /// @param _token the Token contract
    function setToken(address _token) external onlyOwner {
        token = _token;
    }

    /// @notice Add new plan
    /// @param initial the initial amount without lock
    /// @param period the vesting period
    /// @param start the start date
    function addPlan(uint256 initial, uint256 period, uint256 start) external onlyOwner {
        plans.push(Plan(initial, period, start));

        emit AddPlan(plans.length - 1, initial, period, start);
    }

    /// @notice Update plan start date
    /// @param planId the plan id
    /// @param period the vesting period
    /// @param start the start date
    function startPlan(uint256 planId, uint256 period, uint256 start) external onlyOwner {
        plans[planId].period = period;
        plans[planId].start = start;

        emit AddPlan(planId, plans[planId].initial, period, start);
    }

    /// @notice Batch set the allocations
    /// @param _planIds the plan id list
    /// @param _users the user list
    /// @param _allocations the allocation list
    function allocate(uint256[] calldata _planIds, address[] calldata _users, uint256[] calldata _allocations) external onlyOwner {
        for (uint256 i = 0; i < _users.length; i++) {
            allocations[_planIds[i]][_users[i]] += _allocations[i];

            emit Allocated(_planIds[i], _users[i], _allocations[i]);
        }
    }

    /// @notice Claim vested amount
    /// @param planId the plan id
    /// @param user the user account
    function claim(uint256 planId, address user) external {
        require(planId < plans.length, 'V013');
        require(allocations[planId][user] != 0, 'V011');

        uint256 amount = claimable(planId, user);
        require(amount > 0, 'V012');

        claimed[planId][user] += amount;

        IERC20(token).transfer(user, amount);

        emit Claimed(planId, user, amount);
    }

    /// @notice Get claimable amount
    /// @param planId the plan id
    /// @param user the user account
    /// @return the amount
    function claimable(uint256 planId, address user) public view returns (uint256) {
        Plan memory plan = plans[planId];

        // check vesting start date and allocation
        if (plan.start == 0 || plan.start > block.timestamp || allocations[planId][user] == 0) {
            return 0;
        }

        uint256 vestedPeriod = block.timestamp - plan.start;

        // check period passed or no period or initial more than allocation
        if (vestedPeriod == 0 || vestedPeriod > plan.period || plan.initial > allocations[planId][user]) {
            return allocations[planId][user] - claimed[planId][user];
        }

        uint256 vesting = allocations[planId][user] - plan.initial;
        return plan.initial + vesting * vestedPeriod / plan.period - claimed[planId][user];
    }

    /// @notice Withdraw all vesting token to admin for suspend
    function withdrawByAdmin() external onlyOwner {
        IERC20 tokenC = IERC20(token);
        uint256 amount = tokenC.balanceOf(address(this));
        tokenC.transfer(msg.sender, amount);
    }
}
