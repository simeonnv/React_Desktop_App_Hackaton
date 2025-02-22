"use client";
import React, { useState } from "react";
import { SidebarDemo, SidebarBody, SidebarLink } from "../ui/sidebarDemo";
import HomeSVG from "@/components/ui/HomeSVG";
import ChartSVG from "@/components/ui/ChartSVG";
import NotificationSVG from "@/components/ui/NotificationSVG";
import PlusSVG from "@/components/ui/PlusSVG";
import UserSVG from "@/components/ui/UserSVG";
import Link from "next/link";
import { motion } from "framer-motion";

import { cn } from "@/lib/utils";

export function Sidebar() {
  const links = [
    {
      label: "Dashboard",
      href: "#",
      icon: (
        <HomeSVG classes="text-neutral-700 dark:text-neutral-200 h-5 w-5 flex-shrink-0" />
      ),
    },
    {
      label: "Profile",
      href: "#",
      icon: (
        <NotificationSVG classes="text-neutral-700 dark:text-neutral-200 h-5 w-5 flex-shrink-0" />
      ),
    },
    {
      label: "Settings",
      href: "#",
      icon: (
        <ChartSVG classes="text-neutral-700 dark:text-neutral-200 h-5 w-5 flex-shrink-0" />
      ),
    },
    {
      label: "Logout",
      href: "#",
      icon: (
        <PlusSVG classes="text-neutral-700 dark:text-neutral-200 h-5 w-5 flex-shrink-0" />
      ),
    },
  ];
  const [open, setOpen] = useState(false);
  return (
    <div
      className={cn(
        "rounded-md flex flex-col md:flex-row bg-gray-100 dark:bg-neutral-800  flex-1 h-screen mx-auto border border-neutral-200 dark:border-neutral-700 overflow-hidden"
      )}
    >
      <SidebarDemo open={open} setOpen={setOpen}>
        <SidebarBody className="justify-between gap-10">
          <div className="flex flex-col flex-1 overflow-y-auto overflow-x-hidden">
            {open ? <Logo /> : <LogoIcon />}
            <div className="mt-8 flex flex-col gap-2">
              {links.map((link, idx) => (
                <SidebarLink key={idx} link={link} />
              ))}
            </div>
          </div>
          <div>
            <SidebarLink
              link={{
                label: "User",
                href: "#",
                icon: <UserSVG classes="h-5 w-5 flex-shrink-0" />,
              }}
            />
          </div>
        </SidebarBody>
      </SidebarDemo>
    </div>
  );
}
export const Logo = () => {
  return (
    <Link
      href="#"
      className="font-normal flex space-x-2 items-center text-sm text-black py-1 relative z-20"
    >
      <div className="h-5 w-6 bg-black dark:bg-white rounded-br-lg rounded-tr-sm rounded-tl-lg rounded-bl-sm flex-shrink-0" />
      <motion.span
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        className="font-medium text-black dark:text-white whitespace-pre"
      >
        Acet Labs
      </motion.span>
    </Link>
  );
};
export const LogoIcon = () => {
  return (
    <Link
      href="#"
      className="font-normal flex space-x-2 items-center text-sm text-black py-1 relative z-20"
    >
      <div className="h-5 w-6 bg-black dark:bg-white rounded-br-lg rounded-tr-sm rounded-tl-lg rounded-bl-sm flex-shrink-0" />
    </Link>
  );
};
