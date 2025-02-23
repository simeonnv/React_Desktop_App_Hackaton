import React from "react";
import { FloatingDock, links } from "@/components/ui/floating-dock";

import { Card } from "@/components/auth/Card";
import { Sidebar } from "@/components/dashboard/Sidebar";
import Searchbar from "@/components/auth/Searchbar";
import DockerUsage from "@/components/docker/DockerUsage";
import { Test } from "@/components/ui/test";
import DashboardDemo from "@/components/dashboard/Sidebar";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

const queryClient = new QueryClient();

export default function CombinedComponent() {
  return (
    <>
      <div className="min-h-screen w-full bg-blue-500 flex flex-col items-center justify-center">
        <Searchbar />
        <Sidebar />

        <Card />
      </div>
      <QueryClientProvider client={queryClient}>
        <div className="min-h-screen w-full bg-black flex items-center justify-center">
          {/* <FloatingDock
            mobileClassName="translate-y-20"
            items={links}
          />
          <DockerUsage 
            harvests="RAM" 
            update_time={1} 
            filterBy={{ type: "id", value: "4e44e1b6e0cb93e9abffd9e2513fca065194c5095146153d7072794529fb43b9" }} 
          />
          <Test/> */}
          <DashboardDemo />
        </div>
      </QueryClientProvider>
    </>
  );
}
