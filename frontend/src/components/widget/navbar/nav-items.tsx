"use client";
import Link from "next/link";
import { INavName, NavItem } from "./navbar";
import cx from 'classnames';

import { memo, useMemo } from "react";
import Dashboard from "@/components/icon/dashboard-default.svg";
import DashboardOn from "@/components/icon/dashboard-active.svg";
import Rewards from "@/components/icon/rewards-default.svg";
import RewardsOn from "@/components/icon/rewards-active.svg";
import MinerGame from '@/components/icon/miner-game-default.svg';
import MinerGameOn from '@/components/icon/miner-game-active.svg';
import Referral from "@/components/icon/Referral.svg";
import ReferralOn from "@/components/icon/Referral-on.svg";
import Setting from "@/components/icon/Setting.svg";
import SettingOn from "@/components/icon/Setting-on.svg";

import { usePathname } from "next/navigation";

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
  const pathname = usePathname();
  const { href, name } = item;
  // console.log(pathname, item);
  // const active = false;
  const active = useMemo(() => {
    return pathname === item.href;
  }, [pathname, item]);
  return (
    <Link
      key={href + name}
      href={href}
      className={cx(
        'w-full items-center justify-start flex gap-[16px] text-xl',
        {
          'text-white font-semibold': active,
          'text-[#626977] font-light hover:text-white': !active,
        }
      )}
    >
      <div className="w-[36px]">
        <MemoizedNavIcon name={name} active={active} />
      </div>
      <p>
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
    case "Miner Game":
      if (active) {
        return <MinerGameOn />
      }
      return <MinerGame />
    default:
      return <Dashboard />;
  }
};
const MemoizedNavIcon = memo(NavIcon);

export default NavItems;
