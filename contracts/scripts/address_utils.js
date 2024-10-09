const { ethers, network } = require("hardhat");
const { readFileSync } = require('fs');

async function attachContract(name) {
  const filename = `../public/networks.json`;
  const obj = JSON.parse(readFileSync(filename, 'utf8'));
  const address = obj[network.name][name]["address"];
  const block = obj[network.name][name]["startBlock"];

  const C = await ethers.getContractFactory(name);
  return [await C.attach(address), block];
}

function sleep() {
  let waiting = 10000; // 20s
  if (network.name == "localhost") {
    waiting = 1000;
  }
  return new Promise(resolve => setTimeout(resolve, waiting));
}

module.exports = {
  attachContract,
  sleep
};
