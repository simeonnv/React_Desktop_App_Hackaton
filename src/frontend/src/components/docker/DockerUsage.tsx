"use client";

import { useEffect, useMemo, useState } from "react";
import Charts from "../ui/charts";
import { UsageSockets } from "../../types/UsageSocket";

type Props = {
  harvests: "CPU" | "RAM" | "NET" | "PIDS";
  update_time: number;
  filterBy?: { type: "id" | "name"; value: string }; // New optional prop for filtering
};

export default function DockerUsage({ harvests, update_time, filterBy }: Props) {
  const [containers, setContainers] = useState<UsageSockets[]>([]);
  const [graphData, setGraphData] = useState<{ time: number; usage: number }[]>([]);
  const startTime = useMemo(() => Date.now(), []);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const wsUrl = `ws://localhost:6004/docker/usage?interval=${update_time}`;
    const socket = new WebSocket(wsUrl);

    socket.onopen = () => {
      console.log("WebSocket connection established");
    };

    socket.onmessage = (event) => {
      try {
        let data: UsageSockets[] = JSON.parse(event.data);
        console.log("Received data:", data);

        // Apply filtering if filterBy is provided
        if (filterBy) {
          data = data.filter((container) =>
            filterBy.type === "id"
              ? container.id === filterBy.value
              : container.names.includes(filterBy.value)
          );
        }

        // Process the data for the graph if we have matching containers
        if (data.length > 0) {
          const newDataPoint = refineData(data, harvests);
          setGraphData((prev) => {
            const updated = [...prev, newDataPoint];
            return updated.slice(-50);
          });
        }

        setContainers(data);
        setError(null);
      } catch (err) {
        console.error("Error parsing WebSocket message:", err);
        setError("Failed to parse container data");
      }
    };

    socket.onerror = (err) => {
      console.error("WebSocket error:", err);
      setError("WebSocket connection error");
    };

    socket.onclose = () => {
      console.log("WebSocket connection closed");
      setError("Connection closed");
    };

    return () => {
      if (socket.readyState === WebSocket.OPEN) {
        socket.close();
      }
    };
  }, [harvests, update_time, filterBy]); // Add filterBy to dependencies

  const refineData = (data: UsageSockets[], metric: Props["harvests"]) => {
    // Use the first container after filtering (assuming single container match)
    const container = data[0] || {};
    const time = (new Date(container.read || Date.now()).getTime() - startTime) / 1000;

    let usage = 0;
    switch (metric) {
      case "CPU":
        if (container.cpu_stats) {
          const cpuDelta = container.cpu_stats.total_usage || 0;
          const systemDelta = container.cpu_stats.system_cpu_usage || 1;
          const cpuCount = container.cpu_stats.online_cpus || 1;
          usage = (cpuDelta / systemDelta) * cpuCount * 100 * 100;
        }
        break;
      case "RAM":
        if (container.memory_stats) {
          usage = (container.memory_stats.usage || 0) / Math.pow(10, 9);
        }
        break;
      case "NET":
        if (container.network) {
          usage = ((container.network.rx_bytes || 0) + (container.network.tx_bytes || 0)) / Math.pow(10, 6);
        }
        break;
      case "PIDS":
        if (container.pids_stats) {
          usage = (container.pids_stats.current || 0) * 100;
        }
        break;
      default:
        usage = 0;
    }

    return { time, usage };
  };

  const getMaxUsage = () => {
    switch (harvests) {
      case "CPU":
        return 100;
      case "RAM":
        return containers[0]?.memory_stats?.limit || 1024 * 1024 * 1024;
      case "NET":
        return 1024 * 1024 * 1024;
      case "PIDS":
        return containers[0]?.pids_stats?.limit || 1000;
      default:
        return 100;
    }
  };

  const getTitle = () => {
    const baseTitle = (() => {
      switch (harvests) {
        case "CPU": return "Total processor usage %";
        case "RAM": return "Total GB of RAM used";
        case "NET": return "Total MB of bandwidth";
        case "PIDS": return "Total amount of PIDS";
      }
    })();
    
    return baseTitle

  };

  return (
    <div>
      {error && <p>Error: {error}</p>}
      {containers.length === 0 && filterBy && !error && (
        <p>No containers found matching {filterBy.type}: {filterBy.value}</p>
      )}
      <Charts 
        graph_data={graphData} 
        max_usage={getMaxUsage()} 
        title={getTitle()} 
        description={"Docker Container Statistics"}
      />
    </div>
  );
}