"use client";
import Link from "next/link";
import Image from "next/image";
import { useCallback, useLayoutEffect, useMemo, useState } from "react";
import { Button } from "@/components/ui/button";
import { evmWallet } from "@/web3/wallet";
import { useWeb3Modal } from "@web3modal/wagmi/react";
import { useSession } from "@/components/hooks/useSession";
import { parseSignature } from "viem";
import { useToast } from "@/components/ui/use-toast";
import { useRouter } from "next/navigation";
import { appName } from "@/constants/constants";
import sleep from "@/lib/sleep";
import { useRecoilState } from "recoil";
import { FailedRoute } from "@/components/state/globalState";
import pozk from "@/services/pozk";
// import { useAccount, useDisconnect } from "wagmi";
// import { switchChain } from '@wagmi/core'
// import { chain, wagmiConfig } from "@/web3/wagmi.config";

export default function Home() {
  const [failedRoute, setFailedRoute] = useRecoilState(FailedRoute);
  const { toast } = useToast();
  const [loading, setLoading] = useState(false);
  const { close } = useWeb3Modal();
  // const { disconnect } = useDisconnect();
  // const { chainId } = useAccount();
  const { setToken, setAccount, hasToken } = useSession();
  const router = useRouter();
  const redirectPath = useMemo(() => {
    return failedRoute ?? "/dashboard";
  }, [failedRoute]);
  useLayoutEffect(() => {
    if (hasToken) {
      setFailedRoute(undefined);
      router.push(redirectPath);
    }
  }, [hasToken, redirectPath]);
  const connectWallet = useCallback(async () => {
    try {
      setLoading(true);
      // console.log(chainId, chain.id);
      // if (chainId !== chain.id) {
      //   await switchChain(wagmiConfig, { chainId: chain.id });
      // }
      const { message, signature, ...msgData } = await evmWallet.signByEIP4361(
        "Welcome to Zytron!"
      );
      if (message && signature) {
        console.log({ signature });
        const params = {
          message: message,
          signature: signature,
        };
        console.log({ params });
        const res: any = await pozk.login(params);
        const token = res.token;
        // const local: any = await api.post(
        //   "/login",
        //   { token },
        //   {
        //     useUrl: true,
        //   }
        // );
        setToken(token);
        setAccount();
        // console.log({ res, local });
        setFailedRoute(undefined);
        await sleep(0.6);
        // router.refresh();
        router.push(redirectPath);
      } else {
        setLoading(false);
        close();
        // disconnect();
      }
    } catch (error: any) {
      setLoading(false);
      console.error("Error signing in with VIEM:", error);
      const { code, message } = error || {};
      console.log(1111);
      toast({
        variant: "destructive",
        title: code ?? "Failed",
        description: <p className="text-[14px]">{message ?? "Failed"}</p>,
      });
    }
  }, []);
  return (
    <div className="w-full h-[800px] flex items-center justify-center">
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
