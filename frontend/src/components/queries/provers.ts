import { gql } from "@apollo/client";

export const GET_PROVERS = gql`
  query MyQuery {
    provers(where: {stop: false}) {
      work
      version
      stop
      status
      name
      minable
      overtime
      id
      approved
    }
  }
`;
