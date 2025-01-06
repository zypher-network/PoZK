const { ethers, upgrades, network } = require("hardhat");
const { attachContract, sleep } = require("./address_utils.js");

const ONE_TOKEN = 10000000000000000000n;

// Testnet
const ADDR = "0x0Ff04e9D82314010A1AC539249390470c684A0Dd";
const PROVER = "0x432d35f3717f195070c450f471311a221ef275cd";

async function start() {
  const C = await ethers.getContractFactory("MiningCompetition");
  const c = await C.attach(ADDR);

  // await c.changeStatus(1, PROVER, ONE_TOKEN * 1000n, ONE_TOKEN * 200n);

  // await sleep();
  const status = await c.status();
  const [t, _] = await attachContract("Token");
  const balance = await t.balanceOf(ADDR);
  console.log("Status:", status, balance, balance / ONE_TOKEN);
}

async function stop() {
  const C = await ethers.getContractFactory("MiningCompetition");
  const c = await C.attach(ADDR);

  await c.changeStatus(2, PROVER, 0, 0);

  await sleep();
  const status = await c.status();
  const [t, _] = await attachContract("Token");
  const balance = await t.balanceOf(ADDR);
  console.log("Status:", status, balance, balance / ONE_TOKEN);
}

async function main() {
  await start();
  // await stop();
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
