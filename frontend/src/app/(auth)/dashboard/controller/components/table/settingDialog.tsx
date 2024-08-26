"use client";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { memo, useCallback, useMemo, useState } from "react";
import Warn from "@/components/icon/warn.svg";
import { Button } from "@/components/ui/button";
import { IAction } from "@/types/IAction";
import { usePostController } from "../../../hook/useController";
import { Address } from "viem";
import { Card } from "@/components/ui/card";
import { useToast } from "@/components/ui/use-toast";
import { FaCopy } from "react-icons/fa";
import useControllerStore from "@/components/state/controllerStore";
import sleep from "@/lib/sleep";
const SettingDialog = ({
  address,
  action,
  open,
  onOpenChange,
}: {
  address: Address;
  action: IAction;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}) => {
  const { toast } = useToast();
  const [loading, setLoading] = useState(false);
  const { setController, deleteController, exportController } =
    usePostController();

  const refetch = useControllerStore(state => state.fetch);
  const [key, setKey] = useState<string | undefined>("");
  const { title, content } = useMemo(() => {
    return action === "set"
      ? {
          title: "Set Controller?",
          content: "Are you sure you want to set this controller?",
        }
      : action === "delete"
      ? {
          title: "Delete Controller?",
          content: "Are you sure you want to delete this controller?",
        }
      : action === "export"
      ? {
          title: "Export Secret Key?",
          content:
            "Are you sure you want to export the secret key? This action cannot be undone.",
        }
      : {
          title: "",
          content: "",
        };
  }, [action]);
  const copyHandle = useCallback(() => {
    navigator.clipboard.writeText(key || "");
    toast({
      variant: "success",
      title: "Copy Success!",
    });
  }, [key]);
  const handle = useCallback(async () => {
    setLoading(true);
    let success: boolean = false;
    if (action === "set") {
      success = await setController(address);
    }
    if (action === "delete") {
      success = await deleteController();
    }
    if (action === "export") {
      const [result, secretKey] = await exportController(address);
      if (result) {
        setKey(secretKey);
      }
    }
    if (success) {
      onOpenChange(false);
    }
    console.log('rrrrrrrr fetch');
    await refetch(1);
    setLoading(false);
  }, [action]);

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="w-[512px]">
        <DialogHeader>
          <DialogTitle>{title}</DialogTitle>
        </DialogHeader>
        <div className="flex flex-col justify-center items-center">
          <Warn className="mt-[32px] mb-[18px]" />
          <p className="text-[#ffeb3b] bg-[#303511] rounded-sm py-[4px] px-[20px] text-center mb-[8px]">
            {address}
          </p>
          {key && action === "export" ? (
            <Card className="flex justify-center items-center gap-[10px] text-[#26d962] bg-[#093116] rounded-sm py-[10px] px-[20px] mt-[8px]">
              <p className="flex-8 break-words max-w-[340px]">{key}</p>
              <FaCopy className="flex-1" onClick={copyHandle} />
            </Card>
          ) : (
            <>
              <p className="text-[18px] font-light text-center">{content}</p>
              <Button
                type="submit"
                className="h-[48px] w-[280px] mt-[48px] mb-[20px]  rounded-[100px]
            text-[20px]
            "
                isLoading={loading}
                onClick={handle}
              >
                Confirm
              </Button>
            </>
          )}
        </div>
      </DialogContent>
    </Dialog>
  );
};
export default memo(SettingDialog);
