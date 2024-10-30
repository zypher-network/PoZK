"use client";
import { memo } from "react";
import { MemoizedLocation } from "../navbar/navbar";
import ConnectButton from "../walletButton/ConnectWalletButton";
import { useSession } from "@/components/hooks/useSession";
import { useAccount } from "wagmi";

const Header = () => {
  const { hasToken } = useSession();
  const { isConnected } = useAccount();
  return (
    <div className="flex items-center justify-between min-h-[44px] mt-[44px] mb-[40px]">
      <MemoizedLocation />
      {!isConnected && <ConnectButton />}
      {hasToken && isConnected ? <ConnectButton /> : null}
    </div>
  );
};
export default memo(Header);
