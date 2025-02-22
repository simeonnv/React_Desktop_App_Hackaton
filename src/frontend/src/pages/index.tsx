import React from "react";
import { FloatingDock, links } from "@/components/ui/floating-dock";

export default function index() {
  return (
    <>
      <div className="min-h-screen w-full bg-blue-500 flex items-center justify-center">
      <FloatingDock
        mobileClassName="translate-y-20"
        items={links}
      />
      </div>
    </>
  );
}