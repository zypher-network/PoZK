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
      "Prover",
      "Task",
      "Controller",
      "L2Vesting",
      ["others", "MiningCompetition"],
    ];

    contracts.forEach(function (item) {
      let root = item + ".sol";
      let name = item;
      if (Array.isArray(item)) {
        name = item[1];
        root = item[0] + "/" + name + ".sol";
      }
      const readPath = `build/artifacts/contracts/${root}/${name}.json`;
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
