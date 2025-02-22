import React from "react";
import { FloatingDock, links } from "@/components/ui/floating-dock";
import DockerUsage from "@/components/docker/DockerUsage";
import { Test } from "@/components/ui/test";

export default function CombinedComponent() {
  return (
    <>
      <div className="min-h-screen w-full bg-blue-500 flex items-center justify-center">
        <FloatingDock
          mobileClassName="translate-y-20"
          items={links}
        />
        <DockerUsage harvests="RAM" update_time={1}/>
        <Test/>
      </div>
    </>
  );
}