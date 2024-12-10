// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/utils/introspection/ERC165Checker.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

import "./interface/IAddresses.sol";
import "./interface/IEpoch.sol";
import "./interface/IProver.sol";
import "./interface/IVerifier.sol";

/// @notice Manage all registered provers
contract Prover is Initializable, OwnableUpgradeable, IProver {
    using ERC165Checker for address;

    /// @notice Unit struct for number change
    struct ProverWork {
        /// @notice Current value
        uint256 value;
        /// @notice Next epoch value
        uint256 newValue;
        /// @notice Next epoch height
        uint256 newEpoch;
    }

    /// @notice Unit struct for prover verifier
    struct ProverVerifier {
        /// @notice Current verifier
        address value;
        /// @notice Next epoch verifier
        address newValue;
        /// @notice Next epoch height
        uint256 newEpoch;
    }

    /// @notice GameProver struct
    struct GameProver {
        /// @notice Prover status, include: Reviewing, Working, Upgrading, Stopped
        ProverStatus status;
        /// @notice Prover type, include: ZK, Z4, ZKVM
        ProverType ptype;
        /// @notice The prover owner account
        address owner;
        /// @notice Current & future work status
        ProverWork work;
        /// @notice Current & future version status
        ProverWork version;
        /// @notice Current & future overtime status
        ProverWork overtime;
        /// @notice Current & future verifier
        ProverVerifier verifier;
        /// @notice The prover is minable, control by prover DAO
        bool minable;
    }

    /// @notice Common Addresses contract
    address addresses;

    /// @notice Current & future total prover work
    ProverWork private proversTotalWork;

    /// @notice Store all prover list
    mapping(address => GameProver) private provers;

    /// @notice Emit when new prover register and waiting reviewing
    event RegisterProver(address prover, ProverType ptype, uint256 work, uint256 version, uint256 overtime, address verifier, string name);

    /// @notice Emit when prover owner transfer to others
    event TransferProver(address prover, address owner);

    /// @notice Emit when the prover start upgrading and waiting reviewing, before approve, it will still use old info
    event UpgradeProver(address prover, ProverType ptype, uint256 work, uint256 version, uint256 overtime, address verifier, string name);

    /// @notice Emit when the prover is approved or reject
    event ApproveProver(address prover, ProverType ptype, uint256 work, uint256 total, uint256 epoch, uint256 version, uint256 overtime, address verifier, bool minable, bool approved);

    /// @notice Emit when the prover is stopped
    event StopProver(address prover);

    /// @notice Initialize
    /// @param _addresses the Addresses contract
    function initialize(address _addresses) public initializer {
        __Ownable_init(msg.sender);
        addresses = _addresses;
    }

    /// @notice Set the Addresses contract
    /// @param _addresses the Addresses contract
    function setAddresses(address _addresses) external onlyOwner {
        addresses = _addresses;
    }

    /// @notice Register new prover to market, the sender is prover owner, and waiting review
    /// @param prover the prover contract(or not) address (unique identifier)
    /// @param _ptype the prover type
    /// @param _work the prover pozk work, calculation based on zk scheme and circuit size
    /// @param _version the prover prover version
    /// @param _overtime the limit time when doing zkp, if overflow the time, others miner can accept the task again
    /// @param _verifier the verifier contract, uses the IVerifier interface
    function register(address prover, ProverType _ptype, uint256 _work, uint256 _version, uint256 _overtime, address _verifier) external {
        require(provers[prover].version.value == 0 && _version > 0, "G01");
        require(_verifier.supportsInterface(type(IVerifier).interfaceId), "G04");
        string memory name = IVerifier(_verifier).name();

        GameProver storage g = provers[prover];
        g.status = ProverStatus.Reviewing;
        g.ptype = _ptype;
        g.owner = msg.sender;
        g.work = ProverWork(0, _work, 0);
        g.version = ProverWork(_version, _version, 0);
        g.overtime = ProverWork(_overtime, _overtime, 0);
        g.verifier = ProverVerifier(_verifier, _verifier, 0);
        g.minable = false;

        emit RegisterProver(prover, _ptype, _work, _version, _overtime, _verifier, name);
        emit TransferProver(prover, msg.sender);
    }

    /// @notice Prover owner can unregister the prover and cannot register same prover address again
    /// @param prover the prover address
    function unregister(address prover) external {
        require(provers[prover].owner == msg.sender, "G02");

        provers[prover].status = ProverStatus.Stopped;

        emit StopProver(prover);
    }

    /// @notice Prover owner can upgrade the prover to new verison and new info
    /// @param prover the prover
    /// @param _ptype the prover type
    /// @param _work the prover next pozk work, calculation based on zk scheme and circuit size
    /// @param _version the prover next prover version
    /// @param _overtime the limit time when doing zkp, if overflow the time, others miner can accept the task again
    /// @param _verifier the next verifier contract, uses the IVerifier interface
    function upgrade(address prover, ProverType _ptype, uint256 _work, uint256 _version, uint256 _overtime, address _verifier) external {
        require(provers[prover].owner == msg.sender, "G02");
        require(_verifier.supportsInterface(type(IVerifier).interfaceId), "G04");
        string memory name = IVerifier(_verifier).name();

        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).getAndUpdate();

        GameProver storage g = provers[prover];
        if (g.status == ProverStatus.Working || g.status == ProverStatus.Upgrading) {
            g.status = ProverStatus.Upgrading;
        } else {
            g.status = ProverStatus.Reviewing;
        }

        // update work
        if (currentEpoch >= g.work.newEpoch) {
            g.work.value = g.work.newValue;
        }
        g.work.newValue = _work;
        g.work.newEpoch = type(uint256).max;

        // update version
        if (currentEpoch >= g.version.newEpoch) {
            g.version.value = g.version.newValue;
        }
        g.version.newValue = _version;
        g.version.newEpoch = type(uint256).max;

        // update overtime
        if (currentEpoch >= g.overtime.newEpoch) {
            g.overtime.value = g.overtime.newValue;
        }
        g.overtime.newValue = _overtime;
        g.overtime.newEpoch = type(uint256).max;

        // update verifier
        if (currentEpoch >= g.verifier.newEpoch) {
            g.verifier.value = g.verifier.newValue;
        }
        g.verifier.newValue = _verifier;
        g.verifier.newEpoch = type(uint256).max;
        g.ptype = _ptype;

        emit UpgradeProver(prover, _ptype, _work, _version, _overtime, _verifier, name);
    }

    /// @notice Prover owner can transfer ownership to others
    /// @param prover the prover
    /// @param _owner the new owner account
    function transferProverOwner(address prover, address _owner) external {
        require(provers[prover].owner == msg.sender, "G02");

        provers[prover].owner = _owner;

        emit TransferProver(prover, _owner);
    }

    /// @notice DAO can approve or reject the prover register and upgrade, if approve, it will works in next epoch
    /// @param prover the prover
    /// @param minable if the prover is minable, that means when create/accept the prover task, will get reward from network
    /// @param approved approve or reject
    function approve(address prover, bool minable, bool approved) external {
        IEpoch e = IEpoch(IAddresses(addresses).get(Contracts.Epoch));
        require(e.isDao(msg.sender), "E02");

        GameProver storage g = provers[prover];
        require(g.status == ProverStatus.Reviewing || g.status == ProverStatus.Upgrading, "G03");

        uint256 currentEpoch = e.getAndUpdate();

        g.minable  = minable;

        // update work & version
        g.version.newEpoch = currentEpoch;  // version update immediately
        g.overtime.newEpoch = currentEpoch; // overtime update immediately
        g.verifier.newEpoch = currentEpoch; // verifier update immediately
        if (approved) {
            g.status = ProverStatus.Working;
            g.work.newEpoch = currentEpoch + 1; // work need upgrade next epoch

            g.version.value = g.version.newValue;
            g.overtime.value = g.overtime.newValue;
            g.verifier.value = g.verifier.newValue;

            // update proversTotalWork
            if (currentEpoch >= proversTotalWork.newEpoch) {
                proversTotalWork.value = proversTotalWork.newValue;
            }
            bool isAdd = g.work.newValue > g.work.value;
            if (isAdd) {
                proversTotalWork.newValue += g.work.newValue - g.work.value;
            } else {
                proversTotalWork.newValue -= g.work.value - g.work.newValue;
            }
            proversTotalWork.newEpoch = currentEpoch + 1;
        } else {
            // revoke
            g.work.newEpoch = currentEpoch;
            g.work.newValue = g.work.value;

            g.version.newValue = g.version.value;
            g.overtime.newValue = g.overtime.value;
            g.verifier.newValue = g.verifier.value;
        }

        emit ApproveProver(prover, g.ptype, g.work.newValue, proversTotalWork.newValue, g.work.newEpoch, g.version.newValue, g.overtime.newValue, g.verifier.newValue, minable, approved);
    }

    /// @notice DAO can stop a prover
    /// @param prover the prover
    function stop(address prover) external {
        require(IEpoch(IAddresses(addresses).get(Contracts.Epoch)).isDao(msg.sender), "E02");

        provers[prover].status = ProverStatus.Stopped;

        emit StopProver(prover);
    }

    /// @notice Check a prover is working (working or upgrading)
    /// @param prover the prover
    /// @return working or not
    function isProver(address prover) external view returns (bool) {
        return provers[prover].status == ProverStatus.Working || provers[prover].status == ProverStatus.Upgrading;
    }

    /// @notice Get all provers work
    /// @return the work of all provers
    function totalWork() external view returns (uint256) {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).get();

        if (currentEpoch >= proversTotalWork.newEpoch) {
            return proversTotalWork.newValue;
        } else {
            return proversTotalWork.value;
        }
    }

    /// @notice Get a prover work
    /// @param prover the prover
    /// @return the work of the prover
    function work(address prover) external view returns (uint256) {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).get();
        ProverWork storage w = provers[prover].work;

        if (currentEpoch >= w.newEpoch) {
            return w.newValue;
        } else {
            return w.value;
        }
    }

    /// notice Get a prover version
    /// @param prover the prover
    /// @return the version of the prover
    function version(address prover) external view returns (uint256) {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).get();
        ProverWork storage v = provers[prover].version;

        if (currentEpoch >= v.newEpoch) {
            return v.newValue;
        } else {
            return v.value;
        }
    }

    /// notice Get a prover overtime
    /// @param prover the prover
    /// @return the overtime of the prover
    function overtime(address prover) external view returns (uint256) {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).get();
        ProverWork storage v = provers[prover].overtime;

        if (currentEpoch >= v.newEpoch) {
            return v.newValue;
        } else {
            return v.value;
        }
    }

    /// notice Get a prover verifier
    /// @param prover the prover
    /// @return the verifier of the prover
    function verifier(address prover) external view returns (address) {
        uint256 currentEpoch = IEpoch(IAddresses(addresses).get(Contracts.Epoch)).get();
        ProverVerifier storage v = provers[prover].verifier;

        if (currentEpoch >= v.newEpoch) {
            return v.newValue;
        } else {
            return v.value;
        }
    }

    /// notice Get a prover owner
    /// @param prover the prover
    /// @return the owner account of the prover
    function owner(address prover) external view returns (address) {
        return provers[prover].owner;
    }

    /// notice Check the prover is need URL or not
    /// @param prover the prover
    /// @param url the url string
    /// @return the result
    function checkUrl(address prover, string memory url) external view returns (bool) {
        if (provers[prover].ptype != ProverType.Z4) {
            return true;
        }

        bytes memory urlBytes = bytes(url);

        // https://x.x
        if (urlBytes.length < 11) {
            return false;
        }

        // Check if url starts with 'https://'
        if (urlBytes[0] == 'h' &&
            urlBytes[1] == 't' &&
            urlBytes[2] == 't' &&
            urlBytes[3] == 'p' &&
            urlBytes[4] == 's' &&
            urlBytes[5] == ':' &&
            urlBytes[6] == '/' &&
            urlBytes[7] == '/'
           ) {
            return true;
        }

        return false;
    }
}
