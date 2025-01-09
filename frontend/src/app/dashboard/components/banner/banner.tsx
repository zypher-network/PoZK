"use client";
import { memo, useMemo, useState } from "react";
import { useShallow } from "zustand/react/shallow";
import cx from 'classnames';
import Wifi from "@/components/icon/wifi.svg";
import WifiOff from "@/components/icon/wifi-off.svg";
import Copy from "@/components/icon/copy.svg";
import { useToast } from "@/components/ui/use-toast";
import { FaCheck } from "react-icons/fa";
import { Card, CardTitle } from "@/components/ui/card";
import { shortenWalletAddress } from "@/lib/shorten";
import { useAccount, useBalance, useReadContract, useReadContracts } from "wagmi";
import BigNumberJs, { BM18 } from "@/lib/BigNumberJs";
import useSubgraphStore from "@/components/state/subgraphStore";
import TokenABI from "@/constants/ABI/Token.json";
import { isTodaysEpoch } from "@/lib/day";
import { CHAINID, contractAddress } from "@/web3/constants";
import useControllerStore from "@/components/state/controllerStore";
import { wagmiConfig } from "@/web3/wagmi.config";

const Banner = () => {
  const { isConnected, address } = useAccount();
  const active = useControllerStore(state => state.active);
  const { staking, reward, epoches } = useSubgraphStore(useShallow(state => ({ staking: state.staking, epoches: state.epoches, reward: state.reward })));
  const minerToken = useReadContracts({
    contracts: [
      {
        abi: TokenABI,
        address: contractAddress[CHAINID].Token,
        functionName: 'balanceOf',
        args: [address],
      },
      {
        abi: TokenABI,
        address: contractAddress[CHAINID].Token,
        functionName: 'symbol',
      },
    ],
    config: wagmiConfig,
    query: {
      refetchInterval: 5000,
      enabled: Boolean(address),
      select: (result) => {
        return result ? `${new BigNumberJs(`${result[0]?.result ?? 0}`).div(BM18).toFormat()} ${result[1]?.result ?? ''}` : '';
      }
    },
  });
  const gasBalance = useBalance({
    address: active as any,
    config: wagmiConfig,
    query: {
      enabled: Boolean(active),
      refetchInterval: 5000,
      select: (result) => {
        return result ? `${new BigNumberJs(result.value.toString(10)).div(10 ** result.decimals).toFormat()} ${result.symbol}` : '';
      }
    }
  })
  const rewardInfo = useMemo(() => {
    let totalRewards = new BigNumberJs('0');
    const lastEpochEarning = epoches.data[1];
    const todaysEpoches = epoches.data.filter(epoch => isTodaysEpoch(epoch)).map(epoch => epoch.id);
    let latestEarnings = new BigNumberJs('0');
    let todayEarnings = new BigNumberJs('0');
    if (reward.data) {
      for (const claim of (reward.data.claimList ?? [])) {
        totalRewards = totalRewards.plus(claim.claim ?? claim.estimate ?? '0');
        if (claim.epoch === lastEpochEarning?.id) {
          latestEarnings = latestEarnings.plus(claim.estimate ?? '0');
        }
        if (todaysEpoches.includes(claim.epoch)) {
          todayEarnings = todayEarnings.plus(claim.claim ?? claim.estimate ?? '0');
        }
      }
    }
    return {
      latestEpoch: lastEpochEarning?.id ?? '-',
      totalEarnings: totalRewards.div(BM18).toFormat(),
      latestEarnings: latestEarnings.div(BM18).toFormat(),
      todayEarnings: todayEarnings.div(BM18).toFormat(),
    }
  }, [reward.data, epoches.data]);

  const totalStaking = useMemo(() => {
    return staking.data.reduce((prev, curr) => prev.plus(curr.newAmount), new BigNumberJs('0'));
  }, [staking])
  return (
    <div className="flex justify-between items-stretch gap-[24px]">
      <Card
        className="
        basis-2/3
        bg-gradient-to-br from-[#9277FD] to-[#674EFF]
        relative
        min-h-[246px]
      "
      >
        <p className="text-[20px]">Earnings Estimate</p>
        <h4 className="font-semibold text-[44px]">{rewardInfo.totalEarnings}</h4>
        <ul
          className="
          grid
          items-start
          relative
          gap-2
          z-10"
          style={{
            gridTemplateColumns: '160px 20px 160px 20px 160px',
            filter: 'drop-shadow(1px 1px 1px black)'
          }}
        >
          {[
            {
              label: `Epoch ${rewardInfo.latestEpoch} Earnings`,
              value: rewardInfo.latestEarnings,
            },
            {
              label: "Today's Earnings",
              value: rewardInfo.todayEarnings,
            },
            {
              label: "Staking amount",
              value: totalStaking.div(BM18).toFormat(2),
            },
          ].map((v, index) => (
            <Item key={v.label} isLast={index === 2} item={v} />
          ))}
          {[
            {
              label: "Miner Token Balance",
              value: minerToken.data ?? '-',
            },
            {
              label: "Controller Gas Balance",
              value: gasBalance.data ?? '-',
            },
          ].map((v, index) => (
            <Item key={v.label} isLast={index === 1} item={v} />
          ))}
        </ul>
        <img
          className="absolute top-4 right-4"
          src="/dashboard/banner_earning.webp"
          width={180}
          height={180}
          alt="dashboard"
        />
      </Card>
      <Card className="flex flex-col gap-0 basis-1/3 justify-start">
        <CardTitle className="border-b-[1px] pb-[28px]border-[#1F2D4E] pb-[12px]">
          Network State
        </CardTitle>
        <div
          className={cx(
            'border bg-opacity-30 rounded-[100px] flex justify-between items-center px-[28px] h-[48px] my-[16px]',
            {
              'border-[#674DFF] bg-[#674DFF]': isConnected,
              'border-[#4C4B56] bg-[#4C4B56]': !isConnected,
            }
          )}
        >
          {isConnected ? <Wifi /> : <WifiOff />}
          <div className="flex justify-center gap-[12px] items-center">
            <p className="font-light text-[20px]">
              {isConnected ? 'Connected' : 'Disconnect'}
            </p>
            <div
              className={cx(
                'w-[8px] h-[8px] rounded-[50%]',
                {
                  'bg-[#A3E636] shadow-[0_0_0_4px_rgba(163,230,54,0.3)]': isConnected,
                  'bg-[#E44042] shadow-[0_0_0_4px_rgba(228,64,66,0.3)]': !isConnected,
                }
              )}
            />
          </div>
          <div />
        </div>
        {isConnected && <Address />}
      </Card>
    </div>
  );
};
type ITem = {
  label: string;
  value: string;
};
const Item = ({ item, isLast }: { item: ITem; isLast: boolean }) => {
  const { label, value } = item;
  return (
    <>
      <li>
        <p className="font-light text-[16px] text-nowrap">{label}</p>
        <h5 className="text-[20px] font-medium text-nowrap">{value}</h5>
      </li>
      {isLast ? null : (
        <li className="min-w-[1px] h-[44px] bg-[#fff] opacity-10" style={{ margin: 'auto' }} />
      )}
    </>
  );
};
const Address = () => {
  const { address } = useAccount();
  const active = useControllerStore(state => state.active);
  const [showAddressCopy, setShowAddressCopy] = useState<boolean>(false);
  const [showControllerCopy, setShowControllerCopy] = useState<boolean>(false);
  const { toast } = useToast();
  const handleCopyClick = (isAddress: boolean) => {
    toast({
      title: "Copied to clipboard!",
      description: "address",
    });
    if (isAddress) {
      setShowAddressCopy(true);
      setShowControllerCopy(false);
    } else {
      setShowAddressCopy(false);
      setShowControllerCopy(true);
    }
    setTimeout(() => {
      if (isAddress) {
        setShowAddressCopy(false);
      } else {
        setShowControllerCopy(false);
      }
    }, 3000);
  };
  const shorten = useMemo(() => {
    return address ? shortenWalletAddress(address, "normal") : "";
  }, [address]);
  const shortenController = useMemo(() => {
    return active ? shortenWalletAddress(active, "normal") : "";
  }, [active]);
  return (
    <div
      className="flex flex-col items-center rounded-[10px] bg-[#0A1223] px-6 py-[12px] gap-1"
    >
      <div className="flex justify-between items-center w-full">
        <div className="flex items-center">
          <p className="text-[14px] text-nowrap">wallet address:</p>
          <p className="font-light text-[18px] px-2">{shorten}</p>
        </div>
        {showAddressCopy ? <FaCheck /> : <Copy className="cursor-pointer" onClick={() => handleCopyClick(true)} />}
      </div>
      <div className="flex justify-between items-center w-full">
        <div className="flex items-center">
          <p className="text-[14px] text-nowrap">controller address:</p>
          <p className="font-light text-[18px] px-2">{shortenController}</p>
        </div>
        {showControllerCopy ? <FaCheck /> : <Copy className="cursor-pointer" onClick={() => handleCopyClick(false)} />}
      </div>
    </div>
  );
};
export default memo(Banner);
