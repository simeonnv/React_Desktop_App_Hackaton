from datetime import time

import aiohttp
import pandas as pd

from app.bridges.docker_usage import get_docker_usage


def analyze_container_performance(df):
    avg_cpu = df["cpuPercent"].mean()
    avg_memory_usage = df["memoryUsage"].mean()
    avg_memory_limit = df["memoryLimit"].mean()
    memory_usage_percent = (avg_memory_usage / avg_memory_limit * 100) if avg_memory_limit else 0

    if avg_cpu < 10 and memory_usage_percent < 30:
        recommendation = "Over-provisioned: Consider reducing allocated resources to optimize cost."
    elif avg_cpu > 80 or memory_usage_percent > 90:
        recommendation = "Under-provisioned: Consider increasing resources to improve performance."
    else:
        recommendation = "Optimal resource usage."

    return {
        "avg_cpuPercent": avg_cpu,
        "avg_memoryUsage_percent": memory_usage_percent,
        "recommendation": recommendation
    }

def check_continuous_high_load(df, metric="cpuPercent", threshold_margin=0.10, consecutive_count=3):
    avg_val = df[metric].mean()
    threshold = avg_val * (1 + threshold_margin)
    count = 0
    for val in df[metric]:
        if val > threshold:
            count += 1
            if count >= consecutive_count:
                return True, threshold
        else:
            count = 0
    return False, threshold

# --- Async Main Function ---
async def main_async():
    # Hard-coded parameters (adjust as needed)
    duration = 60              # Total duration (in seconds) to collect metrics.
    interval = 5               # Sampling interval in seconds.
    do_plot = True             # Set to True to plot the cpuPercent metric.
    stats_window_size = 10     # Use the most recent N samples for analysis.
    max_history_size = 50      # Keep a maximum of N samples in history per container.
    threshold_margin = 0.10    # 10% above average CPU usage for alerting.
    consecutive_count = 3      # Alert if 3 consecutive samples exceed threshold.

    metrics_data: dict[str, list[dict]] = {}  # Keyed by container id.
    start_time = time.time()

    async with aiohttp.ClientSession() as session:
        async for stats_list in get_docker_usage(session, interval_secs=interval):
            current_time = time.time()
            if current_time - start_time > duration:
                break
            for cs in stats_list:
                container_id = cs["id"]
                if container_id not in metrics_data:
                    metrics_data[container_id] = []
                metrics_data[container_id].append(cs)
                if len(metrics_data[container_id]) > max_history_size:
                    metrics_data[container_id] = metrics_data[container_id][-max_history_size:]
                print(f"[{cs.name}] Collected metrics at")

    # Analyze and output results for each container
    for container_id, data_list in metrics_data.items():
        if not data_list:
            print(f"No data for container {container_id}")
            continue

        if len(data_list) > stats_window_size:
            data_list = data_list[-stats_window_size:]
        df = pd.DataFrame(data_list)
        df.sort_values("timestamp", inplace=True)
        summary = analyze_container_performance(df)
        print(f"\nContainer: {df['name'].iloc[0]} ({container_id})")
        print(f"Average CPU Usage (last {len(df)} samples): {summary['avg_cpuPercent']:.2f}%")
        print(f"Average Memory Usage (last {len(df)} samples): {summary['avg_memoryUsage_percent']:.2f}% of limit")
        print(f"Recommendation: {summary['recommendation']}")
        alert, threshold = check_continuous_high_load(
            df, metric="cpuPercent", threshold_margin=threshold_margin, consecutive_count=consecutive_count
        )
        if alert:
            print(f"ALERT: Container {df['name'].iloc[0]} ({container_id}) is experiencing continuous high load. "
                  f"CPU usage is above {threshold:.2f}% for at least {consecutive_count} consecutive samples.")
        else:
            print(f"No continuous high load alert for container {df['name'].iloc[0]} ({container_id}).")
        metrics_csv_file = f"metrics_{container_id}.csv"
        df.to_csv(metrics_csv_file, index=False)
        print(f"Saved detailed metrics to {metrics_csv_file}")