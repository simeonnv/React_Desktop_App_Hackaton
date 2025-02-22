interface CpuStats {
    online_cpus: number;
    total_usage: number;
}

function calculateCpuPercent(current: CpuStats, previous: CpuStats | null): number {
    if (!previous) {
        return 0.0; // No previous data, return 0
    }

    const cpuDelta = current.total_usage - previous.total_usage;
    const numCpus = current.online_cpus || 1;

    // Without system_cpu_usage, we can't calculate a true percentage relative to system usage.
    // Instead, we can normalize cpuDelta over the number of CPUs as a rough approximation.
    // Note: This assumes total_usage is in a unit like nanoseconds; adjust if it's different.
    if (cpuDelta > 0) {
        return (cpuDelta / numCpus) * 100.0; // Rough normalization, adjust scaling factor as needed
    }
    return 0.0;
}