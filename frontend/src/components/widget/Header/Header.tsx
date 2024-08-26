"use client";
import { memo } from "react";
import { MemoizedLocation } from "../navbar/navbar";
import ConnectButton from "../walletButton/ConnectWalletButton";
import { useSession } from "@/components/hooks/useSession";

const Header = () => {
  const { hasToken } = useSession();
  return (
    <div className="flex items-center justify-between min-h-[44px] mt-[44px] mb-[40px]">
      <MemoizedLocation />
      {hasToken ? <ConnectButton /> : null}
    </div>
  );
};
export default memo(Header);
