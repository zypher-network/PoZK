"use client";
import Link from "next/link";
import { appName } from "@/constants/constants";
import NavItems from "./nav-items";
import { memo, useMemo } from "react";
import { usePathname } from "next/navigation";
export type INavName = "Dashboard" | "Rewards" | "Referral" | "Setting" | "Miner Game";
export type NavItem = {
  name: INavName;
  href: string;
  description: string;
};
export const navigation: NavItem[] = [
  {
    name: "Miner Game",
    href: "/dashboard/miner-game",
    description: "Miner Game",
  },
  {
    name: "Dashboard",
    href: "/dashboard",
    description: "Dashboard",
  },
  {
    name: "Rewards",
    href: "/dashboard/rewards",
    description: "Rewards",
  },
  // {
  //   name: "Referral",
  //   href: "/referral",
  //   description: "referral",
  // },
  // {
  //   name: "Setting",
  //   href: "/setting",
  //   description: "Setting",
  // },
];

const getParentPath = (path: string) => {
  const arrayNamePath = path.split("/");
  return `/${arrayNamePath[1]}`;
};
export const useLocation = () => {
  const pathname = usePathname();
  const path = useMemo(() => {
    return getParentPath(pathname);
  }, [pathname]);
  return path;
};
const Location = ({ titleProps }: { titleProps?: string }) => {
  const pathname = usePathname();
  const title = useMemo(() => {
    try {
      return (
        titleProps ?? navigation.filter((v) => v.href === `${pathname}`)[0].name
      );
    } catch (err) {
      return "   ";
    }
  }, [pathname, titleProps]);
  return <h3 className="text-[28px] font-bold">{title}</h3>;
};
export const MemoizedLocation = memo(Location);
const Navbar = () => {
  return (
    <div className="overflow-show pl-[48px] pt-[48px]">
      <Link href="/dashboard">
        <img src="/nav/logo_dark.png" width={185} height={40} alt={appName} />
      </Link>
      <nav className="pt-[48px] flex gap-[40px] flex-col">
        <NavItems navItems={navigation} />
      </nav>
    </div>
  );
};
export default Navbar;
