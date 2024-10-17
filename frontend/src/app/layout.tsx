import type { Metadata } from "next";
import "./globals.css";
import { kanit } from "@/lib/font";
import generateMetadata from "@/lib/generateMetadata";
import { cn } from "@/lib/utils";
import MountedProvider from "@/components/provider/MountedProvider";
import Web3ModalProvider from "@/components/provider/Web3ModalProvider";
import { Toaster } from "@/components/ui/toaster";
// import { cookieToInitialState } from "wagmi";
// import { wagmiConfig } from "@/web3/wagmi.config";
// import { headers } from "next/headers";
import RecoilRootProvider from "@/components/provider/RecoilRootProvider";
import SessionProvider from "@/components/provider/SessionProvider";
import ApolloProvider from "@/components/provider/ApolloProvider";

export const metadata: Metadata = generateMetadata();

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={cn(kanit.className, "dark", "h-full")}>
        <RecoilRootProvider>
          <Web3ModalProvider>
            <MountedProvider>
              <SessionProvider>
                <ApolloProvider>
                  {children}
                </ApolloProvider>
              </SessionProvider>
            </MountedProvider>
          </Web3ModalProvider>
        </RecoilRootProvider>
        <Toaster />
      </body>
    </html>
  );
}
