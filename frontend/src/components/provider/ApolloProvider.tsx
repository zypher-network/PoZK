'use client';
import React from 'react'
import { ApolloClient, InMemoryCache, ApolloProvider as Provider } from '@apollo/client';

const client = new ApolloClient({
  uri: 'https://pozk-subgraph.zypher.dev/subgraphs/name/testnet/pozk/',
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
