"use client";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import Back from "@/components/widget/back/back";
import { memo, useCallback, useMemo, useState } from "react";
import { usePostController } from "../../hook/useController";
import { useParams } from "next/navigation";
import { IAction } from "@/types/IAction";
import { Button } from "@/components/ui/button";
import ControllerForm from "./ControllerForm";
import ControllerTable from "../components/table/ControllerTable";

const ControllerPage = () => {
  const [loading, setLoading] = useState(false);
  const params = useParams();
  const { newController } = usePostController();
  const Action = useMemo(() => {
    return params.action.toString() as IAction;
  }, [params.action]);
  const Title = useMemo(() => {
    return `${Action.charAt(0).toUpperCase()}${Action.slice(1)} Controller`;
  }, [Action]);
  const createHandle = useCallback(async () => {
    setLoading(true);
    await newController();
    setLoading(false);
  }, [newController]);

  return (
    <>
      <Card className="mb-[48px]">
        <CardHeader className="flex flex-row justify-between items-center pb-[48px]">
          <CardTitle className="flex items-center">
            <Back />
            {Title}
          </CardTitle>
        </CardHeader>
        <Tabs defaultValue="New Secret Key" className="w-full max-w-[700px]">
          <TabsList className="grid w-full grid-cols-2">
            <TabsTrigger value="New Secret Key">New Secret Key</TabsTrigger>
            <TabsTrigger value="Add Secret Key">Add Secret Key</TabsTrigger>
          </TabsList>
          <TabsContent value="New Secret Key" className="mt-[40px]">
            <Card className="flex justify-center items-center">
              <Button
                className="w-[200px]"
                onClick={createHandle}
                isLoading={loading}
                disabled={loading}
              >
                Create
              </Button>
            </Card>
          </TabsContent>
          <TabsContent value="Add Secret Key">
            <Card className="flex justify-center items-center">
              <ControllerForm />
            </Card>
          </TabsContent>
        </Tabs>
        <ControllerTable />
      </Card>
    </>
  );
};
export default memo(ControllerPage);
