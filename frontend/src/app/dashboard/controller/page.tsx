import { memo } from "react";
import generateMetadata from "@/lib/generateMetadata";
import ControllerPage from "./components/ControllerPage";
export const metadata = generateMetadata({ tit: "Controller" });
const Controller = () => {
  return <ControllerPage />;
};
export default memo(Controller);
