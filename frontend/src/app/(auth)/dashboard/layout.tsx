import generateMetadata from "@/lib/generateMetadata";
import { ReactNode } from "react";
export const metadata = generateMetadata({ tit: "Dashboard" });
export default function Dashboard({ children }: { children: ReactNode }) {
  return <>{children}</>;
}
