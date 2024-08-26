"use client";
import { IAction } from "@/types/IAction";
import { useRouter } from "next/navigation";
import { useCallback } from "react";
// /dashboard/setting
export const useToSettingPath = (prePath: string) => {
  const router = useRouter();
  return useCallback(({ action }: { action: IAction }) => {
    const actionString = encodeURIComponent(String(action).toLowerCase());
    router.push(`${prePath}/${actionString}`);
  }, []);
};
