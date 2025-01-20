import { Address } from "viem";
import network from "@/../../../public/networks.json";

export enum ChainId {
  ZYTRONTESTNET = "50098", // zytron testnet
  ZYTRON = "9901",         // zytron mainnet
  BASESEPOLIA = "84532",   // base sepolia
  BASE = "8453",           // base mainnet
}

export const CHAINID = ChainId.BASESEPOLIA; // default network

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
  [ChainId.ZYTRONTESTNET]: {
      Addresses: network.zytrontestnet.Addresses.address as Address,
      Token: network.zytrontestnet.Token.address as Address,
      Vesting: network.zytrontestnet.Vesting.address as Address,
      Epoch: network.zytrontestnet.Epoch.address as Address,
      Stake: network.zytrontestnet.Stake.address as Address,
      Reward: network.zytrontestnet.Reward.address as Address,
      Prover: network.zytrontestnet.Prover.address as Address,
      Task: network.zytrontestnet.Task.address as Address,
      Controller: network.zytrontestnet.Controller.address as Address,
  },
  [ChainId.ZYTRON]: {
      Addresses: network.zytron.Addresses.address as Address,
      Token: network.zytron.Token.address as Address,
      Vesting: network.zytron.Vesting.address as Address,
      Epoch: network.zytron.Epoch.address as Address,
      Stake: network.zytron.Stake.address as Address,
      Reward: network.zytron.Reward.address as Address,
      Prover: network.zytron.Prover.address as Address,
      Task: network.zytron.Task.address as Address,
      Controller: network.zytron.Controller.address as Address,
  },
  [ChainId.BASESEPOLIA]: {
      Addresses: network.basesepolia.Addresses.address as Address,
      Token: network.basesepolia.Token.address as Address,
      Vesting: network.basesepolia.Vesting.address as Address,
      Epoch: network.basesepolia.Epoch.address as Address,
      Stake: network.basesepolia.Stake.address as Address,
      Reward: network.basesepolia.Reward.address as Address,
      Prover: network.basesepolia.Prover.address as Address,
      Task: network.basesepolia.Task.address as Address,
      Controller: network.basesepolia.Controller.address as Address,
  },
  [ChainId.BASE]: {
      Addresses: network.base.Addresses.address as Address,
      Token: network.base.Token.address as Address,
      Vesting: network.base.Vesting.address as Address,
      Epoch: network.base.Epoch.address as Address,
      Stake: network.base.Stake.address as Address,
      Reward: network.base.Reward.address as Address,
      Prover: network.base.Prover.address as Address,
      Task: network.base.Task.address as Address,
      Controller: network.base.Controller.address as Address,
  }
};

export const ChainRpcUrls: Record<ChainId, string[]> = {
  [ChainId.ZYTRONTESTNET]: ["https://rpc-testnet.zypher.network"],
  [ChainId.ZYTRON]: ["https://rpc.zypher.network"],
  [ChainId.BASESEPOLIA]: ["https://sepolia.base.org"],
  [ChainId.BASE]: ["https://mainnet.base.org"],
};

export const SubgraphUrls: Record<ChainId, string> = {
  [ChainId.ZYTRONTESTNET]: "https://pozk--subgraph.zypher.dev/subgraphs/name/zytrontestnet/pozk/",
  [ChainId.ZYTRON]: "https://pozk-subgraph.zypher.dev/subgraphs/name/zytron/pozk/",
  [ChainId.BASESEPOLIA]: "https://pozk-subgraph.zypher.dev/subgraphs/name/basesepolia/pozk/",
  [ChainId.BASE]: "https://pozk-subgraph.zypher.dev/subgraphs/name/base/pozk/",
}

export const ZeroGasUrls: Record<ChainId, string> = {
  [ChainId.ZYTRONTESTNET]: "https://gas-testnet.zypher.network",
  [ChainId.ZYTRON]: "https://gas.zypher.network",
  [ChainId.BASESEPOLIA]: "https://gas-basesepolia.zypher.dev",
  [ChainId.BASE]: "https://gas-base.zypher.dev",
}
