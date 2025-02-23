# import time
# from prometheus_client import start_http_server, Gauge
# import docker
#
# # Create Prometheus metrics with labels
# CONTAINER_CPU_USAGE = Gauge(
#     'container_cpu_usage_percent',
#     'CPU usage percentage of the container',
#     ['container_name', 'container_id']
# )
#
# CONTAINER_MEMORY_USAGE = Gauge(
#     'container_memory_usage_bytes',
#     'Memory usage of the container in bytes',
#     ['container_name', 'container_id']
# )
#
# # Initialize Docker client
# docker_client = docker.from_env()
#
# # Dictionary to store previous CPU stats for calculation
# previous_cpu_stats = {}
#
#
# def collect_container_metrics():
#     """Collect and update metrics for all running containers"""
#     containers = docker_client.containers.list()
#
#     for container in containers:
#         try:
#             stats = container.stats(stream=False)
#             container_name = container.name
#             container_id = container.short_id
#
#             # Update memory metric
#             memory_stats = stats.get('memory_stats', {})
#             memory_usage = memory_stats.get('usage', 0)
#             CONTAINER_MEMORY_USAGE.labels(
#                 container_name=container_name,
#                 container_id=container_id
#             ).set(memory_usage)
#
#             # Update CPU metric
#             cpu_stats = stats.get('cpu_stats', {})
#             cpu_usage = cpu_stats.get('cpu_usage', {})
#             system_cpu_usage = cpu_stats.get('system_cpu_usage', 0)
#             current_cpu = cpu_usage.get('total_usage', 0)
#
#             # Calculate CPU percentage
#             prev = previous_cpu_stats.get(container_id, (0, 0))
#             prev_cpu, prev_system = prev
#
#             if prev_system != 0 and system_cpu_usage != prev_system:
#                 cpu_delta = current_cpu - prev_cpu
#                 system_delta = system_cpu_usage - prev_system
#
#                 if system_delta > 0 and cpu_delta > 0:
#                     cpu_percent = (cpu_delta / system_delta) * 100
#                     CONTAINER_CPU_USAGE.labels(
#                         container_name=container_name,
#                         container_id=container_id
#                     ).set(cpu_percent)
#
#             # Store current stats for next calculation
#             previous_cpu_stats[container_id] = (current_cpu, system_cpu_usage)
#
#         except Exception as e:
#             print(f"Error processing container {container.name}: {str(e)}")
#
#
# if __name__ == '__main__':
#     # Start Prometheus metrics server on port 8000
#     start_http_server(8000)
#     print("Metrics server started on port 8000")
#
#     # Collect metrics every 5 seconds
#     while True:
#         collect_container_metrics()
#         time.sleep(5)
# # import os
# #
# # import docker
# #
# # def calculate_cpu_percent(stats: dict) -> float:
# #     previous_cpu = stats['precpu_stats']['cpu_usage']['total_usage']
# #     previous_system = stats['precpu_stats']['system_cpu_usage']
# #     cpu_delta = stats['cpu_stats']['cpu_usage']['total_usage'] - previous_cpu
# #     system_delta = stats['cpu_stats']['system_cpu_usage'] - previous_system
# #
# #     if system_delta > 0 and cpu_delta > 0:
# #         num_cpus = len(stats['cpu_stats']['cpu_usage']['percpu_usage'])
# #         return (cpu_delta / system_delta) * num_cpus * 100.0
# #     return 0.0
# #
# # client = docker.from_env()
# # for container in client.containers.list():
# #     stats = container.stats(decode=False, stream=False)
# #     total_usage = calculate_cpu_percent(stats)
# #
# #     print(total_usage)