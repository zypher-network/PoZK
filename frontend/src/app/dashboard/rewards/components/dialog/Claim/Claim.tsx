"use client";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { memo, useCallback, useState } from "react";
import { Button } from "@/components/ui/button";
import useEpochStore from "@/components/state/epochStore";
import BigNumberJs, { BM18 } from "@/lib/BigNumberJs";
import { useAccount } from "wagmi";
import { useShallow } from "zustand/react/shallow";
import useSubgraphStore from "@/components/state/subgraphStore";
import { RewardContract } from "@/web3/contract/RewardContract";

const Claim = () => {
  const [isLoading, setisLoading] = useState(false);
  const reset = useSubgraphStore(state => state.reset);
  const { epoch, setEpoch } = useEpochStore(useShallow(state => ({ epoch: state.selected, setEpoch: state.setSelectEpoch })))
  const { address } = useAccount();

  const claimHandle = useCallback(async () => {
    if (epoch) {
      setisLoading(true);
      try {
        const contract = RewardContract();
        await contract.writeContractMethod('minerBatchCollect', [BigInt(epoch.id), address]);
        reset('reward');
        reset('epoches');
        setEpoch(null);
      } catch (error) {
        console.log(error);
      } finally {
        setisLoading(false);
      }
    }
  }, [epoch, address]);
  const cancelHandle = useCallback(() => {
    setEpoch(null);
  }, []);
  return (
    <Dialog open={Boolean(epoch)} onOpenChange={cancelHandle}>
      <DialogContent className="w-[512px] fixed">
        <DialogHeader>
          <DialogTitle>{`Epoch ${epoch?.epoch}`}</DialogTitle>
        </DialogHeader>
        <div className="flex flex-col justify-center items-center mt-[60px] z-10">
          <p className="text-[20px] font-light">Estimated Reward Points</p>
          <h3 className="text-[48px] font-medium">{new BigNumberJs(epoch?.estimate ?? '0').div(BM18).toFormat()}</h3>
          <Button
            type="submit"
            variant={"default"}
            className="h-[62px] w-[212px] 
             mt-[60px] mb-[20px] 
             rounded-[100px] 
            font-light
            text-[20px]
            "
            isLoading={isLoading}
            disabled={isLoading}
            onClick={claimHandle}
          >
            Collect
          </Button>
        </div>
        <img
          className="absolute top-[40px] left-0 w-full"
          src="/rewards/claim_bg.png"
          alt="dashboard"
          width={512}
          height={248}
        />
      </DialogContent>
    </Dialog>
  );
};
export default memo(Claim);
