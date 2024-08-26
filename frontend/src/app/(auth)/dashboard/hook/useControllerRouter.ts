"use client";
import { useLayoutEffect } from "react";
// import { useRecoilValue } from "recoil";
import { useShallow } from 'zustand/react/shallow'
import { usePathname, useRouter } from "next/navigation";
import useControllerStore from "@/components/state/controllerStore";
// import { ControllerList } from "../state/dashboardState";

export const useControllerRouter = () => {
  const router = useRouter();
  const store = useControllerStore(
    useShallow(state => ({
      fetching: state.fetching,
      active: state.active,
      controllers: state.controllers,
    }))
  );
  // const controllerList = useRecoilValue(ControllerList);
  const pathname = usePathname();
  // console.log({ controllerList });
  useLayoutEffect(() => {
    if (!store.fetching) {
      if (!store.controllers.length) {
        if (pathname !== "/dashboard/controller/add") {
          router.push("/dashboard/controller/add");
        }
      } else {
        if (pathname !== "/dashboard/controller" && !store.active) {
          router.push("/dashboard/controller");
        }
      }
    }
  }, [store]);
};
