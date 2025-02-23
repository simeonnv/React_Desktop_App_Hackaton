import React from "react";
import { FloatingDock, links } from "@/components/ui/floating-dock";
import { Card } from "@/components/auth/Card";
import { Sidebar } from "@/components/auth/Sidebar";
import Searchbar from "@/components/auth/Searchbar";

export default function CombinedComponent() {
  return (
    <>
      <div className="min-h-screen w-full bg-blue-500 flex flex-col items-center justify-center">
        <Searchbar />
        <Sidebar />

        <Card />
      </div>
    </>
  );
}
