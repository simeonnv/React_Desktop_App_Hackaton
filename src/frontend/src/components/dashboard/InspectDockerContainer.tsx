import { IconDirectionSignFilled } from "@tabler/icons-react";
import DockerUsage from "../docker/DockerUsage";
import { useDashboard } from "./Sidebar";


type props = {
    indentifier: { type: "id" | "name"; value: string };
}


export default function InspectDockerContainer({ indentifier }: props) {
    const { selectedContainer, setSelectedContainer } = useDashboard();
  
    return (
      <div className="flex flex-col w-full h-screen max-h-screen overflow-y-auto gap-4 p-4">
        <div className="flex items-center text-purple-500 text-3xl font-bold gap-2">
          <IconDirectionSignFilled
            className="h-8 w-8 cursor-pointer"
            onClick={() => setSelectedContainer(null)}
          />
        </div>

        <div className="flex flex-col w-full align-middle justify-center pb-8 items-center text-purple-500 text-3xl font-bold gap-2">
          <p>Container Statistics</p>
            <div className="flex flex-row gap-4">
                <p className="text-purple-500">{indentifier.type}: </p> 
                <p className="text-white">{indentifier.value} </p> 
            </div>
        </div>
  
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 w-full max-w-6xl mx-auto">
          <DockerUsage harvests="CPU" update_time={1} filterBy={indentifier} />
          <DockerUsage harvests="RAM" update_time={1} filterBy={indentifier} />
          <DockerUsage harvests="NET" update_time={1} filterBy={indentifier} />
          <DockerUsage harvests="PIDS" update_time={1} filterBy={indentifier} />
        </div>
  
      </div>
    );
  }