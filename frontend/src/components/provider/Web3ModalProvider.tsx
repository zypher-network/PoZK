"use client";
import { chain, projectId, wagmiConfig } from "@/web3/wagmi.config";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { createWeb3Modal } from "@web3modal/wagmi/react";
import { ReactNode } from "react";
import { type State, WagmiProvider } from "wagmi";

createWeb3Modal({
  wagmiConfig: wagmiConfig,
  // siweConfig: siweConfig,
  projectId: projectId,
  defaultChain: chain,
  enableOnramp: true,
  enableAnalytics: true,
  termsConditionsUrl: "https://evoverses.com/terms",
  privacyPolicyUrl: "https://evoverses.com/privacy",
  themeVariables: {
    "--w3m-accent": "hsl(248 100% 65%)",
  },
});
export default function Web3ModalProvider({
  children,
  initialState,
}: {
  children: ReactNode;
  initialState?: State;
}) {
  const queryClient = new QueryClient();
  return (
    <WagmiProvider config={wagmiConfig} initialState={initialState}>
      <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
    </WagmiProvider>
  );
}
