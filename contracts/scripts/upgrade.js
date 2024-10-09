// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// You can also run a script with `npx hardhat run <script>`. If you do that, Hardhat
// will compile your contracts, add the Hardhat Runtime Environment's members to the
// global scope, and execute the script.
const { ethers, upgrades, network } = require("hardhat");
const { attachContract, sleep } = require("./address_utils.js");
const { writeFile } = require('fs');

const ONE_TOKEN = 1000000000000000000n;

async function upgradeContractWithProxy(name) {
  const [address, _] = await attachContract(name);
  const Factory = await ethers.getContractFactory(name);
  console.log(`${name} upgrading...`);
  await upgrades.upgradeProxy(address, Factory);
  console.log(`${name} upgraded`);
}

async function deployContractWithProxy(name, params=[]) {
  const block = await ethers.provider.getBlockNumber();
  const Factory = await ethers.getContractFactory(name);
  //  use upgradeable deploy, then contracts can be upgraded success, otherwise will get error about ERC 1967 proxy
  const contract = await upgrades.deployProxy(Factory, params);
  await contract.waitForDeployment();
  const address = await contract.getAddress();
  console.log(`${name} address: ${address}, startBlock: ${block}`);

  if (network.name == "localhost") {
    return [address, 0];
  } else {
    return [address, block];
  }
}

async function redeploy(name, item, params=[]) {
  const [a, _] = await attachContract("Addresses");
  const addresses = await a.getAddress();

  const [s, _s] = await deployContractWithProxy(name, [addresses, ...params]);

  // 0, // Contracts.Token,
  // 1, // Contracts.Vesting,
  // 2, // Contracts.Epoch,
  // 3, // Contracts.Stake,
  // 4, // Contracts.Reward,
  // 5, // Contracts.Prover,
  // 6, // Contracts.Task,
  // 7, // Contracts.Controller
  await a.set(item, s);
}

async function upgrade() {
  //await upgradeContractWithProxy("Vesting");
  // await upgradeContractWithProxy("Stake");

  // await redeploy("Stake", 3, [1000n * ONE_TOKEN]);
}

async function main() {
  await upgrade();
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
