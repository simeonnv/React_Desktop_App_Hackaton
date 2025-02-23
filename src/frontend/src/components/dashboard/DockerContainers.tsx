// src/components/DockerContainers.jsx
import { useQuery } from '@tanstack/react-query';
import axios from 'axios';
import { useDashboard } from './Sidebar'; // Ensure this matches your export path
import InspectDockerContainer from './InspectDockerContainer';
import { IconDirectionSignFilled } from '@tabler/icons-react';

const fetchDockerContainers = async () => {
    const response = await axios.get('http://localhost:6004/docker/containers');
    return response.data; // Expecting { status: string, data: array }
};

function DockerContainers() {
    const { data, isLoading, error } = useQuery({
        queryKey: ['dockerContainers'],
        queryFn: fetchDockerContainers,
        refetchInterval: 5000,
    });

    const { selectedContainer, setSelectedContainer } = useDashboard();

    if (isLoading) return <div>Loading...</div>;
    if (error) return <div>Error: {error.message}</div>;

    // Conditional rendering based on selectedContainer
    return (
        <>
            {selectedContainer === null ? (
                <div className="flex flex-col items-center p-4 gap-4 bg-stone-950">
                    <h1 className="text-purple-500 text-4xl font-bold">Docker Containers</h1>
                    <div className="flex flex-row gap-2 text-xl font-bold">
                        <p>Status: </p>
                        <p className="text-purple-500">{data.status}</p>
                    </div>
                    {/* Added max-h-[500px] and overflow-y-auto */}
                    <div className="grid md:grid-cols-2 grid-cols-1 gap-4 max-h-[500px] overflow-y-auto p-2">
                        {data.data.map((container: DockerContainer) => (
                            <div
                                key={container.id}
                                className="border-2 border-purple-500/40 rounded-xl p-4 break-words"
                            >
                                <div>
                                    <p>
                                        <strong className="text-purple-500">Main Name: </strong> {container.names[0]}
                                    </p>
                                    <p>
                                        <strong className="text-purple-500">ID: </strong> ({container.id.slice(0, 12)})
                                    </p>
                                </div>
                                <br />
                                <div>
                                    <p>
                                        <strong className="text-purple-500">Image: </strong> {container.image}
                                    </p>
                                    <p>
                                        <strong className="text-purple-500">State: </strong> {container.state}
                                    </p>
                                    <p>
                                        <strong className="text-purple-500">Status: </strong> {container.status}
                                    </p>
                                </div>
                                <br />
                                <div className='flex flex-row items-center justify-center'>
                                    <div>
                                        <strong className="text-purple-500">Ports: </strong>
                                        {container.ports.map((port) => `${port.PublicPort}:${port.PrivatePort}`).join(', ')}
                                    </div>
                                    <div className='grow'/>
                                    { container.state === "running" ?
                                        <div className='text-purple-500'>
                                            <IconDirectionSignFilled className='h-8 w-8' onClick={() => setSelectedContainer({type: "name", value: container.names[0]})}/>
                                        </div> : ""
                                    }
                                </div>
                            </div>
                        ))}
                    </div>
                </div>
            ) : (
                <InspectDockerContainer indentifier={selectedContainer} />
            )}
        </>
    );
}

export default DockerContainers;