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
  const { address, isConnected } = useAccount();
  const { fetch, fetching } = useControllerStore(useShallow(state => ({ fetch: state.fetch, fetching: state.fetching })));

  const isSameAccount = useMemo(() =>
    account?.toLowerCase().startsWith(address?.toLowerCase() ?? '') ?? false,
    [address],
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
    if (hasToken && isConnected && isSameAccount) {
      handleInit();
    } else {
      setIsCompleted(true);
      loginOut();
    }
  }, [hasToken, isConnected, isSameAccount]);

  return [hasAuth, isCompleted];
}

export default useAuth;
