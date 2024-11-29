import { Address } from "viem";
export enum IGameConfigKey {
  id = "id",
  name = "name",
  minerStaking = "minerStaking",
  minerStakingStr = "minerStakingStr",
  isMiner = "isMiner",
  isMinerStr = "isMinerStr",
  claimable = "claimable",
  minStakeAmount = "minStakeAmount",
  minStakeAmountStr = "minStakeAmountStr",
  prover = "prover",
  tokenBalance = "tokenBalance",
  tokenBalanceStr = "tokenBalanceStr",
  approve = "approve",
}
export type IGameConfig = {
  [IGameConfigKey.id]: string;
  [IGameConfigKey.name]: string;
  [IGameConfigKey.minerStaking]: string;
  [IGameConfigKey.minerStakingStr]: string;
  [IGameConfigKey.isMiner]: boolean;
  [IGameConfigKey.isMinerStr]: string;
  [IGameConfigKey.claimable]: string;
  [IGameConfigKey.minStakeAmount]: string;
  [IGameConfigKey.minStakeAmountStr]: string;
  [IGameConfigKey.prover]: Address;
  [IGameConfigKey.tokenBalance]: string;
  [IGameConfigKey.tokenBalanceStr]: string;
  [IGameConfigKey.approve]: string;
  version: string;
};

export type UserConatiner = {
  id: string;
  running: boolean;
  created: string;
  needUpgrade: boolean;
}

export type GQLProver = {
  work: string;
  version: string;
  stop: boolean;
  status: number;
  name: string;
  minable: boolean;
  overtime: string;
  checkUrl: boolean;
  id: Address;
  approved: boolean;
}

export type UserProver = GQLProver & {
  name: string;
  containers: UserConatiner[];
}
