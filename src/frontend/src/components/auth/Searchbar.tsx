"use client";
import { Input } from "../ui/input";
//import { cn } from "../../lib/utils";
import SearchSVG from "../ui/SearchSVG";

export default function Searchbar() {
  return (
    <div className=" flex flex-row gap-1 items-center bg-neutral-800 rounded-lg w-60 h-12 pr-3">
      <SearchSVG classes="h-5 w-5 flex-shrink-0 ml-3" />
      <Input
        className="w-full "
        placeholder="My container"
        id="searchbar"
        type="text"
      />
    </div>
  );
}
