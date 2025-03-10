import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Fragment, memo, useCallback, useState } from "react";
import Download from "@/components/icon/download.svg";
import Warn from "@/components/icon/warn.svg";
import { Button } from "@/components/ui/button";
import pozk from "@/services/pozk";
import { Address } from "viem";
import { useFailedRoute } from "@/components/hooks/useFailedRoute";
import useProverStore from "@/components/state/proverStore";
import { useAccount } from "wagmi";
import sleep from "@/lib/sleep";
import Loading from '@/components/icon/loading.svg';

const Recommendation = ({ image, tag, name, overtime, ptype, types, needUpgrade }: { image: Address; tag: string, name: string, overtime: string, ptype: number, types: string, needUpgrade: boolean }) => {
  const [open, setOpen] = useState(false);
  const { address } = useAccount();
  const FailedRoute = useFailedRoute();
  const [loading, setLoading] = useState(false);
  const refetch = useProverStore(state => state.fetchUserProvers);

  const download = useCallback(async () => {
    setLoading(true);
    try {
      await pozk.pullProve(image, tag, name, overtime, ptype, types);
      let isDownloadCompleted = false;
      while (!isDownloadCompleted) {
        const containers = await pozk.getProverContainers(1);
        isDownloadCompleted = containers.some(container => [container.prover.toLowerCase(), container.tag].join('-') === [image.toLowerCase(), `v${tag}`].join('-'));
        if (!isDownloadCompleted) {
          await sleep(3);
        }
      }
      address && await refetch();
      setOpen(false);
    } catch (error) {
      FailedRoute(error);
    }
    setLoading(false);
  }, [image, tag, name, address]);

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <div
          className="flex flex-row items-center justify-center w-[28px] h-[28px] bg-[#A3E636] rounded-[6px]
                cursor-pointer
                opacity-100
                hover:opacity-80
                transition:opacity
                "
        >
          <Download className="stroke-0A1223" />
        </div>
      </DialogTrigger>
      <DialogContent className="w-[512px]">
        <DialogHeader>
          <DialogTitle>Recommendation</DialogTitle>
        </DialogHeader>
        <div className="flex flex-col justify-center items-center">
          <Warn className="mt-[32px] mb-[24px]" />
          <p className="text-[18px] font-light">
            The minimum configuration that allows this{" "}
          </p>
          <p className="text-[18px] font-light">
            Prove is <strong>4 cores 8G</strong>
          </p>
          <p className="text-[18px] font-light mt-[8px]">
            Too low a configuration may result in a loss{" "}
          </p>
          <p className="text-[18px] font-light">
            Of the number of staking tokens
          </p>
          <Button
            type="submit"
            className="h-[62px] w-[280px] mt-[48px] mb-[20px] bg-[#82c01e] rounded-[100px] text-[#0A1223] 
            font-light
            text-[20px]
            transition-background
            hover:bg-[hsl(hsl(83, 78%, 56%) / 0.9)]
            "
            disabled={loading}
            onClick={download}
          >
            {loading ? (
              <Fragment>
                <div className="flex items-center justify-center animate-spin mr-[8px]">
                  <Loading className='scale-y-[-1]' height={'20px'} width={'20px'} /> 
                </div>
                Downloading...
              </Fragment>
            ) : (
              <Fragment>
                <Download className="stroke-0A1223 mr-[8px]" />
                {needUpgrade ? 'Upgrade' : 'Download'}
              </Fragment>
            )}
          </Button>
        </div>
      </DialogContent>
    </Dialog>
  );
};
export default memo(Recommendation);
