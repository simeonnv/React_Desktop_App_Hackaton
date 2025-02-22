export interface UsageSockets {
    read: string;
    names: string[];
    id: string;
    memory_stats: MemoryStats;
    cpu_stats: CpuStats;
    pids_stats: PidsStats;
    network?: Network;  // Optional type using ?
}

interface MemoryStats {
    usage: number;  // Using number for u64 (JS doesn't have exact u64)
    limit: number;
}

interface CpuStats {
    online_cpus: number;
    total_usage: number;
    system_cpu_usage: number
}

interface PidsStats {
    current: number;
    limit: number;
}

interface Network {
    rx_dropped: number;
    rx_bytes: number;
    rx_errors: number;
    tx_packets: number;
    tx_dropped: number;
    rx_packets: number;
    tx_errors: number;
    tx_bytes: number;
}