// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// You can also run a script with `npx hardhat run <script>`. If you do that, Hardhat
// will compile your contracts, add the Hardhat Runtime Environment's members to the
// global scope, and execute the script.
const { ethers, upgrades, network } = require("hardhat");
const { attachContract, sleep } = require("./address_utils.js");
const { writeFile, readFileSync } = require('fs');

async function deployContractWithProxy(name, params=[]) {
  const block = await ethers.provider.getBlockNumber();
  const Factory = await ethers.getContractFactory(name);
  //  use upgradeable deploy, then contracts can be upgraded success, otherwise will get error about ERC 1967 proxy
  const contract = await upgrades.deployProxy(Factory, params);
  await contract.waitForDeployment();
  const address = await contract.getAddress();
  console.log(`${name} address: ${address} from ${block}`);

  return [address, block];
}

async function deployContract(name, params=[]) {
  const block = await ethers.provider.getBlockNumber();
  const Factory = await ethers.getContractFactory(name);
  const contract = await Factory.deploy(...params);
  console.log(contract);
  const address = await contract.getAddress();
  console.log(`${name} address: ${address} from ${block}`);

  return [address, block];
}

const ONE_TOKEN = 10000000000000000000n;

async function deployNew() {
  const addresses = await attachContract("Addresses");
  let addr = await addresses.getAddress();

  const vesting = await deployContractWithProxy("Stake", [addr]);

  await addresses.set(3, vesting);
}

async function deploy() {
  const [token, token_s] = await deployContract("Token", [1000000000n * ONE_TOKEN]); // 1,000,000,000 TOEKN
  //const tokenContract = await attachContract("Token");
  //const token = await tokenContract.getAddress();

  const [addresses, addresses_s] = await deployContractWithProxy("Addresses", []);
  const [vesting, vesting_s] = await deployContractWithProxy("Vesting", [addresses, 10000n * ONE_TOKEN]);
  const [epoch, epoch_s] = await deployContractWithProxy("Epoch", [addresses, 100]);
  const [stake, stake_s] = await deployContractWithProxy("Stake", [addresses, 100n * ONE_TOKEN]);
  const [reward, reward_s] = await deployContractWithProxy("Reward", [addresses, 1, 4, 1, 4, 1, 4, 90, 10, 10000]);
  const [prover, prover_s] = await deployContractWithProxy("Prover", [addresses]);
  const [task, task_s] = await deployContractWithProxy("Task", [addresses]);
  const [controller, controller_s] = await deployContractWithProxy("Controller", [addresses]);

  const addressesContract = await ethers.getContractFactory("Addresses");
  const C = await addressesContract.attach(addresses);
  await C.batchSet(
    [
      0, // Contracts.Token,
      1, // Contracts.Vesting,
      2, // Contracts.Epoch,
      3, // Contracts.Stake,
      4, // Contracts.Reward,
      5, // Contracts.Prover,
      6, // Contracts.Task,
      7, // Contracts.Controller
    ],
    [
      token,
      vesting,
      epoch,
      stake,
      reward,
      prover,
      task,
      controller
    ]
  );

  const filename = `../public/networks.json`;
  const filebytes = readFileSync(filename, 'utf8');
  let obj = {};
  if (filebytes) {
    obj = JSON.parse(filebytes);
  }

  obj[network.name] = {
    Addresses: {
      address: addresses,
      startBlock: addresses_s,
    },
    Token: {
      address: token,
      startBlock: token_s,
    },
    Vesting: {
      address: vesting,
      startBlock: vesting_s,
    },
    Epoch: {
      address: epoch,
      startBlock: epoch_s,
    },
    Stake: {
      address: stake,
      startBlock: stake_s,
    },
    Reward: {
      address: reward,
      startBlock: reward_s,
    },
    Prover: {
      address: prover,
      startBlock: prover_s,
    },
    Task: {
      address: task,
      startBlock: task_s,
    },
    Controller: {
      address: controller,
      startBlock: controller_s,
    }
  };

  writeFile(
    filename,
    JSON.stringify(obj, null, 4),
    function(err) {
      if (err) {
        console.log(err);
      }
    });
  console.log(`Save to ${filename}`);
}

async function main() {
  await deploy();
  // await deployNew();
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
