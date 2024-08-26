"use client";
import { useRecoilState } from "recoil";
import { useCallback, useLayoutEffect, useMemo } from "react";
import api from "@/lib/fetch";
import { ControllerList, IControllerItem } from "../state/dashboardState";
import { Address } from "viem";
import { ControllerContract } from "@/web3/contract/ControllerContract";
import { useToast } from "@/components/ui/use-toast";
import { evmWallet } from "@/web3/wallet";
import { useFailedRoute } from "@/components/hooks/useFailedRoute";
import pozk from "@/services/pozk";
import useControllerStore from "@/components/state/controllerStore";
const useGetData = () => {
  const Failed = useFailedRoute();
  const [controllerList, setControllerList] = useRecoilState(ControllerList);
  const getData = useCallback(async () => {
    try {
      if (!controllerList.list.length) {
        const res: any = await api.get(
          // page_count = 页数
          // page_size = 页面大小
          "/controller/list?page_size=100&page_count=1"
        );
        console.log({ res });
        if (res && res.data.data.length) {
          // 获取active
          let active: undefined | Address = undefined;
          try {
            const resSet: any = await api.get("/controller/set");
            if (resSet && resSet.data && resSet.data.controller) {
              active = resSet.data.controller.toLowerCase();
            }
          } catch (e: any) {}
          const data = res.data.data.map((v: Address) => ({
            address: v,
            status: active === v.toLowerCase() ? "on" : "off",
          })) as IControllerItem[];
          setControllerList({
            isLoading: false,
            hasOnController: !!active,
            list: data.sort((a, b) => {
              if (a.status === "on" && b.status !== "on") {
                return -1; // a 在前面
              } else if (a.status !== "on" && b.status === "on") {
                return 1; // b 在前面
              } else {
                return 0; // 相等
              }
            }),
          });
        } else {
          throw Error("No data");
        }
      }
    } catch (err: any) {
      Failed(err);
      setControllerList({
        isLoading: false,
        hasOnController: false,
        list: [],
      });
    }
  }, [controllerList.list.length, setControllerList]);
  return { getData };
};
export const useController = () => {
  const { getData } = useGetData();
  useLayoutEffect(() => {
    getData();
  }, []);
};
export const usePostController = () => {
  const { getData } = useGetData();
  const { toast } = useToast();
  const refetch = useControllerStore(state => state.fetch);
  const Failed = useFailedRoute();
  const contract = useMemo(() => {
    return ControllerContract();
  }, []);
  const contractAdd = useCallback(
    async (controller: Address) => {
      const contractRes = await contract.writeContractMethod("add", [
        controller,
      ]);
      console.log({ contractRes });
      getData();
    },
    [contract, getData]
  );

  const newController = useCallback(async () => {
    try {
      const controller = await pozk.newController();
      if (controller) {
        await contractAdd(controller);
        toast({
          title: "Create New Secret Key Success",
          variant: "success",
        });
        refetch(1);
      } else {
        throw Error("Controller new Error");
      }
    } catch (error: any) {
      await Failed(error);
    }
  }, [contractAdd, toast, Failed]);
  const addController = useCallback(
    async (secretKey: `0x${string}`) => {
      try {
        const res: any = await api.post("/controller/add", {
          signing_key: secretKey,
        });
        if (res.code === 0) {
          const controller = evmWallet.getAccount(secretKey);
          await contractAdd(controller);
          toast({
            title: "Add New Secret Key Success",
            variant: "success",
          });
          getData();
        } else {
          throw Error(res ?? "Controller add Error");
        }
      } catch (error: any) {
        await Failed(error);
      }
    },
    [contractAdd, toast, Failed]
  );

  const setController = useCallback(
    async (address: Address): Promise<boolean> => {
      try {
        const res: any = await api.post(
          `/controller/set/${address}`,
          undefined
        );
        if (res.code === 0) {
          toast({
            title: "Set controller Success",
            variant: "success",
          });
          getData();
          return true;
        } else {
          throw Error(res ?? "Controller set Error");
        }
      } catch (error: any) {
        await Failed(error);
        return false;
      }
    },
    []
  );
  const deleteController = useCallback((): boolean => {
    return false;
  }, []);
  const exportController = useCallback(
    async (address: Address): Promise<[boolean, string]> => {
      try {
        const res: any = await api.get(
          `/controller/export/${address}`,
          undefined
        );
        console.log({ res });
        if (res.code === 0) {
          // toast({
          //   title: "Set controller Success",
          //   variant: "success",
          // });
          // getData();
          return [true, res.data.singing_key];
        } else {
          throw Error(res ?? "Controller set Error");
        }
      } catch (error: any) {
        await Failed(error);
        return [false, ""];
      }
    },
    []
  );
  return {
    newController,
    addController,
    setController,
    deleteController,
    exportController,
  };
};
