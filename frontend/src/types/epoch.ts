import { Address } from "viem";

export type EpochClaim = {
  claim: string | null;
  epoch: string;
  estimate: string;
  id: string;
  prover: Address;
  roleType: number;
}

export type GQLEpochReward = {
  totalClaim: string;
  id: Address;
  claimList: EpochClaim[];
}

export type GQLUnstakingClaim = {
  id: Address;
  amount: string;
}

export type GQLEpoch = {
  id: string;
  startAt: string;
  endAt: string | null;
}

export type UserEpoch = GQLEpoch & {
  epoch: string;
  estimate: string;
  provers: Address[];
  claimable: boolean;
}
