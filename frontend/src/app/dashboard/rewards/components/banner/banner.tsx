"use client";
import { FC, memo, SVGProps, useEffect, useMemo, useState } from "react";
import cx from 'classnames';
import { Card } from "@/components/ui/card";
import Wallet from "@/components/icon/wallet.svg";
import KChat from "@/components/icon/kchat.svg";
import Gold from "@/components/icon/gold.svg";
import { cn } from "@/lib/utils";
import { useAccount } from "wagmi";
import BigNumberJs, { BM18 } from "@/lib/BigNumberJs";
import useSubgraphStore from "@/components/state/subgraphStore";
import Loading from "@/components/ui/loading";
import { Button } from "@/components/ui/button";
import ContractService from "@/web3/contract/contract";
import { CHAINID, contractAddress } from "@/web3/constants";
import StakeAbi from '@/web3/contract/abi/Stake.json';

const Banner = () => {
  const { address } = useAccount();
  const { data, pending } = useSubgraphStore(state => state.reward);
  const claimedAmount = useSubgraphStore(state => state.claimedAmount);
  const reset = useSubgraphStore(state => state.reset);
  const [claimable, setClaimable] = useState('0');

  const rewardInfo = useMemo(() => {
    // const totalRewards = new BigNumberJs(data?.totalClaim ?? '0').div(BM18).toString();
    let collecteddRewards = new BigNumberJs('0');
    let totalRewards = new BigNumberJs('0');
    for (const claim of (data?.claimList ?? [])) {
      collecteddRewards = collecteddRewards.plus(claim.claim ?? '0');
      totalRewards = totalRewards.plus(claim.claim ?? claim.estimate ?? '0');
      // claimedRewards = new BigNumberJs(claim.claim ?? '0').div(BM18).plus(claimedRewards).toString(10);
    }
    return [
      {
        title: "Total Rewards Estimate",
        value: totalRewards.div(BM18).toFormat(),
        Icon: Wallet,
        borderLeftClassName: "bg-[#FACC16]",
        className: "bg-gradient-to-b from-[#9277FD] to-[#674EFF]",
      },
      {
        title: "Collected Rewards",
        value: claimable,
        Icon: KChat,
        borderLeftClassName: "bg-[#BF38FF]",
        className: "bg-gradient-to-b from-[#C8D254] to-[#71A61A]",
      },
      {
        title: "Claimed Rewards",
        value: claimedAmount.data,
        Icon: Gold,
        borderLeftClassName: "bg-[#3B5AFF]",
        className: "bg-gradient-to-b from-[#E7C56D] to-[#E18802]",
      },
    ];
  }, [address, data, claimable, claimedAmount]);

  const fetchClaimableReward = async () => {
    setClaimable('0');
    try {
      const contract = new ContractService(contractAddress[CHAINID].Stake, StakeAbi);
      const rawClaimable = await contract.readContractData('claimable', [address]) as unknown as bigint;
      setClaimable(new BigNumberJs(rawClaimable.toString()).div(BM18).toFormat());
    } catch (error) {
      setClaimable('0');
    }
  }

  const handleClaim = async () => {
    try {
      const contract = new ContractService(contractAddress[CHAINID].Stake, StakeAbi);
      await contract.writeContractMethod('claim', [address]);
      reset('reward');
    } catch (error) {
      console.log(error);
    }
  }

  useEffect(() => {
    if (data && address) {
      fetchClaimableReward();
    }
  }, [data, address]);

  return (
    <div className="flex justify-between items-stretch gap-[24px]">
      {rewardInfo.map((v) => <Item item={v} key={v.title} pending={pending} onClaim={handleClaim} />)}
    </div>
  );
};
type IItem = {
  title: string;
  value: string;
  Icon: FC<SVGProps<SVGElement>>;
  borderLeftClassName?: string;
  className?: string;
};
const Item = ({ item, pending, onClaim }: { item: IItem, pending: boolean; onClaim?: () => Promise<void> }) => {
  const { title, value, Icon, className, borderLeftClassName } = item;

  return (
    <Card
      className={cn("basis-1/3 relative min-h-[184px] pl-[50px]", className)}
    >
      <p className="font-light text-[20px] text-nowrap  pt-[10px]">{title}</p>
      <h3 className="text-[44px] font-semibold text-nowrap pt-[10px]">
        {pending ? <Loading /> : value}
      </h3>
      <div className={cx('mt-3 flex justify-end', { 'hidden pointer-events-none': !['Collected Rewards'].includes(title) || value === '0' })}>
        <Button onClick={() => onClaim?.()}>Claim Rewards</Button>
      </div>
      <Icon
        className="absolute
          top-[56px] 
       right-[10%] size-[72px]"
      />
      <div
        className={cn(
          "w-[8px] h-[88px] absolute top-[48px] left-0 z-10 rounded-r-[8px]",
          borderLeftClassName
        )}
      />
    </Card>
  );
};
export default memo(Banner);
