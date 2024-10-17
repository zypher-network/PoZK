"use client";
import { memo, useMemo, useState } from "react";
import Wifi from "@/components/icon/wifi.svg";
import Copy from "@/components/icon/copy.svg";
import { useToast } from "@/components/ui/use-toast";
import { FaCheck } from "react-icons/fa";
import { Card, CardTitle } from "@/components/ui/card";
import { shortenWalletAddress } from "@/lib/shorten";
import { useAccount } from "wagmi";
import BigNumberJs, { BM18 } from "@/lib/BigNumberJs";
import useSubgraphStore from "@/components/state/subgraphStore";
import { useShallow } from "zustand/react/shallow";
import { isTodaysEpoch } from "@/lib/day";
const Banner = () => {
  const { staking, reward, epoches } = useSubgraphStore(useShallow(state => ({ staking: state.staking, epoches: state.epoches, reward: state.reward })));

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
          max-w-max
          mt-[32px] flex justify-between items-start
          relative
          z-10"
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
        </ul>
        <img
          className="absolute bottom-0 right-0"
          src="/dashboard/banner_earning.png"
          width={180}
          height={180}
          alt="dashboard"
        />
      </Card>
      <Card className="flex flex-col justify-between basis-1/3">
        <CardTitle className="border-b-[1px] pb-[28px]border-[#1F2D4E] pb-[12px]">
          Network State
        </CardTitle>
        <div
          className="
          border
          border-[#674DFF]
          bg-[#674DFF]
          bg-opacity-30
          rounded-[100px]
          flex
          justify-between
          items-center
          px-[28px]
          h-[48px]
          my-[16px]
        "
        >
          <Wifi />
          <div className="flex justify-center gap-[10px] items-center">
            <p className="font-light text-[20px]">Connected</p>
            <div className="bg-[#A3E636] w-[8px] h-[8px] rounded-[50%] shadow-[0_0_0_4px_rgba(163,230,54,0.3)]" />
          </div>
          <div />
        </div>

        <Address />
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
        <p className="font-light text-[16px] opacity-50 text-nowrap">{label}</p>
        <h5 className="text-[20px] font-medium text-nowrap">{value}</h5>
      </li>
      {isLast ? null : (
        <li
          className="
          min-w-[1px]
          h-[44px]
      mx-[40px]
      bg-[#fff] opacity-10"
        />
      )}
    </>
  );
};
const Address = () => {
  const { address } = useAccount();
  const [showCopy, setShowCopy] = useState<boolean>(false);
  const { toast } = useToast();
  const handleCopyClick = () => {
    toast({
      title: "Copied to clipboard!",
      description: "address",
    });
    setShowCopy(true);
    setTimeout(() => {
      setShowCopy(false);
    }, 3000);
  };
  const shorten = useMemo(() => {
    return address ? shortenWalletAddress(address, "normal") : "";
  }, [address]);
  return (
    <div
      className="flex justify-between 
      items-center
      rounded-[10px]
      bg-[#0A1223]
          px-[28px]
          py-[20px]"
    >
      <div className="flex items-center">
        <p className="text-[18px] text-nowrap">wallet address:</p>
        <p className="font-light text-[18px] pl-[12px] pr-[12px]">{shorten}</p>
      </div>

      {showCopy ? <FaCheck /> : <Copy onClick={handleCopyClick} />}
    </div>
  );
};
export default memo(Banner);
