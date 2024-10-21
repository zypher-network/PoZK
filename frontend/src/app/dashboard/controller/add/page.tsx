import { memo } from "react";
import generateMetadata from "@/lib/generateMetadata";
import ControllerPage from "./components/ControllerPage";
export const metadata = generateMetadata({ tit: "Setting Controller" });

const ControllerAction = () => {
  return <ControllerPage />;
};
export default memo(ControllerAction);
