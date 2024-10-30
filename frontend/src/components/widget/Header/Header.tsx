"use client";
import { memo, useMemo } from "react";
import { MemoizedLocation } from "../navbar/navbar";
import ConnectButton from "../walletButton/ConnectWalletButton";
import { useSession } from "@/components/hooks/useSession";
import { useAccount } from "wagmi";
import { getGradientColors } from "@/lib/avatar";
import { zeroAddress } from "viem";
import { useWeb3Modal } from "@web3modal/wagmi/react";


const Header = () => {
  const { hasToken } = useSession();
  const { isConnected, address } = useAccount();
  const { open } = useWeb3Modal(); 

  const avatarColor = useMemo(() => getGradientColors(address ?? zeroAddress), [address]);
  return (
    <div className="flex items-center justify-between min-h-[44px] mt-[44px] mb-[40px]">
      <MemoizedLocation />
      {!isConnected && <ConnectButton />}
      {hasToken && isConnected ? (
        <div className="cursor-pointer flex justify-center items-center border-2 border-[#FFFFFF33] rounded-full" onClick={() => open({ view: 'Account' })}>
          <div
            className="w-[48px] h-[48px] rounded-full"
            style={{
              boxShadow: 'inset 0 0 0 1px rgba(0, 0, 0, 0.1)',
              backgroundColor: avatarColor[0],
              backgroundImage: `
                radial-gradient(at 66% 77%, ${avatarColor[1]} 0px, transparent 50%),
                radial-gradient(at 29% 97%, ${avatarColor[2]} 0px, transparent 50%),
                radial-gradient(at 99% 86%, ${avatarColor[3]} 0px, transparent 50%),
                radial-gradient(at 29% 88%, ${avatarColor[4]} 0px, transparent 50%)
              `
            }}
          />
        </div>
      ) : null}
    </div>
  );
};
export default memo(Header);
