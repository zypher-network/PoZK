"use client";
import { FaCopy } from "react-icons/fa";
import { memo, useCallback, useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import { UserProver } from "@/types/IProver";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { Input } from "@/components/ui/input";
import { Card } from "@/components/ui/card";
import { CHAINID, contractAddress } from "@/web3/constants";
import { shortenWalletAddress } from "@/lib/shorten";
import { useToast } from "@/components/ui/use-toast";
import { usePostStake } from "../../../hook/useStake";
import BigNumberJs, { BM18 } from "@/lib/BigNumberJs";
import { useRecoilState, useRecoilValue } from "recoil";
import { StakeFormBtnLabel, StakeItem } from "../../../state/dashboardState";
import useBalanceStore from "@/components/state/balanceStore";
import { useShallow } from "zustand/react/shallow";
import useGetMinerStaking from "@/components/hooks/useGetMinerStaking";

const FormSchema = z.object({
  Amount: z.string().regex(/^\d+$/, {
    message: "Amount must only contain numbers",
  }),
});

const StakeForm = ({ item }: { item: UserProver }) => {
  const { stakeHandler } = usePostStake();
  const [btnLabel, setBtnLabel] = useRecoilState(StakeFormBtnLabel);
  const { payToken, minStake } = useBalanceStore(useShallow(state => ({ payToken: state.payToken, minStake: state.minStake })));
  const [loading, setLoading] = useState(false);
  const [amount, setAmount] = useState("");
  const stakeItem = useRecoilValue(StakeItem);
  const stakingAmount = useGetMinerStaking(item.id);
  const form = useForm<z.infer<typeof FormSchema>>({
    resolver: zodResolver(FormSchema),
    defaultValues: {
      Amount: "",
    },
  });
  useEffect(() => {
    setBtnLabel("Confirm");
    const inputAmount = new BigNumberJs(amount).times(BM18);
    if (stakeItem && stakeItem.key === "Stake") {
      const tokenAmount = new BigNumberJs(payToken.value);
      if (tokenAmount.gte(0) && tokenAmount.gte(inputAmount)) {
        if (new BigNumberJs(payToken.allowance).lt(inputAmount)) {
          setBtnLabel("Approve");
        }
      } else {
        if (amount !== "") {
          setBtnLabel("No Balance");
        } else {
          setBtnLabel("Confirm");
        }
      }
    } else {
      const tokenAmount = new BigNumberJs(stakingAmount);
      if (tokenAmount.lt(inputAmount)) {
        setBtnLabel("No Balance");
      }
    }
  }, [amount, item, setBtnLabel, stakeItem]);
  const onSubmit = useCallback(
    async (data: z.infer<typeof FormSchema>) => {
      setLoading(true);
      await stakeHandler(item.id, data.Amount);
      setLoading(false);
    },
    [item, stakeHandler]
  );
  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
        <Card className="p-[12px]">
          <Item
            fl={"Prover Address"}
            fr={shortenWalletAddress(item.id, "normal")}
            isCopy={true}
            copyMassage={item.id}
          />
          <Item fl={"Prover Name"} fr={item.name} />
          <Item fl={"Min Stake Amount"} fr={minStake.format} />
          <Item fl={"Staking"} fr={new BigNumberJs(stakingAmount).div(BM18).toFormat()} />
          <Item fl={"Is Miner"} fr={new BigNumberJs(stakingAmount).gte(minStake.value) ? "✅" : ""} />
          <Item
            fl={"Pay Token"}
            fr={shortenWalletAddress(contractAddress[CHAINID].Token, "normal")}
            isCopy={true}
            copyMassage={contractAddress[CHAINID].Token}
          />
          <Item
            fl={"Pay Token Balance"}
            fr={payToken.format}
          />
        </Card>
        <FormField
          control={form.control}
          name="Amount"
          render={({ field }) => (
            <FormItem>
              <FormLabel className="leading-[40px]">Amount</FormLabel>
              <FormControl>
                <Input
                  placeholder="Amount"
                  {...field}
                  onChange={(e) => {
                    setAmount(e.target.value);
                    field.onChange(e.target.value);
                  }}
                />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <div>
          <Button
            disabled={btnLabel === "No Balance" || loading}
            className="w-full font-normal my-[24px]"
            type="submit"
            isLoading={loading}
          >
            {btnLabel}
          </Button>
        </div>
      </form>
    </Form>
  );
};
const Item = ({
  fl,
  fr,
  isCopy,
  copyMassage,
}: {
  fl: string;
  fr: string;
  isCopy?: boolean;
  copyMassage?: string;
}) => {
  const { toast } = useToast();
  const copyHandle = useCallback(() => {
    navigator.clipboard.writeText(copyMassage || "");
    toast({
      variant: "success",
      title: "Copy Success!",
    });
  }, []);
  return (
    <div className="flex flex-row justify-between items-center gap-2 py-1">
      <FormLabel className="text-slate-400">{fl}:</FormLabel>
      <div className="flex flex-row justify-end items-center gap-2">
        <span>{fr}</span>
        {isCopy ? <FaCopy className="flex-1 cursor-pointer" onClick={copyHandle} /> : null}
      </div>
    </div>
  );
};
export default memo(StakeForm);
