import React from "react";
import { FloatingDock, links } from "@/components/ui/floating-dock";
import { CardDemo } from "@/components/auth/CardDemo";
import { SidebarDemo } from "@/components/auth/SidebarDemo";

export default function CombinedComponent() {
  return (
    <>
      <div className="min-h-screen w-full bg-blue-500 flex flex-col items-center justify-center">
        <SidebarDemo />
        <FloatingDock mobileClassName="translate-y-20" items={links} />
        <CardDemo />
      </div>
    </>
  );
}
