import { gql } from "@apollo/client";

export const GET_EPOCHES = gql`
  query MyQuery {
    epoches(orderBy: endAt, orderDirection: desc) {
      id
      startAt
      endAt
    }
  }
`

export const GET_MINER_REWARDS = gql`
  query MyQuery($id: String) {
    reward(id: $id) {
      totalClaim
      id
      claimList(orderDirection: desc, orderBy: epoch) {
        claim
        epoch
        estimate
        id
        prover
        roleType
      }
    }
    unstakingClaims(where: {unstaking_: {id: $id}}) {
      id
      amount
    }
  }
`
