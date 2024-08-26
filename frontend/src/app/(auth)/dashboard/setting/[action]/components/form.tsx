"use client";
import { zodResolver } from "@hookform/resolvers/zod";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { memo, useLayoutEffect, useMemo } from "react";
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
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { toast } from "@/components/ui/use-toast";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { useAccount } from "wagmi";
import { useParams } from "next/navigation";
import Back from "@/components/widget/back/back";
import { useRecoilValue } from "recoil";
import { GameConfig } from "../../../state/dashboardState";
const FormSchema = z.object({
  "URL / IP": z.string().min(2, {
    message: "URL / IP must be at least 2 characters.",
  }),
  Prover: z.string().min(2, {
    message: "Prover must be at least 2 characters.",
  }),
  Controller: z.string().min(2, {
    message: "Controller must be at least 2 characters.",
  }),
  "Staking amount": z.string().min(2, {
    message: "Staking amount must be at least 2 characters.",
  }),
});

const FormComp = () => {
  const params = useParams();
  const Title = useMemo(() => {
    const action = params.action.toString();
    return `${action.charAt(0).toUpperCase()}${action.slice(1)} Device`;
  }, [params.action]);
  const { address } = useAccount();
  const gameConfig = useRecoilValue(GameConfig);
  const form = useForm<z.infer<typeof FormSchema>>({
    resolver: zodResolver(FormSchema),
    defaultValues: {
      "URL / IP": "",
      Prover: "",
      Controller: "",
      "Staking amount": "",
    },
  });
  useLayoutEffect(() => {
    if (gameConfig.length) {
      console.log({ aaa: gameConfig[0].id });
      form.reset({
        "URL / IP": "",
        Prover: gameConfig[0].id,
        Controller: address?.toString(),
        "Staking amount": gameConfig[0].minStakeAmount,
      });
    }
  }, [gameConfig.length, form, address]);

  // 54.245.48.42:9098 公网的IP
  function onSubmit(data: z.infer<typeof FormSchema>) {
    toast({
      title: "You submitted the following values:",
      description: (
        <pre className="mt-2 w-[340px] rounded-md bg-slate-950 p-4">
          <code className="text-white">{JSON.stringify(data, null, 2)}</code>
        </pre>
      ),
    });
  }
  return (
    <Card className="mb-[48px]">
      <CardHeader className="flex flex-row justify-between items-center pb-[48px]">
        <CardTitle className="flex items-center">
          <Back />
          {Title}
        </CardTitle>
      </CardHeader>
      <CardContent className="flex justify-center items-center ">
        <Form {...form}>
          <form
            onSubmit={form.handleSubmit(onSubmit)}
            className="w-[640px] space-y-6"
          >
            <FormField
              control={form.control}
              name="URL / IP"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>URL / IP</FormLabel>
                  <FormControl>
                    <Input placeholder="URL / IP" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="Prover"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Prover</FormLabel>
                  <Select
                    onValueChange={field.onChange}
                    value={field.value}
                    defaultValue={field.value}
                  >
                    <FormControl>
                      <SelectTrigger>
                        <SelectValue placeholder="Prover" />
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      {gameConfig.map((v) => (
                        <SelectItem value={v.id} key={v.id}>
                          {v.name}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="Controller"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Controller</FormLabel>
                  <FormControl>
                    <Input placeholder="Controller" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="Staking amount"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Staking amount</FormLabel>
                  <FormControl>
                    <Input placeholder="Staking amount" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <div>
              <Button
                className="w-full h-[62px] rounded-[100px] text-[20px] font-normal my-[24px]"
                type="submit"
              >
                Confirm
              </Button>
            </div>
          </form>
        </Form>
      </CardContent>
    </Card>
  );
};
export default memo(FormComp);
