import { useAccount } from "wagmi";
import { useSession } from "./useSession";
import { useLayoutEffect, useMemo, useState } from "react";
import useControllerStore from "../state/controllerStore";
import { useFailedRoute } from "./useFailedRoute";
import { useShallow } from "zustand/react/shallow";

const useAuth = () => {
  const [hasAuth, setHasAuth] = useState(false);
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
    } catch (error) {
      Failed(error);
    }
  }

  useLayoutEffect(() => {
    if (hasToken && isConnected && isSameAccount) {
      handleInit();
    } else {
      loginOut();
    }
  }, [hasToken, isConnected, isSameAccount]);

  return hasAuth;
}

export default useAuth;
