"use client";
import { useToast } from "@/components/ui/use-toast";

import { useCallback } from "react";
import { useSetRecoilState } from "recoil";
import { FailedRoute } from "../state/globalState";
import { usePathname, useRouter } from "next/navigation";
import { useSession } from "./useSession";

export const useFailedRoute = () => {
  const { toast } = useToast();
  const route = useRouter();
  const pathname = usePathname();
  const setFailedRoute = useSetRecoilState(FailedRoute);
  const { deleteToken } = useSession();
  const Failed = useCallback(
    async (error: any) => {
      const { code, message } = error || {};
      if (message.indexOf("HTTP error 5") === -1) {
        toast({
          variant: "destructive",
          title: code ?? "Failed",
          description: <p className="text-[14px]">{message ?? "Failed"}</p>,
        });
      }
      if (message === "HTTP error 401") {
        // const local: any = await api.post("/logout", undefined, {
        //   useUrl: true,
        // });
        // if (local) {
        deleteToken();
        setFailedRoute(pathname);
        route.push("/");
        // }
      }
    },
    [toast]
  );
  return Failed;
};
