const { readFileSync, writeFileSync } = require("fs");

const main = async () => {
  try {
    const contracts = [
      "Token",
      "Addresses",
      "Vesting",
      "Epoch",
      "Stake",
      "Reward",
      "ProverMarket",
      "TaskMarket",
      "Controller"
    ];

    contracts.forEach(function (name) {
      const readPath = `build/artifacts/contracts/${name}.sol/${name}.json`;
      const contract = JSON.parse(readFileSync(readPath, 'utf8'));
      console.log(`Load contract: ${name}`);

      const savePath = `../public/ABI/${name}.json`;
      writeFileSync(savePath, JSON.stringify(contract.abi, null, 2));
      console.log(`Saved contract ${name}`);
    });
  } catch (e) {
    console.log(`e`, e);
  }
};

main();
