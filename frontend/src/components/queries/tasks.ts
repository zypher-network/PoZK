import { gql } from "@apollo/client";

export const GET_TASKS = gql`
  query MyQuery($address: String, $prover: String) {
    tasks(where: {miner: $address, prover: $prover}) {
      id
      miner
      prover
    }
  }
`
