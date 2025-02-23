import React, { useState } from "react";
import { Sidebar, SidebarBody, SidebarLink } from "../ui/sidebar";
import {
  IconArrowLeft,
  IconBrandDocker,
  IconBrandTabler,
  IconSettings,
  IconUserBolt,
} from "@tabler/icons-react";
import Link from "next/link";
import { motion } from "framer-motion";
import { cn } from "@/lib/utils";
import DockerUsage from "../docker/DockerUsage";
import DockerContainers from "./DockerContainers";

export default function SidebarMain() {
  const links = [
    {
      label: (
        <p className="">Dashboard</p>
      ),
      href: "#",
      icon: (
        <IconBrandTabler className=" h-5 w-5 flex-shrink-0" />
      ),
    },
    {
      label: (
        <p className="">Profile</p>
      ),
      href: "#",
      icon: (
        <IconUserBolt className=" h-5 w-5 flex-shrink-0" />
      ),
    },
    {
      label: (
        <p className="">Settings</p>
      ),
      href: "#",
      icon: (
        <IconSettings className=" h-5 w-5 flex-shrink-0" />
      ),
    },
    {
      label: (
        <p className="">Logout</p>
      ),
      href: "#",
      icon: (
        <IconArrowLeft className=" h-5 w-5 flex-shrink-0" />
      ),
    },
  ];

  const [open, setOpen] = useState(false);

  return (
    <div
      className={cn(
        "rounded-md flex flex-col md:flex-row bg-neutral-900 ease-in-out w-full flex-1 max-w-7xl mx-auto border border-neutral-900 overflow-hidden",
        "h-screen"
      )}
    >
      <Sidebar open={open} setOpen={setOpen}>
        <SidebarBody className="justify-between gap-10 !bg-slate-950 border-r-2 border-purple-500/40 rounded-xl">
          <div className="flex flex-col flex-1 overflow-y-auto overflow-x-hidden">
            <Link
              href="#"
              className="font-normal flex space-x-2 items-center justify-center py-1 relative z-20"
            >
              <IconBrandDocker className=" h-5 w-5 flex-shrink-0" />
              <motion.span
                animate={{ opacity: open ? 1 : 0, width: open ? "auto" : 0 }}
                transition={{ duration: 0.3, ease: "easeInOut" }} // Adjust duration to match sidebar
                className="font-medium text-black dark:text-white whitespace-pre overflow-hidden"
              >
                <p className="">Open Docker Monitoring</p>
              </motion.span>
            </Link>
            <div className="mt-8 flex flex-col gap-2">
              {links.map((link: any, idx) => (
                <SidebarLink key={idx} link={link} />
              ))}
            </div>
          </div>
        </SidebarBody>
      </Sidebar>
      <DockerContainers/>
    </div>
  );
}
