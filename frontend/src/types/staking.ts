import { Address } from "viem";

export type GQLStaking = {
  id: string;
  prover: Address;
  amount: string;
  account: Address;
  newAmount: string;
  roleType: number;
}
