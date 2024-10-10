// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// You can also run a script with `npx hardhat run <script>`. If you do that, Hardhat
// will compile your contracts, add the Hardhat Runtime Environment's members to the
// global scope, and execute the script.
const { ethers, upgrades, network } = require("hardhat");
const { attachContract, sleep } = require("./address_utils.js");

async function updateEpochPeriod() {
  const [c, _c] = await attachContract("Epoch");
  // await c.setPeriod(600); // 10min
  await c.getAndUpdate();
}

async function updateMinStaking() {
  const [c, _c] = await attachContract("Stake");
  await c.setMinStakeAmount(1000000000000000000000n); // 1000
}

async function main() {
  await updateEpochPeriod();
  //await updateMinStaking();
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
