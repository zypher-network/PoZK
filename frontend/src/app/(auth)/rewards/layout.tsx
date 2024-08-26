import generateMetadata from "@/lib/generateMetadata";
import { ReactNode } from "react";
export const metadata = generateMetadata({ tit: "Rewards" });
export default function Rewards({ children }: { children: ReactNode }) {
  return <>{children}</>;
}
