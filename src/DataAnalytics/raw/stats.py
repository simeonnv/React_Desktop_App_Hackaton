from time import sleep

import docker
import pandas as pd
from sklearn.ensemble import IsolationForest

client = docker.from_env()


def calculate_cpu_percent(stats: dict) -> float:
    previous_cpu = stats['precpu_stats']['cpu_usage']['total_usage']
    previous_system = stats['precpu_stats']['system_cpu_usage']
    cpu_delta = stats['cpu_stats']['cpu_usage']['total_usage'] - previous_cpu
    system_delta = stats['cpu_stats']['system_cpu_usage'] - previous_system

    if system_delta > 0 and cpu_delta > 0:
        num_cpus = len(stats['cpu_stats']['cpu_usage']['percpu_usage'])
        return (cpu_delta / system_delta) * num_cpus * 100.0
    return 0.0


def get_container_stats(container_id):
    """Fetch container resource usage stats"""
    container = client.containers.get(container_id)
    stats = container.stats(stream=False)

    # CPU calculation
    cpu_percent = 0.0
    cpu_delta = stats['cpu_stats']['cpu_usage']['total_usage'] - stats['precpu_stats']['cpu_usage']['total_usage']
    system_delta = stats['cpu_stats']['system_cpu_usage'] - stats['precpu_stats']['system_cpu_usage']
    if system_delta > 0 and cpu_delta > 0:
        cpu_percent = (cpu_delta / system_delta) * len(stats['cpu_stats']['cpu_usage']['percpu_usage']) * 100

    # Memory calculation
    memory_usage = stats['memory_stats']['usage'] / (1024 ** 2)  # MB
    memory_limit = stats['memory_stats']['limit'] / (1024 ** 2)  # MB

    return {
        'container_id': container_id,
        'cpu_percent': cpu_percent,
        'memory_usage': memory_usage,
        'memory_limit': memory_limit,
        'network_io': stats['networks']['eth0']['rx_bytes'] + stats['networks']['eth0']['tx_bytes']
    }


def analyze_resource_utilization(stats_history):
    """Analyze resource usage with per-container anomaly detection"""
    # Create a DataFrame from the complete stats history
    df = pd.DataFrame(stats_history)
    anomaly_results = []

    # Group by container_id to analyze each container's history separately
    for container_id, group in df.groupby("container_id"):
        group = group.copy()
        # Simple threshold alerts per container
        group['cpu_alert'] = group['cpu_percent'] > 80
        group['memory_alert'] = group['memory_usage'] > 0.9 * group['memory_limit']

        # Run anomaly detection only if there are enough records
        if len(group) > 1:
            model = IsolationForest(contamination=0.1)
            anomalies = model.fit_predict(group[['cpu_percent', 'memory_usage']])
            group['anomaly'] = anomalies == -1
        else:
            group['anomaly'] = False

        anomaly_results.append(group)

    # Concatenate the per-container results back together
    return pd.concat(anomaly_results)


def container_health_check(container):
    """Basic container health assessment"""
    status = container.status
    restart_count = container.attrs['RestartCount']

    health_status = "Healthy"
    if restart_count > 3:
        health_status = "Unstable"
    if status != 'running':
        health_status = "Critical"

    return {
        'container_id': container.id,
        'status': status,
        'restart_count': restart_count,
        'health_status': health_status
    }


def cost_optimization_analysis(stats_history):
    """Basic cost estimation and optimization suggestions"""
    df = pd.DataFrame(stats_history)
    # Simple cost estimation (assuming $0.0001 per CPU-minute and $0.00005 per MB-hour)
    df['estimated_cost'] = (df['cpu_percent'] / 100 * 0.0001 * (1 / 60)) + \
                           (df['memory_usage'] * 0.00005 * (1 / 60))

    # Identify underutilized resources
    df['memory_underutilized'] = df['memory_usage'] < 0.3 * df['memory_limit']
    df['cpu_underutilized'] = df['cpu_percent'] < 20

    return df


def main():
    # Monitor all containers
    containers = client.containers.list()

    stats_history = []
    for _ in range(5):  # Run 5 iterations
        for container in containers:
            # Resource monitoring
            stats = get_container_stats(container.id)
            stats_history.append(stats)

            # Health monitoring
            health = container_health_check(container)
            print(f"Health Status for {container.id[:12]}: {health['health_status']}")

        # Run analytics every 5 iterations using the entire history for per-container analysis
        if len(stats_history) % (len(containers) * 2) == 0:
            resource_df = analyze_resource_utilization(stats_history)
            cost_df = cost_optimization_analysis(stats_history)

            print("\nResource Anomalies:")
            print(resource_df[['cpu_percent', 'anomaly']].tail())
            print(resource_df[['memory_usage', 'anomaly']].tail())

            print("\nCost Optimization Suggestions:")
            print(cost_df[['estimated_cost', 'memory_underutilized']].tail())
            print(cost_df[['estimated_cost', 'cpu_underutilized']].tail())

            # Save anomaly results to a CSV file
            resource_df.to_csv("anomalies_results.csv", index=True)
            print("\nAnomaly results saved to anomalies_results.csv")

        sleep(5)  # Check every 5 seconds


if __name__ == "__main__":
    main()
