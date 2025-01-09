import { gql } from "@apollo/client";
import { Address } from "viem";

export const GET_MINER_TESTS = gql`
  query MyQuery($address: String) {
    minerTests(where: {account: $address, result: null}) {
      submitAt
      result
      overtimeAt
      prover
      id
      account
    }
  }
`

export type MinerTest = {
  submitAt: string | null;
  result: boolean | null;
  overtimeAt: string;
  prover: Address;
  id: string;
  account: Address;
}

export interface IMinerTests {
  minerTests: MinerTest[];
}
