import { IconDirectionSignFilled } from "@tabler/icons-react";
import DockerUsage from "../docker/DockerUsage";
import { useDashboard } from "./Sidebar";


type props = {
    indentifier: { type: "id" | "name"; value: string };
}


export default function InspectDockerContainer({ indentifier }: props) {

    const { selectedContainer, setSelectedContainer } = useDashboard();

    return (
        <div className="flex flex-col w-full justify-center items-center h-screen max-h-screen gap-4">
            <div className="flex flex-col text-purple-500 text-3xl font-bold">
                <div className='text-purple-500'>
                    <IconDirectionSignFilled className='h-8 w-8' onClick={() => setSelectedContainer(null)}/>
                </div>
            </div>
            <div className="flex flex-col text-purple-500 text-3xl font-bold">
                <p>Container Statistics</p>
            </div>

            <div className="grid grid-cols-2 p-40">
                <DockerUsage harvests="CPU" update_time={1} filterBy={indentifier}/>
                <DockerUsage harvests="RAM" update_time={1} filterBy={indentifier}/>
                <DockerUsage harvests="NET" update_time={1} filterBy={indentifier}/>
                <DockerUsage harvests="PIDS" update_time={1} filterBy={indentifier}/>
            </div>
            {indentifier.type}
            {indentifier.value}
        </div>
    )
}