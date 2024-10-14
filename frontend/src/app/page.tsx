"use client";
import Link from "next/link";
import Image from "next/image";
import cx from 'classnames';
import { useLayoutEffect, useMemo, useState } from "react";
import { Button } from "@/components/ui/button";
import { evmWallet } from "@/web3/wallet";
import { useSession } from "@/components/hooks/useSession";
import { useToast } from "@/components/ui/use-toast";
import { useRouter } from "next/navigation";
import { appName } from "@/constants/constants";
import sleep from "@/lib/sleep";
import { useRecoilState } from "recoil";
import { FailedRoute } from "@/components/state/globalState";
import pozk from "@/services/pozk";
import useAuth from "@/components/hooks/useAuth";
import { useAccount } from "wagmi";
// import { useAccount, useDisconnect } from "wagmi";
// import { switchChain } from '@wagmi/core'
// import { chain, wagmiConfig } from "@/web3/wagmi.config";

export default function Home() {
  const [failedRoute, setFailedRoute] = useRecoilState(FailedRoute);
  const { toast } = useToast();
  const [loading, setLoading] = useState(false);
  const { setToken, setAccount } = useSession();
  const router = useRouter();
  const [hasAuth, isCompleted] = useAuth();
  const { address, chainId } = useAccount();
  const redirectPath = useMemo(() => {
    return failedRoute ?? "/dashboard";
  }, [failedRoute]);

  const connectWallet = async () => {
    try {
      setLoading(true);
      const { message, signature } = await evmWallet.signByEIP4361(
        "Welcome to Zytron!"
      );
      if (message && signature) {
        const params = {
          message: message,
          signature: signature,
        };
        const res: any = await pozk.login(params);
        const token = res.token;
        if (address && chainId) {
          setToken(token);
          setAccount(address, chainId);
          setFailedRoute(undefined);
          await sleep(0.6);
          // router.push(redirectPath);
        }
      } else {
        setLoading(false);
        // disconnect();
      }
    } catch (error: any) {
      setLoading(false);
      console.error("Error signing in with VIEM:", error);
      const { code, message } = error || {};
      toast({
        variant: "destructive",
        title: code ?? "Failed",
        description: <p className="text-[14px]">{message ?? "Failed"}</p>,
      });
    }
  };

  useLayoutEffect(() => {
    if (hasAuth && isCompleted) {
      setFailedRoute(undefined);
      router.push(redirectPath);
    }
  }, [hasAuth, redirectPath]);

  return (
    <div className={cx('w-full h-[800px] flex items-center justify-center', { 'opacity-30 pointer-events-none': !isCompleted })}>
      <div className="flex items-center justify-center flex-col max-w-lg w-full space-y-8 p-14 shadow-lg rounded-lg bg-slate-800">
        <Link href="/">
          <Image
            src="/nav/logo_dark.png"
            width={185}
            height={40}
            alt={appName}
          />
        </Link>
        <h1 className="text-2xl font-bold mb-6 text-center">
          Welcome to Zytron
        </h1>
        <Button
          className="w-[150px]"
          onClick={connectWallet}
          isLoading={loading}
          disabled={loading}
        >
          Connect Wallet
        </Button>
      </div>
    </div>
  );
}
