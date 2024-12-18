"use client";
import Link from "next/link";
import { useAccount } from "wagmi";
import { zeroAddress } from "viem";
import { useRecoilState } from "recoil";
import cx from 'classnames';
import { useEffect, useMemo, useState } from "react";
import { Button } from "@/components/ui/button";
import { evmWallet } from "@/web3/wallet";
import { useSession } from "@/components/hooks/useSession";
import { useToast } from "@/components/ui/use-toast";
import { useRouter } from 'next/navigation';
import { appName } from "@/constants/constants";
import { FailedRoute } from "@/components/state/globalState";
import pozk from "@/services/pozk";

export default function Home() {
  const [failedRoute, setFailedRoute] = useRecoilState(FailedRoute);
  const { toast } = useToast();
  const [loading, setLoading] = useState(false);
  const { setToken, setAccount, account, hasToken, loginOut } = useSession();
  const router = useRouter();
  const { address, chainId, isConnected } = useAccount();
  const redirectPath = useMemo(() => {
    return failedRoute ?? "/dashboard";
  }, [failedRoute]);

  const signLogin = async () => {
    try {
      const { message, signature } = await evmWallet.signByEIP4361(
        "Welcome to PoZK!"
      );
      if (message && signature && address && chainId) {
        const params = {
          message: message,
          signature: signature,
        };
        const res: any = await pozk.login(params);
        const token = res.token;
        setAccount(address, chainId);
        setToken(token);
        setTimeout(() => {
          setFailedRoute(undefined);
          router.push(redirectPath);
        }, 1000);
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

  const handleConnect = async () => {
    setLoading(true);
    loginOut();
    try {
      if (!isConnected) {
        await evmWallet.initAndConnect();
      }
    } catch (error) {
      console.log(error);
      setLoading(false);
    }
  }

  useEffect(() => {
    if (loading && isConnected) {
      signLogin();
    }
  }, [loading, isConnected]);

  useEffect(() => {
    if (isConnected && hasToken && account?.startsWith(address ?? zeroAddress) && !loading) {
      setFailedRoute(undefined);
      router.push(redirectPath);
    }
  }, [isConnected, hasToken, account, address, loading]);

  return (
    <div className={cx('w-full h-[800px] flex items-center justify-center', { 'opacity-30 pointer-events-none': false })}>
      <div className="flex items-center justify-center flex-col max-w-lg w-full space-y-8 p-14 shadow-lg rounded-lg bg-slate-800">
        <Link href="/">
          <img
            src="/nav/logo_dark.png"
            width={185}
            height={40}
            alt={appName}
          />
        </Link>
        <h1 className="text-2xl font-bold mb-6 text-center">
          Welcome to PoZK
        </h1>
        <Button
          className="w-[150px]"
          onClick={handleConnect}
          isLoading={loading}
          disabled={loading}
        >
          Connect Wallet
        </Button>
      </div>
    </div>
  );
}
