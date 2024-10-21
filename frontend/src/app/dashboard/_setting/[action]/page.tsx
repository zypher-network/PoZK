import { memo } from "react";
import FormComp from "./components/form";
import generateMetadata from "@/lib/generateMetadata";
export const metadata = generateMetadata({ tit: "Networks Setting" });

export async function generateStaticParams() {
  return [
    { action: 'add' },
  ];
}

const SettingAction = () => {
  return <FormComp />;
};

export default memo(SettingAction);
