"use client";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { memo, useCallback, useEffect, useMemo, useState } from "react";
import { UserProver } from "@/types/IProver";
import { StakeItem } from "../../../state/dashboardState";
import { useRecoilState } from "recoil";
import StakeForm from "./StakeForm";

const StakeDialog = () => {
  const [title, setTitle] = useState("");
  const [open, setOpen] = useState(false);
  const [stakeItem, setStakeItem] = useRecoilState(StakeItem);
  useEffect(() => {
    if (stakeItem) {
      setTitle(stakeItem.key);
      setOpen(true);
    } else {
      setOpen(false);
    }
  }, [stakeItem]);
  const item: UserProver | undefined = useMemo(() => {
    return stakeItem?.item;
  }, [stakeItem?.item]);
  const onOpenChange = useCallback(
    (show: boolean) => {
      if (!show) {
        setStakeItem(undefined);
      }
    },
    [setStakeItem]
  );
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{title}</DialogTitle>
        </DialogHeader>
        {!!item && <StakeForm item={item} />}
      </DialogContent>
    </Dialog>
  );
};
export default memo(StakeDialog);
