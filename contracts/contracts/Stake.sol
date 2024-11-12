// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

import "./interface/IAddresses.sol";
import "./interface/IEpoch.sol";
import "./interface/IVesting.sol";
import "./interface/IStake.sol";
import "./interface/IProver.sol";
import "./interface/IVerifier.sol";

/// @notice Stake Contract, including player, miner & prover,
/// every change will work in next epoch, and unstake can claim in next epoch
contract Stake is Initializable, OwnableUpgradeable, IStake {
    using SafeERC20 for IERC20;

    /// @notice Unit struct about staking/unstaking
    struct Staking {
        uint256 value;
        uint256 newValue;
        uint256 newEpoch;
    }

    /// @notice Unit struct about staking in a prover
    struct ProverStaking {
        /// @notice Prover self total staking
        Staking proverTotal;
        /// @notice Prover self staking list
        mapping(address => Staking) provers;
        /// @notice Miner total staking
        Staking minerTotal;
        /// @notice Miner staking list
        mapping(address => Staking) miners;
    }

    /// @notice Unit struct about ZK test
    struct ZkTest {
        address payer;
        address miner;
        address prover;
        uint256 amount;
        uint256 overAt;
        bytes publics;
    }

    /// @notice Common Addresses contract
    address addresses;

    /// @notice Miner minStakeAmount
    uint256 public minStakeAmount;

    /// @notice Store all provers staking
    mapping(address => ProverStaking) private proversStaking;

    /// @notice Total players staking
    Staking private playerTotal;

    /// @notice Store all players staking
    mapping(address => Staking) private playersStaking;

    /// @notice Store miners/players unstaking list
    mapping(address => Staking) private unstakings;

    /// @notice Store all miner allowlist
    mapping(address => uint256) public allowlist;

    /// @notice The id of next test, start from 1
    uint256 private nextTestId;

    /// @notice Store all tests for miner in permissioned mode, account => ZK public inputs
    mapping(uint256 => ZkTest) private tests;

    /// @notice Store all tests results
    mapping(bytes32 => uint256) private testsResults;

    /// @notice Store all testing
    mapping(address => mapping(address => uint256)) private testing;

    /// @notice Emit when prover staking change
    event ProverStakeChange(uint256 epoch, address prover, address account, int256 changed, uint256 staking, uint256 total);

    /// @notice Emit when miner staking change
    event MinerStakeChange(uint256 epoch, address prover, address account, int256 changed, uint256 staking, uint256 total);

    /// @notice Emit when player staking change
    event PlayerStakeChange(uint256 epoch, address account, int256 changed, uint256 staking, uint256 total);

    /// @notice Emit when account add unstaking/reward
    event AddUnstaking(uint256 epoch, address account, uint256 amount);

    /// @notice Emit when account claimed the unstaking
    event ClaimUnstaking(address account, uint256 amount);

    /// @notice Emit when add new account to miner allowlist
    event AddAllowlist(address[] accounts, uint256[] amounts);

    /// @notice Emit when add account used the allowlist amounts
    event SubAllowlist(address account, uint256 amount);

    /// @notice Emit when miner need do a test
    event MinerTestRequire(uint256 id, address account, address prover);

    /// @notice Emit when test have been created and start
    event MinerTestCreate(uint256 id, address account, address prover, uint256 overtime, bytes inputs, bytes publics);

    /// @notice Emit when pass the miner test
    event MinerTestSubmit(uint256 id, uint256 submitAt);

    /// @notice Emit when cancel the miner test
    event MinerTestCancel(uint256 id);

    /// @notice Initialize
    /// @param _addresses the Addresses contract
    /// @param _minStakeAmount the minimum value of miner staking
    function initialize(address _addresses, uint256 _minStakeAmount) public initializer {
        __Ownable_init(msg.sender);
        addresses = _addresses;
        minStakeAmount = _minStakeAmount;
        nextTestId = 1;
    }

    /// @notice Set the Addresses contract
    /// @param _addresses the Addresses contract
    function setAddresses(address _addresses) external onlyOwner {
        addresses = _addresses;
    }

    /// @notice Set minimum stake amount
    /// @param _minStakeAmount the minimum value of miner staking
    function setMinStakeAmount(uint256 _minStakeAmount) external onlyOwner {
        minStakeAmount = _minStakeAmount;
    }

    /// @notice Add allowlist accounts (multiple)
    /// @param accounts the accounts
    /// @param amounts the true or false
    function addAllowlist(address[] calldata accounts, uint256[] calldata amounts) external onlyOwner {
        for(uint i = 0; i < accounts.length; i++) {
            allowlist[accounts[i]] += amounts[i];
        }

        emit AddAllowlist(accounts, amounts);
    }

    /// --------------- Prover --------------- ///

    /// @notice Get total prover staking
    /// @param prover the prover address
    /// @return total prover staking amount
    function proverTotalStaking(address prover) external view returns (uint256) {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).get();

        Staking storage st = proversStaking[prover].proverTotal;

        if (currentEpoch >= st.newEpoch) {
            return st.newValue;
        } else {
            return st.value;
        }
    }

    /// @notice Get prover staking by account
    /// @param prover the prover address
    /// @param account the staking account
    /// @return the staking amount of this account
    function proverStaking(address prover, address account) external view returns (uint256) {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).get();
        Staking storage st = proversStaking[prover].provers[account];

        if (currentEpoch >= st.newEpoch) {
            return st.newValue;
        } else {
            return st.value;
        }
    }

    /// @notice Stake by prover self(others)
    /// @param prover the prover address
    /// @param amount new staking amount
    function proverStake(address prover, uint256 amount) external {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).getAndUpdate();

        // transfer from account
        IERC20(IAddresses(addresses).get(Contracts.Token)).safeTransferFrom(msg.sender, address(this), amount);

        ProverStaking storage gs = proversStaking[prover];

        // add to staking
        Staking storage sp = gs.provers[msg.sender];
        if (currentEpoch >= sp.newEpoch) {
            sp.value = sp.newValue;
            sp.newEpoch = currentEpoch + 1;
        }
        sp.newValue += amount;

        // add to total staking
        Staking storage st = gs.proverTotal;
        if (currentEpoch >= st.newEpoch) {
            st.value = st.newValue;
            st.newEpoch = currentEpoch + 1;
        }
        st.newValue += amount;

        emit ProverStakeChange(st.newEpoch, prover, msg.sender, int256(amount), sp.newValue, st.newValue);
    }

    /// @notice Unstake by prover self(others)
    /// @param prover the prover address
    /// @param amount the unstaking amount
    function proverUnstake(address prover, uint256 amount) external {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).getAndUpdate();

        ProverStaking storage gs = proversStaking[prover];
        Staking storage sp = gs.provers[msg.sender];

        // update new staking
        if (currentEpoch >= sp.newEpoch) {
            sp.value = sp.newValue;
            sp.newEpoch = currentEpoch + 1;
        }
        require(sp.newValue >= amount, "S01");

        // append to unstaking
        this.addUnstaking(msg.sender, amount);

        // remove from total staking
        Staking storage st = gs.proverTotal;
        if (currentEpoch >= st.newEpoch) {
            st.value = st.newValue;
            st.newEpoch = currentEpoch + 1;
        }
        st.newValue -= amount;

        emit ProverStakeChange(st.newEpoch, prover, msg.sender, -int256(amount), sp.newValue, st.newValue);
    }

    /// --------------- Miner --------------- ///

    /// @notice Get total miner staking
    /// @param prover the prover address
    /// @return the total miner staking amount
    function minerTotalStaking(address prover) external view returns (uint256) {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).get();
        Staking storage st = proversStaking[prover].minerTotal;

        uint256 minersVesting = IVesting(IAddresses(addresses).get(Contracts.Vesting)).minersTotal();

        if (currentEpoch >= st.newEpoch) {
            return st.newValue + minersVesting;
        } else {
            return st.value + minersVesting;
        }
    }

    /// @notice Get miner staking
    /// @param prover the prover address
    /// @param account miner account
    /// @return the miner staking amount
    function minerStaking(address prover, address account) public view returns (uint256) {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).get();
        Staking storage st = proversStaking[prover].miners[account];

        uint256 minerVesting = IVesting(IAddresses(addresses).get(Contracts.Vesting)).miner(account);

        if (currentEpoch >= st.newEpoch) {
            return st.newValue + minerVesting;
        } else {
            return st.value + minerVesting;
        }
    }

    /// @notice Check account is miner or not
    /// @param prover the prover address
    /// @param account the checking account
    /// @return account is miner or not
    function isMiner(address prover, address account) external view returns (bool) {
        uint256 staking = minerStaking(prover, account);
        return staking >= minStakeAmount;
    }

    /// @notice Stake by miner
    /// @param prover the prover address
    /// @param amount the new staking amount
    function minerStake(address prover, uint256 amount) external {
        minerStakeFor(msg.sender, prover, amount);
    }

    /// @notice Stake by someone for the miner
    /// @param miner the miner address
    /// @param prover the prover address
    /// @param amount the new staking amount
    function minerStakeFor(address miner, address prover, uint256 amount) public {
        // transfer from payer(msg.sender)
        IERC20(IAddresses(addresses).get(Contracts.Token)).safeTransferFrom(msg.sender, address(this), amount);

        IEpoch e = IEpoch(IAddresses(addresses).get(Contracts.Epoch));
        NetworkMode enm = e.networkMode();

        // check network mode & allowlist
        if (enm == NetworkMode.Allowlist) {
            require(allowlist[miner] > 0, "E01");
            allowlist[miner] -= amount;
            emit SubAllowlist(miner, amount);
        } else if (enm == NetworkMode.Permissioned) {
            // check already pass the test
            Staking memory sm = proversStaking[prover].miners[miner];
            if (sm.value == 0 && sm.newValue == 0) {
                require(amount >= minStakeAmount, "S03");

                uint256 testId = testing[prover][miner];
                if (testId == 0) {
                    testId = nextTestId;
                    testing[prover][miner] = testId;
                    nextTestId++;
                }

                // do test
                ZkTest storage test = tests[testId];
                test.payer = msg.sender;
                test.miner = miner;
                test.prover = prover;
                test.amount += amount;

                emit MinerTestRequire(testId, miner, prover);
                return;
            }
        }

        _minerStakeFor(miner, prover, amount);
    }

    /// @notice DAO create the test
    /// @param id the test id
    /// @param inputs the zk input data
    /// @param publics the zk public inputs
    function minerTest(uint256 id, bytes calldata inputs, bytes calldata publics) external {
        require(IEpoch(IAddresses(addresses).get(Contracts.Epoch)).isDao(msg.sender), "E02");

        ZkTest storage test = tests[id];
        require(test.amount != 0, "S96");
        test.publics = publics;

        uint256 overtime = IProver(IAddresses(addresses).get(Contracts.Prover)).overtime(test.prover);
        test.overAt = overtime + block.timestamp;

        emit MinerTestCreate(id, test.miner, test.prover, test.overAt, inputs, publics);
    }

    /// @notice Miner submit the proof of the test
    /// @param id the test id
    /// @param autoNew auto renew the task if over time
    /// @param proof the zk proof
    function minerTestSubmit(uint256 id, bool autoNew, bytes calldata proof) external {
        bytes32 hash = keccak256(proof);
        require(testsResults[hash] == 0, "S97");
        testsResults[hash] = id;

        ZkTest storage test = tests[id];
        require(test.amount != 0, "S96");

        // check overtime
        if (test.overAt < block.timestamp) {
            if (autoNew) {
                emit MinerTestRequire(id, test.miner, test.prover);
                return;
            } else {
                revert("S98");
            }
        }

        // check zk verifier
        address verifier = IProver(IAddresses(addresses).get(Contracts.Prover)).verifier(test.prover);
        require(IVerifier(verifier).verify(test.publics, proof), "S99");

        emit MinerTestSubmit(id, block.timestamp);

        _minerStakeFor(test.miner, test.prover, test.amount);

        delete tests[id];
        delete testing[test.prover][test.miner];
    }

    /// @notice Miner cancel the proof of the test
    /// @param id the test id
    function minerTestCancel(uint256 id) external {
        ZkTest storage test = tests[id];
        require(test.amount != 0 && test.miner == msg.sender, "S96");

        // transfer amount to payer
        IERC20(IAddresses(addresses).get(Contracts.Token)).transfer(test.payer, test.amount);
        delete tests[id];

        emit MinerTestCancel(id);
    }

    /// @notice Stake by someone for the miner (Private)
    /// @param miner the miner address
    /// @param prover the prover address
    /// @param amount the new staking amount
    function _minerStakeFor(address miner, address prover, uint256 amount) private {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).getAndUpdate();

        ProverStaking storage gs = proversStaking[prover];

        // add to staking
        Staking storage sm = gs.miners[miner];
        if (currentEpoch >= sm.newEpoch) {
            sm.value = sm.newValue;
            sm.newEpoch = currentEpoch + 1;
        }
        sm.newValue += amount;
        require(sm.newValue >= minStakeAmount, "S03");

        // add to total staking
        Staking storage st = gs.minerTotal;
        if (currentEpoch >= st.newEpoch) {
            st.value = st.newValue;
            st.newEpoch = currentEpoch + 1;
        }
        st.newValue += amount;

        emit MinerStakeChange(st.newEpoch, prover, miner, int256(amount), sm.newValue, st.newValue);
    }

    /// @notice Unstake by miner
    /// @param prover the prover address
    /// @param amount the unstaking amount
    function minerUnstake(address prover, uint256 amount) external {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).getAndUpdate();

        ProverStaking storage gs = proversStaking[prover];
        Staking storage sm = gs.miners[msg.sender];

        // update new staking
        if (currentEpoch >= sm.newEpoch) {
            sm.value = sm.newValue;
            sm.newEpoch = currentEpoch + 1;
        }
        require(sm.newValue >= amount, "S01");

        // remove from staking
        sm.newValue -= amount;
        require(sm.newValue == 0 || sm.newValue >= minStakeAmount, "S04");

        // append to unstaking
        this.addUnstaking(msg.sender, amount);

        // remove from total staking
        Staking storage st = gs.minerTotal;
        if (currentEpoch >= st.newEpoch) {
            st.value = st.newValue;
            st.newEpoch = currentEpoch + 1;
        }
        st.newValue -= amount;

        emit MinerStakeChange(st.newEpoch, prover, msg.sender, -int256(amount), sm.newValue, st.newValue);
    }

    /// @notice Miner can transfer staking from one prover to another without unclaim lock time
    /// @param from the from prover address
    /// @param to the to prover address
    /// @param amount the staking amount
    function minerTransferStaking(address from, address to, uint256 amount) external {
        address miner = msg.sender;
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).getAndUpdate();

        // sub from staking
        ProverStaking storage gsf = proversStaking[from];
        Staking storage smf = gsf.miners[miner];
        if (currentEpoch >= smf.newEpoch) {
            smf.value = smf.newValue;
            smf.newEpoch = currentEpoch + 1;
        }
        require(smf.newValue >= amount, "S01");

        // remove from staking
        smf.newValue -= amount;
        require(smf.newValue == 0 || smf.newValue >= minStakeAmount, "S04");

        // remove from total staking
        Staking storage stf = gsf.minerTotal;
        if (currentEpoch >= stf.newEpoch) {
            stf.value = stf.newValue;
            stf.newEpoch = currentEpoch + 1;
        }
        stf.newValue -= amount;

        emit MinerStakeChange(stf.newEpoch, from, miner, -int256(amount), smf.newValue, stf.newValue);

        // add to staking
        ProverStaking storage gst = proversStaking[to];
        Staking storage smt = gst.miners[miner];
        if (currentEpoch >= smt.newEpoch) {
            smt.value = smt.newValue;
            smt.newEpoch = currentEpoch + 1;
        }
        smt.newValue += amount;
        require(smt.newValue >= minStakeAmount, "S03");

        // add to total staking
        Staking storage stt = gst.minerTotal;
        if (currentEpoch >= stt.newEpoch) {
            stt.value = stt.newValue;
            stt.newEpoch = currentEpoch + 1;
        }
        stt.newValue += amount;

        emit MinerStakeChange(stt.newEpoch, to, miner, int256(amount), smt.newValue, stt.newValue);
    }

    function minerSlashStaking(address miner, address prover, address player, uint256 amount) external {
        require(msg.sender == IAddresses(addresses).get(Contracts.Task), "S10");

        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).getAndUpdate();

        ProverStaking storage gs = proversStaking[prover];
        Staking storage sm = gs.miners[miner];

        // update new staking
        if (currentEpoch >= sm.newEpoch) {
            sm.value = sm.newValue;
            sm.newEpoch = currentEpoch + 1;
        }

        uint256 slashAmount = amount;
        uint256 unslashAmount = 0;

        if (sm.newValue < amount) {
            slashAmount = sm.newValue;
            unslashAmount = amount - slashAmount;

            Staking storage su = unstakings[miner];
            if (currentEpoch >= su.newEpoch) {
                su.value += su.newValue;
                su.newValue = 0;
                su.newEpoch = currentEpoch + 1;
            }

            if (su.newValue < unslashAmount) {
                uint256 remain = unslashAmount - su.newValue;
                if (su.value < remain) {
                    unslashAmount = su.newValue + su.value;
                    su.value = 0;
                } else {
                    su.value -= remain;
                }

                su.newValue = 0;
            } else {
                su.newValue -= unslashAmount;
            }

            emit ClaimUnstaking(miner, unslashAmount);
        }

        // remove from miner staking
        sm.newValue -= slashAmount;

        // append to player unstaking
        this.addUnstaking(player, slashAmount + unslashAmount);

        // remove from total staking
        Staking storage st = gs.minerTotal;
        if (currentEpoch >= st.newEpoch) {
            st.value = st.newValue;
            st.newEpoch = currentEpoch + 1;
        }
        st.newValue -= slashAmount;

        emit MinerStakeChange(st.newEpoch, prover, miner, -int256(slashAmount), sm.newValue, st.newValue);
    }

    /// --------------- Player --------------- ///

    /// @notice Get total player staking
    /// @return the total staking amount of players
    function playerTotalStaking() external view returns (uint256) {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).get();
        Staking storage st = playerTotal;

        if (currentEpoch >= st.newEpoch) {
            return st.newValue;
        } else {
            return st.value;
        }
    }

    /// @notice Get player staking
    /// @param account the player account
    /// @return the staking amount of player
    function playerStaking(address account) external view returns (uint256) {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).get();
        Staking storage st = playersStaking[account];

        if (currentEpoch >= st.newEpoch) {
            return st.newValue;
        } else {
            return st.value;
        }
    }

    /// @notice Stake by player
    /// @param amount the new staking amount of player
    function playerStake(uint256 amount) external {
        playerStakeFor(msg.sender, amount);
    }

    /// @notice Stake by player
    /// @param player the player address
    /// @param amount the new staking amount of player
    function playerStakeFor(address player, uint256 amount) public {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).getAndUpdate();

        // transfer from account
        IERC20(IAddresses(addresses).get(Contracts.Token)).safeTransferFrom(player, address(this), amount);

        // add to staking
        Staking storage sp = playersStaking[player];
        if (currentEpoch >= sp.newEpoch) {
            sp.value = sp.newValue;
            sp.newEpoch = currentEpoch + 1;
        }
        sp.newValue += amount;

        // add to total staking
        Staking storage st = playerTotal;
        if (currentEpoch >= st.newEpoch) {
            st.value = st.newValue;
            st.newEpoch = currentEpoch + 1;
        }
        st.newValue += amount;

        emit PlayerStakeChange(st.newEpoch, player, int256(amount), sp.newValue, st.newValue);
    }

    /// @notice Unstake by player
    /// @param amount the unstaking amount
    function playerUnstake(uint256 amount) external {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).getAndUpdate();

        Staking storage sp = playersStaking[msg.sender];

        // update new staking
        if (currentEpoch >= sp.newEpoch) {
            sp.value = sp.newValue;
            sp.newEpoch = currentEpoch + 1;
        }
        require(sp.newValue >= amount, "S01");

        // remove from staking
        sp.newValue -= amount;

        // append to unstaking
        this.addUnstaking(msg.sender, amount);

        // remove from total staking
        Staking storage st = playerTotal;
        if (currentEpoch >= st.newEpoch) {
            st.value = st.newValue;
            st.newEpoch = currentEpoch + 1;
        }
        st.newValue -= amount;

        emit PlayerStakeChange(st.newEpoch, msg.sender, -int256(amount), sp.newValue, st.newValue);
    }

    /// --------------- Unstaking --------------- ///

    /// @notice Add new unstaking to next epoch, only this contract and reward contract.
    /// @param account the unstaking account
    /// @param amount the unstaking amount
    function addUnstaking(address account, uint256 amount) external {
        require(msg.sender == address(this) || msg.sender == IAddresses(addresses).get(Contracts.Reward), "S05");
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).getAndUpdate();

        // append to unstaking
        Staking storage su = unstakings[account];
        if (currentEpoch >= su.newEpoch) {
            su.value += su.newValue;
            su.newValue = 0;
            su.newEpoch = currentEpoch + 1;
        }
        su.newValue += amount;

        emit AddUnstaking(currentEpoch + 1, account, amount);
    }

    /// @notice Get claimable unstaking amount
    /// @param account the claiming account
    /// @return the amount which can claim now
    function claimable(address account) external view returns (uint256) {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).get();
        Staking storage su = unstakings[account];

        if (currentEpoch >= su.newEpoch) {
            return su.value + su.newValue;
        } else {
            return su.value;
        }
    }

    /// @notice Claim unstaking to account
    /// @param account the claiming account
    function claim(address account) external {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).getAndUpdate();
        Staking storage su = unstakings[account];

        uint256 amount = su.value;
        if (currentEpoch >= su.newEpoch) {
            amount += su.newValue;
            su.newValue = 0;
            su.newEpoch = currentEpoch + 1;
        }
        su.value = 0;
        require(amount > 0, "S02");

        // transfer amount to account
        IERC20(IAddresses(addresses).get(Contracts.Token)).transfer(account, amount);

        emit ClaimUnstaking(account, amount);
    }
}
