import React, { useState } from 'react'
import cx from 'classnames'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Button } from '@/components/ui/button';
import { StakeContract } from "@/web3/contract/StakeContract";
import { waitForTransactionReceipt } from "wagmi/actions";
import { wagmiConfig } from "@/web3/wagmi.config";
import { useToast } from "@/components/ui/use-toast";

interface IMinerTestTipDialog {
  open: boolean;
  type: 'retry' | 'cancel' | '';
  testId: string;
  onClose: () => void;
}

const HeaderText = {
  retry: 'Miner Hardware Testing',
  cancel: 'Cancel Miner Testing',
} as const;

const ContentText = {
  retry: (
    <div>Do you want to retry the testing?</div>
  ),
  cancel: (
    <div className="flex flex-col w-full items-center justify-center">
      <div className="text-lg">Do you want to cancel the testing?</div>
      <div className="text-sm font-medium" style={{ color: '#9277fd' }}>
        *The staked tokens will be returned to you.
      </div>
    </div>
  ),
} as const;

const MinerTestTipDialog: React.FC<IMinerTestTipDialog> = ({ open, type, testId, onClose }) => {
  const { toast } = useToast();
  const [pending, setPending] = useState(false);

  const handleMinerTest = async () => {
    try {
      setPending(true);
      const contract = StakeContract();
      let hash: `0x${string}` | '' = '';
      let title = '';
      if (type === 'retry') {
        hash = await contract.writeContractMethod('minerTestSubmit', [
          testId,
          true,
          '0x',
        ]);
        title = 'Submission Successful';
      }
      if (type === 'cancel') {
        hash = await contract.writeContractMethod('minerTestCancel', [testId]);
        title = 'Cancellation Successful';
      }
      if (hash) {
        await waitForTransactionReceipt(wagmiConfig, { hash });
      }
      toast({
        variant: 'success',
        title,
      });
      onClose();
    } catch (error) {
      toast({
        variant: 'info',
        title: (error as Error).message,
      });
    } finally {
      setPending(false);
    }
  }

  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent autoFocus={false}>
        <DialogHeader>
          <DialogTitle>{type && HeaderText[type]}</DialogTitle>
        </DialogHeader>
        <div className="flex flex-col gap-6 w-full items-center">
          {type && ContentText[type]}
          <div className="flex">
            <Button
              type="submit"
              autoFocus={false}
              isLoading={pending}
              disabled={pending}
              className={cx(
                'w-[120px] h-[42px] text-[20px]',
                {
                  'bg-[#E44042] hover:bg-[#E44042]': type === 'cancel',
                  'bg-[#82c01e] text-[#0A1223] hover:bg-[#82c01e]': type === 'retry',
                }
              )}
              onClick={handleMinerTest}
            >
              Confirm
            </Button>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  )
};

export default MinerTestTipDialog;
