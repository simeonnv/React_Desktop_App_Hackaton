"use client";

import { useEffect, useMemo, useState } from "react";
import Charts from "../ui/charts";
import { UsageSockets } from "../../types/UsageSocket";

type Props = {
  harvests: "CPU" | "RAM" | "NET" | "PIDS";
  update_time: number;
};

export default function DockerUsage({ harvests, update_time }: Props) {
  const [containers, setContainers] = useState<UsageSockets[]>([]);
  const [graphData, setGraphData] = useState<{ time: number; usage: number }[]>([]);
  const startTime = useMemo(() => Date.now(), [])
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const wsUrl = `ws://localhost:6004/docker/usage?interval=${update_time}`;
    const socket = new WebSocket(wsUrl);

    socket.onopen = () => {
      console.log("WebSocket connection established");
    };

    socket.onmessage = (event) => {
      try {
        const data: UsageSockets[] = JSON.parse(event.data);
        console.log("Received data:", data);

        // Process the data for the graph
        const newDataPoint = refineData(data, harvests);
        setGraphData((prev) => {
          const updated = [...prev, newDataPoint];
          // Limit to the last 50 data points to prevent memory issues
          return updated.slice(-50);
        });

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
  }, [harvests, update_time]); // Add dependencies to re-run if they change

  // Function to refine data based on the selected metric
  const refineData = (data: UsageSockets[], metric: Props["harvests"]) => {
    // For simplicity, use the first container's data (adjust as needed)
    const container = data[0] || {};
    const time = (new Date(container.read || Date.now()).getTime() - startTime) / 1000; // Convert ISO string to timestamp

    let usage = 0;
    switch (metric) {
      case "CPU":
        if (container.cpu_stats) {
          const cpuDelta = container.cpu_stats.total_usage || 0;
          const systemDelta = container.cpu_stats.system_cpu_usage || 1; // Avoid division by 0
          const cpuCount = container.cpu_stats.online_cpus || 1;
          usage = (cpuDelta / systemDelta) * cpuCount * 100 * 100; // CPU usage as percentage
        }
        break;
      case "RAM":
        if (container.memory_stats) {
          usage = ((container.memory_stats.usage || 0) / Math.pow(10, 9)); // Memory usage in bytes
        }
        break;
      case "NET":
        if (container.network) {
          usage = ((container.network.rx_bytes || 0) + (container.network.tx_bytes || 0)) / Math.pow(10, 6); // Total network bytes
        }
        break;
      case "PIDS":
        if (container.pids_stats) {
          usage = (container.pids_stats.current || 0) * 100; // Number of PIDs
        }
        break;
      default:
        usage = 0;
    }

    return { time, usage };
  };

  // Determine max_usage based on the metric (customize as needed)
  const getMaxUsage = () => {
    switch (harvests) {
      case "CPU":
        return 100; // Percentage
      case "RAM":
        return containers[0]?.memory_stats?.limit || 1024 * 1024 * 1024; // Default 1GB if no limit
      case "NET":
        return 1024 * 1024 * 1024; // Arbitrary max (1GB), adjust as needed
      case "PIDS":
        return containers[0]?.pids_stats?.limit || 1000; // Default 1000 if no limit
      default:
        return 100;
    }
  };

  const getTitle = () => {
    switch (harvests) {
      case "CPU":
        return "Total proccessor usage %"; 
      case "RAM":
        return "Total GB of RAM used"; 
      case "NET":
        return "Total MB of bandwidth";
      case "PIDS":
        return "Total amount of PIDS";
    }
  };

  return (
    <div>
      {error && <p>Error: {error}</p>}
      <Charts graph_data={graphData} max_usage={getMaxUsage()} title={getTitle()} description={"Docker Container Statistics"}/>
    </div>
  );
}