import { cookieStorage, createStorage } from "wagmi";
import { defaultWagmiConfig } from "@web3modal/wagmi/react/config";
import generateMetadata from "@/lib/generateMetadata";
import { CHAINID, ChainId } from "@/web3/constants";

import { defineChain } from "viem/utils";
// const testnetSourceId = 59141; // Linea Sepolia
const mainnetSourceId = 59144; // Linea Mainnet

// const zytronTestnet = /*#__PURE__*/ defineChain({
//     id: 19546,
//     name: 'Zytron Testnet',
//     nativeCurrency: {
//         name: 'Ether',
//         symbol: 'ETH',
//         decimals: 18,
//     },
//     rpcUrls: {
//         default: {
//             http: ['https://linea-testnet-zytron.zypher.game'],
//         },
//     },
//     blockExplorers: {
//         default: {
//             name: 'Blockscout',
//             url: 'https://linea-testnet-zytron-blockscout.zypher.game',
//         },
//     },
//     testnetSourceId,
// })

const zytronMainnet = /*#__PURE__*/ defineChain({
    id: 9901,
    name: 'Zytron Mainnet',
    nativeCurrency: {
        name: 'Ether',
        symbol: 'ETH',
        decimals: 18,
    },
    rpcUrls: {
        default: {
            http: ['https://rpc.zypher.network'],
        },
    },
    blockExplorers: {
        default: {
            name: 'Blockscout',
            url: 'https://explorer.zypher.network',
        },
    },
    mainnetSourceId,
})

// export const projectId = process.env.NEXT_PUBLIC_PROJECT_ID;
// Get your projectId on https://cloud.walletconnect.com
export const projectId = "bc467c124a7a7a8ce06a41ef40b1b842";
const ChainList = {
  [ChainId.MAINNET]: zytronMainnet,
};
export const chain = ChainList[CHAINID];
export const wagmiConfig = defaultWagmiConfig({
  projectId: projectId,
  chains: [chain],
  metadata: generateMetadata(),
  ssr: true,
  storage: createStorage({ storage: cookieStorage }),
} as any);


// export const wagmiConfig = createConfig({
//   chains: [chain],
//   transports: {
//     [chain.id]: http(),
//   },
//   ssr: true,
//   storage: createStorage({ storage: cookieStorage }),
// })
