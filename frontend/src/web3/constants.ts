import { Address } from "viem";

export enum ChainId {
  TESTNET = "19546",
}
export const CHAINID = ChainId.TESTNET;
type IContractList = {
  Addresses: Address;
  Token: Address;
  Vesting: Address;
  Epoch: Address;
  Stake: Address;
  Reward: Address;
  Prover: Address;
  Task: Address;
  Controller: Address;
};
export const contractAddress: Record<ChainId, IContractList> = {
  [ChainId.TESTNET]: {
      Addresses: "0x5b92b011513F9aAF8F6541003DC088625E7438e8",
      Token: "0x3b942aEbA931c3350E49F4f54ED5Af943cdd86C2",
      Vesting: "0x47732f366aded920b5b96F7f220812b3a49EB66E",
      Epoch: "0x0235f8aA167D848Fa20CA39B5F02C4092D616105",
      Stake: "0xa7cF188836aa197015685F2F868bf470db5E66cE",
      Reward: "0x63d57BdfF1176AB682CE93C289624f2C315810BB",
      Prover: "0x075A3517b1350bFFEBC5F15dF034B6856c7B483A",
      Task: "0xA72C3013A345522057005F0e034a8440f8D7a4aC",
      Controller: "0x40c55B01A9272746AF60A9623f1F7Ceb91eF3AB7",
  },
};
export const gamesList = {
  [ChainId.TESTNET]: {
    game1: {
      address: "0xd64b51e6f5db063c9532bfc5f9f3472265771827" as Address,
      version: "1",
    },
  },
};
export const ChainRpcUrls: Record<ChainId, string[]> = {
  [ChainId.TESTNET]: ["https://linea-testnet-zytron.zypher.game/"],
};
