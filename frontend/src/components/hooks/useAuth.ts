'use client'
import { useAccount } from "wagmi";
import { useSession } from "./useSession";
import { useLayoutEffect, useMemo, useState } from "react";
import useControllerStore from "../state/controllerStore";
import { useFailedRoute } from "./useFailedRoute";
import { useShallow } from "zustand/react/shallow";

const useAuth = () => {
  const [hasAuth, setHasAuth] = useState(false);
  const [isCompleted, setIsCompleted] = useState(false);
  const Failed = useFailedRoute();
  const { account, hasToken, loginOut } = useSession();
  const { address, isConnected, isDisconnected } = useAccount();
  const { fetch } = useControllerStore(useShallow(state => ({ fetch: state.fetch, fetching: state.fetching })));

  const isSameAccount = useMemo(() =>
    account?.toLowerCase().startsWith(address?.toLowerCase() ?? '') ?? false,
    [address, isDisconnected],
  );
  
  const handleInit = async () => {
    try {
      await fetch(1);
      setHasAuth(true);
      setIsCompleted(true);
    } catch (error) {
      setIsCompleted(true);
      Failed(error);
      loginOut();
    }
  }

  useLayoutEffect(() => {
    if (!isDisconnected) {
      if (isConnected && hasToken && isSameAccount) {
        handleInit();
      } else {
        setIsCompleted(true);
        loginOut();
      }
    }
  }, [isDisconnected, hasToken]);

  return [hasAuth, isCompleted];
}

export default useAuth;
