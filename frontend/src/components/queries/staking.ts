import { gql } from "@apollo/client";

export const GET_MINER_STAKING = gql`
  query MyQuery($account: String) {
    stakings(where: {account: $account}) {
      id
      prover
      amount
      account
      newAmount
      roleType
    }
  }
`
