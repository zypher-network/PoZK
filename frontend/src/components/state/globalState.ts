import { atom } from "recoil";
import { localStorageEffect } from "../ui/localStorageEffect";

export const FailedRoute = atom<string | undefined>({
  key: "FailedRoute",
  default: undefined,
  effects_UNSTABLE: [localStorageEffect("FailedRoute")],
});
