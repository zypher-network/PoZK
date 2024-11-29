import { gql } from "@apollo/client";

export const GET_PROVERS = gql`
  query MyQuery {
    provers {
      work
      version
      stop
      status
      name
      minable
      overtime
      checkUrl
      id
      approved
    }
  }
`;
