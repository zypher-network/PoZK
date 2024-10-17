import { memo } from "react";
import generateMetadata from "@/lib/generateMetadata";
import ControllerPage from "./ControllerPage";
export const metadata = generateMetadata({ tit: "Setting Controller" });

export async function generateStaticParams() {
  return [
    { action: 'add' },
  ];
}

const ControllerAction = () => {
  return <ControllerPage />;
};
export default memo(ControllerAction);
