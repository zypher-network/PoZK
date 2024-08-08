// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// You can also run a script with `npx hardhat run <script>`. If you do that, Hardhat
// will compile your contracts, add the Hardhat Runtime Environment's members to the
// global scope, and execute the script.
const { ethers, upgrades, network } = require("hardhat");
const { attachContract, sleep } = require("./address_utils.js");
const { writeFile } = require('fs');

async function upgradeContractWithProxy(name) {
  const address = await attachContract(name);
  const Factory = await ethers.getContractFactory(name);
  console.log(`${name} upgrading...`);
  await upgrades.upgradeProxy(address, Factory);
  console.log(`${name} upgraded`);
}

async function upgrade() {
  await upgradeContractWithProxy("Vesting");
  await upgradeContractWithProxy("Reward");
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
