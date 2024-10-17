"use client";
import SessionManager from "@/lib/session/SessionManager";
import sleep from "@/lib/sleep";
import { usePathname, useRouter } from "next/navigation";
import {
  createContext,
  memo,
  useCallback,
  useState,
} from "react";

interface SessionContextValue {
  account: string | undefined;
  hasToken: boolean;
  getToken: () => string | undefined;
  setToken: (token: string) => void;
  setAccount: (address: string, chainId: number) => void;
  deleteToken: () => void;
  loginOut: () => void;
}

export const SessionContext = createContext<SessionContextValue | undefined>(
  undefined
);

const SessionProvider = ({ children }: { children: React.ReactNode }) => {
  const route = useRouter();
  const pathname = usePathname();
  const [hasToken, setHasToken] = useState(
    SessionManager.getSession() !== undefined
  );
  const [account, _setAccount] = useState(SessionManager.getAccount());

  const getToken = useCallback(() => SessionManager.getSession(), []);
  const setToken = useCallback((token: string) => {
    SessionManager.setSession(token);
    setHasToken(true);
  }, []);
  const setAccount = (address: string, chainId: number) => {
    SessionManager.setAccount(`${address}${chainId}`);
    _setAccount(`${address}${chainId}`);
    setHasToken(true);
  };
  const deleteToken = useCallback(() => {
    SessionManager.deleteSession();
    setHasToken(false);
  }, []);
  const loginOut = useCallback(async () => {
    deleteToken();
    setHasToken(false);
    await sleep(0.2);
    console.log('---- logout');
    pathname !== "/" && route.push("/");
  }, [pathname]);

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
