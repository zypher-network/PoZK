const { ethers, upgrades, network } = require("hardhat");
const { attachContract, sleep } = require("./address_utils.js");

const ONE_TOKEN = 1000000000000000000n;
const INIT_TOKEN = 1000000000000000000000000n // 1,000,000

// base sepolia
const ADDR = "0x4788a311F560aBd27156637b6fb516599C93b21e";
const PROVER = "0xa7b1abf5b41d42c293917cf8d8bddf760b326d17";

async function status() {
  const C = await ethers.getContractFactory("MiningCompetition");
  const c = await C.attach(ADDR);
  const status = await c.status();

  const [t, _] = await attachContract("Token");
  const balance = await t.balanceOf(ADDR);

  console.log("Status:", status, "Balance:", balance / ONE_TOKEN);
}

async function start() {
  const C = await ethers.getContractFactory("MiningCompetition");
  const c = await C.attach(ADDR);
  await c.changeStatus(1, PROVER, ONE_TOKEN * 1000n, ONE_TOKEN * 200n);
  await sleep();
  await status();
}

async function stop() {
  const C = await ethers.getContractFactory("MiningCompetition");
  const c = await C.attach(ADDR);
  await c.changeStatus(2, PROVER, 0, 0);
  await sleep();
  await status();
}

async function deposit() {
  const [t, _] = await attachContract("Token");
  const balance = await t.balanceOf(ADDR);
  console.log("Balance1:", balance / ONE_TOKEN);
  await t.transfer(ADDR, INIT_TOKEN);
  await sleep();
  const balance2 = await t.balanceOf(ADDR);
  console.log("Balance2:", balance2 / ONE_TOKEN);
}

async function allowlist(addr) {
  const C = await ethers.getContractFactory("MiningCompetition");
  const c = await C.attach(ADDR);
  await c.allow(addr, true);
}

async function main() {
  await status();
  // await start();
  // await stop();
  // await deposit();
  // await allowlist("0xdc7ee084e87e4cf62e34534ab8bf9bef19f077d7");
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
