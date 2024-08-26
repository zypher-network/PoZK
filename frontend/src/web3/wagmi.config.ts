import { cookieStorage, createStorage } from "wagmi";
import { opBNBTestnet } from "viem/chains";
import { defaultWagmiConfig } from "@web3modal/wagmi/react/config";
import generateMetadata from "@/lib/generateMetadata";
import { CHAINID, ChainId } from "@/web3/constants";

// export const projectId = process.env.NEXT_PUBLIC_PROJECT_ID;
// Get your projectId on https://cloud.walletconnect.com
export const projectId = "bc467c124a7a7a8ce06a41ef40b1b842";
const ChainList = {
  [ChainId.OPBNBTEST]: opBNBTestnet,
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
