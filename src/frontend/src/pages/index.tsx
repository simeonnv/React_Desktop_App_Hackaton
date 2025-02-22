import React from "react";
import { FloatingDock, links } from "@/components/ui/floating-dock";
import { Card } from "@/components/auth/Card";
import { Sidebar } from "@/components/auth/Sidebar";

export default function CombinedComponent() {
  return (
    <>
      <div className="min-h-screen w-full bg-blue-500 flex flex-col items-center justify-center">
        <Sidebar />
        <FloatingDock mobileClassName="translate-y-20" items={links} />
        <Card />
      </div>
    </>
  );
}
