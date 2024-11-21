'use client';
import React from 'react'
import { ApolloClient, InMemoryCache, ApolloProvider as Provider } from '@apollo/client';
import { CHAINID, SubgraphUrls } from "@/web3/constants";

const client = new ApolloClient({
  uri: SubgraphUrls[CHAINID],
  cache: new InMemoryCache(),
});

const ApolloProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  return (
    <Provider client={client}>
      {children}
    </Provider>
  )
};

export default ApolloProvider;
