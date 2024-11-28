import zeroGas from '@/services/zeroGas';
import React, { useEffect, useMemo, useState } from 'react'

import Loading from '@/components/icon/loading.svg';
import { ControllerContract } from '@/web3/contract/ControllerContract';
import { useAccount } from 'wagmi';
import { useToast } from '@/components/ui/use-toast';

interface IZeroGasStatus {
  address: string;
}

const ZeroGasStatus: React.FC<IZeroGasStatus> = ({ address }) => {
  const [checking, setChecking] = useState(true);
  const [hasSbt, setHasSbt] = useState(false);
  const [hasAdded, setHasAdded] = useState(false);
  const [aaWallet, setAAWallet] = useState('');
  const contract = useMemo(() => ControllerContract(), []);
  const { toast } = useToast();
  const { address: walletAddress } = useAccount();

  const checkSbt = async () => {
    try {
      const result = await zeroGas.getBalance(address);
      const { amount, wallet } = result ?? {};
      if (amount && wallet) {
        const sbtAmount = parseInt(amount, 16);
        setHasSbt(Boolean(sbtAmount));
        if (Boolean(sbtAmount)) {
          const isController = await contract.readContractData("check", [
            walletAddress,
            wallet,
          ]);
          setAAWallet(wallet);
          setHasAdded(isController as unknown as boolean);
        }
      }
    } catch (error) {
      console.log(error);
    }
    setChecking(false);
  }

  const addAAWallet = async () => {
    try {
      setChecking(true);
      const result = await zeroGas.createAAWallet(address);
      if (result) {
        await contract.writeContractMethod("add", [
          result.wallet,
        ]);
      }
      checkSbt();
    } catch (error) {
      toast({
        variant: "destructive",
        title: "Failed",
        description: <p className="text-[14px]">{(error as Error).message}</p>,
      });
      setChecking(false);
    }
  }

  const renderStatus = () => {
    if (checking) {
      return (
        <div className="flex items-center justify-center animate-spin opacity-60">
          <Loading className='scale-y-[-1]' height={'16px'} width={'16px'} /> 
        </div>
      );
    }
    if (hasAdded) {
      return <p className="text-[#26d962]">{'on'}</p>;
    }
    if (hasSbt) {
      return (
        <div className="py-1 cursor-pointer px-3 rounded text-sm bg-[#674dff]" onClick={addAAWallet}>Set</div>
      );
    }
    return <p className="text-[#9e9e9e]">{'off'}</p>;
  }

  useEffect(() => {
    checkSbt();
  }, [address]);
  return (
    <div className="flex justify-start w-[70px]">
      {renderStatus()}
    </div>
  )
};

export default ZeroGasStatus;
