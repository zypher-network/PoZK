// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// You can also run a script with `npx hardhat run <script>`. If you do that, Hardhat
// will compile your contracts, add the Hardhat Runtime Environment's members to the
// global scope, and execute the script.
const { ethers, upgrades, network } = require("hardhat");
const { attachContract, sleep } = require("./address_utils.js");

const ADDRESS = "0x764ae46F345be77eF2a1f707842A9E7Cffb1F2FB";
const WORK = 10000;
const VERSION = 1;
const OVERTIME = 20;

async function main() {
  const c = await attachContract("ProverMarket");
  await c.register(ADDRESS, WORK, VERSION, OVERTIME, ADDRESS);
  await c.approve(ADDRESS, true, true);
  console.log("Prover set ok");
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
