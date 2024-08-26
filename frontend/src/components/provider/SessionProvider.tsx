"use client";
import api from "@/lib/fetch";
import SessionManager from "@/lib/session/SessionManager";
import sleep from "@/lib/sleep";
import { usePathname, useRouter } from "next/navigation";
import {
  createContext,
  memo,
  useCallback,
  useLayoutEffect,
  useMemo,
  useState,
} from "react";
import { useAccount } from "wagmi";

interface SessionContextValue {
  account: string | undefined;
  hasToken: boolean;
  getToken: () => string | undefined;
  setToken: (token: string) => void;
  setAccount: () => void;
  deleteToken: () => void;
  loginOut: () => void;
}

export const SessionContext = createContext<SessionContextValue | undefined>(
  undefined
);

const SessionProvider = ({ children }: { children: React.ReactNode }) => {
  const pathname = usePathname();
  const route = useRouter();
  const { address, chainId } = useAccount();
  const [hasToken, setHasToken] = useState(
    SessionManager.getSession() !== undefined
  );
  const [account, _setAccount] = useState(SessionManager.getAccount());

  const getToken = useCallback(() => SessionManager.getSession(), []);
  const setToken = useCallback((token: string) => {
    SessionManager.setSession(token);
    setHasToken(true);
  }, []);
  const setAccount = useCallback(() => {
    SessionManager.setAccount(`${address}${chainId}`);
    _setAccount(`${address}${chainId}`);
    setHasToken(true);
  }, [address, chainId]);
  const deleteToken = useCallback(() => {
    SessionManager.deleteSession();
    setHasToken(false);
  }, []);
  const loginOut = useCallback(async () => {
    console.log("99999999999999999999999999999999");
    deleteToken();
    setHasToken(false);
    await sleep(0.2);
    pathname !== "/" && route.push("/");
  }, [pathname]);
  // const nowAccount = useMemo(() => {
  //   return `${address}${chainId}`;
  // }, [address, chainId]);
  // useLayoutEffect(() => {
  //   // console.log({ nowAccount, account });
  //   if (
  //     nowAccount === "undefinedundefined" ||
  //     (nowAccount.toLowerCase() !== account?.toLowerCase() &&
  //       account !== "undefinedundefined")
  //   ) {
  //     loginOut();
  //   }
  // }, [nowAccount, account, loginOut]);
  return (
    <SessionContext.Provider
      value={{
        account,
        setAccount,
        hasToken,
        getToken,
        setToken,
        deleteToken,
        loginOut,
      }}
    >
      {children}
    </SessionContext.Provider>
  );
};
export default memo(SessionProvider);
