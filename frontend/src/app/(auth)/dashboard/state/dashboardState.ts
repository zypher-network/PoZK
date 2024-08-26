"use client";
import { IGameConfig, UserProver } from "@/types/IProver";
import { atom } from "recoil";
import { Address } from "viem";

export const StakeFormBtnLabel = atom({
  key: "StakeFormBtnLabel",
  default: "Confirm",
});
export const StakeItem = atom<
  | {
      key: "Stake" | "UnStake";
      item: UserProver;
    }
  | undefined
>({
  key: "StakeItem",
  default: undefined,
  // effects_UNSTABLE: [localStorageEffect("GameConfig")],
});
// 完全无关的
export const GameConfig = atom<IGameConfig[]>({
  key: "GameConfig",
  default: [],
  // effects_UNSTABLE: [localStorageEffect("GameConfig")],
});
// 已 stake 未安装
export const StakedGameConfig = atom<IGameConfig[]>({
  key: "StakedGameConfig",
  default: [],
  // effects_UNSTABLE: [localStorageEffect("GameConfig")],
});
export type IControllerItem = {
  status: "on" | "off";
  address: Address;
};
export const ControllerList = atom<{
  isLoading: boolean;
  hasOnController: boolean;
  list: IControllerItem[];
}>({
  key: "ControllerList",
  default: {
    isLoading: true,
    hasOnController: false,
    list: [],
  },
});
