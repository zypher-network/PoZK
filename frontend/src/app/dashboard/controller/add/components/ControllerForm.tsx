"use client";
import { zodResolver } from "@hookform/resolvers/zod";
import { memo, useCallback, useMemo, useState } from "react";
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
import { useParams } from "next/navigation";
import { usePostController } from "../../../hook/useController";
import { IAction } from "@/types/IAction";
const ethereumSecretKeyRegex = /^0x[a-fA-F0-9]{64}$/;
const FormSchema = z.object({
  Controller: z
    .string()
    .length(66)
    .superRefine((val, ctx) => {
      if (!ethereumSecretKeyRegex.test(val)) {
        ctx.addIssue({
          code: z.ZodIssueCode.custom,
          message: "Invalid Ethereum Secret Key",
        });
      }
    }),
});

const ControllerForm = ({ action }: { action?: IAction }) => {
  const [loading, setLoading] = useState(false);
  const params = useParams();

  const { addController, setController } = usePostController();
  const Action = useMemo(() => {
    return action ?? (params.action?.toString() as IAction ?? '');
  }, [action, params.action]);

  const form = useForm<z.infer<typeof FormSchema>>({
    resolver: zodResolver(FormSchema),
    defaultValues: {
      Controller: "",
    },
  });

  // 54.245.48.42:9098 公网的IP
  const onSubmit = useCallback(
    async (data: z.infer<typeof FormSchema>) => {
      setLoading(true);

      if (Action === "add") {
        await addController(data.Controller as `0x${string}`);
      }
      if (Action === "edit") {
        await setController(data.Controller as `0x${string}`);
      }
      setLoading(false);
      // if (Action === "Delete") {
      //   await deleteController(data.Controller);
      // }
      // if (Action === "Edit") {
      //   await setController(data.Controller);
      // }
      form.reset();
      console.log({ data, Action });
    },
    [Action, addController]
  );
  return (
    <Form {...form}>
      <form
        onSubmit={form.handleSubmit(onSubmit)}
        className="w-[640px] space-y-6"
      >
        <FormField
          control={form.control}
          name="Controller"
          render={({ field }) => (
            <FormItem>
              <FormLabel className="leading-[40px]">Controller</FormLabel>
              <FormControl>
                <Input placeholder="Controller Private Key" {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <div>
          <Button
            className="w-full font-normal my-[24px]"
            type="submit"
            isLoading={loading}
          >
            Confirm
          </Button>
        </div>
      </form>
    </Form>
  );
};
export default memo(ControllerForm);
