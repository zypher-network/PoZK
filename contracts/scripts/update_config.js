// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// You can also run a script with `npx hardhat run <script>`. If you do that, Hardhat
// will compile your contracts, add the Hardhat Runtime Environment's members to the
// global scope, and execute the script.
const { ethers, upgrades, network } = require("hardhat");
const { attachContract, sleep } = require("./address_utils.js");

async function update_player_limit() {
  let addr = "0x322813Fd9A801c5507c9de605d63CEA4f2CE6c44";
  const c = await attachContract("Prover");
  //await c.register(addr, 10000, 1, 10, addr);
  await c.approve(addr, true, true);
  //await c.setMinerPlayerPer(10, 90, 10000);
}

async function main() {
  await update_player_limit();
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
