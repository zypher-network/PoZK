import { useEffect } from "react";
import { useAccount } from "wagmi";
import useBalanceStore from "../state/balanceStore";

const useGetUserBalance = () => {
  const { address } = useAccount();
  const updateBalance = useBalanceStore(state => state.updateBalance);
  useEffect(() => {
    address && updateBalance(address);
  }, [address]);
}

export default useGetUserBalance;
