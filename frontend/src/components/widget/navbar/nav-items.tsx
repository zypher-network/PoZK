"use client";
import Link from "next/link";
import { INavName, NavItem, useLocation } from "./navbar";

import { FC, memo, SVGProps, useMemo } from "react";
import Dashboard from "@/components/icon/Dashboard.svg";
import DashboardOn from "@/components/icon/Dashboard-on.svg";
import Rewards from "@/components/icon/Rewards.svg";
import RewardsOn from "@/components/icon/Rewards-on.svg";
import Referral from "@/components/icon/Referral.svg";
import ReferralOn from "@/components/icon/Referral-on.svg";
import Setting from "@/components/icon/Setting.svg";
import SettingOn from "@/components/icon/Setting-on.svg";

const NavItems = ({ navItems }: { navItems: NavItem[] }) => {
  return (
    <>
      {navItems.map((item) => (
        <MemoizedItem key={item.name} item={item} />
      ))}
    </>
  );
};

const Item = ({ item }: { item: NavItem }) => {
  const pathname = useLocation();
  const { href, name } = item;
  const active = useMemo(() => {
    return pathname === href;
  }, [pathname, href]);
  return (
    <Link
      key={href + name}
      href={href}
      className="w-full items-center justify-start flex gap-[16px]"
    >
      <div className="w-[36px]">
        <MemoizedNavIcon name={name} active={active} />
      </div>
      <p
        className={`${
          active ? "text-white font-semibold" : "text-[#626977] font-normal"
        }`}
      >
        {name}
      </p>
    </Link>
  );
};
const MemoizedItem = memo(Item);

const NavIcon = ({ name, active }: { name: INavName; active: boolean }) => {
  switch (name) {
    case "Dashboard":
      if (active) {
        return <DashboardOn />;
      }
      return <Dashboard />;
    case "Referral":
      if (active) {
        return <ReferralOn />;
      }
      return <Referral />;
    case "Rewards":
      if (active) {
        return <RewardsOn />;
      }
      return <Rewards />;
    case "Setting":
      if (active) {
        return <SettingOn />;
      }
      return <Setting />;
    default:
      return <Dashboard />;
  }
};
const MemoizedNavIcon = memo(NavIcon);

export default NavItems;
