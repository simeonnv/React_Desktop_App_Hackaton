import os
import time
import html
import docker
import requests

# Configuration from environment variables
TELEGRAM_BOT_TOKEN = "7300288725:AAFuyPyNHiW5CCHFzE5kknn1_eMxTfjLsTQ"
TELEGRAM_CHAT_ID = 1370280956

if not TELEGRAM_BOT_TOKEN or not TELEGRAM_CHAT_ID:
    raise ValueError("Please set TELEGRAM_BOT_TOKEN and TELEGRAM_CHAT_ID environment variables")

# Initialize Docker client
client = docker.from_env()


def get_container_logs(container, tail=50, max_length=1000):
    """Get container logs with truncation and error handling"""
    try:
        logs = container.logs(tail=tail, timestamps=True).decode('utf-8')
        # Truncate logs to last 20 lines and max_length characters
        lines = logs.split('\n')[-20:]
        truncated_logs = '\n'.join(lines)
        if len(truncated_logs) > max_length:
            truncated_logs = truncated_logs[:max_length] + '...\n(truncated)'
        return truncated_logs
    except docker.errors.APIError as e:
        return f"Error retrieving logs: {str(e)}"
    except Exception as e:
        return f"Unexpected error: {str(e)}"


def get_container_statuses():
    """Check all containers and return problematic statuses with logs"""
    issues = []

    for container in client.containers.list(all=True):
        status = container.status.lower()

        # Check for exited containers
        if status == 'exited':
            exit_code = container.attrs['State']['ExitCode']
            logs = get_container_logs(container)
            issues.append({
                'name': container.name,
                'status': 'exited',
                'exit_code': exit_code,
                'logs': logs
            })
            continue

        # Check for unhealthy containers
        try:
            health_status = container.attrs['State']['Health']['Status'].lower()
            if health_status == 'unhealthy':
                health_logs = container.attrs['State']['Health']['Log'][-1]['Output']
                container_logs = get_container_logs(container)
                issues.append({
                    'name': container.name,
                    'status': 'unhealthy',
                    'health_logs': health_logs,
                    'container_logs': container_logs
                })
        except KeyError:
            pass  # No health check configured

    return issues


def send_telegram_alert(message):
    """Send message to Telegram using bot"""
    url = f"https://api.telegram.org/bot{TELEGRAM_BOT_TOKEN}/sendMessage"
    payload = {
        'chat_id': TELEGRAM_CHAT_ID,
        'text': message,
        'parse_mode': 'HTML'
    }

    try:
        response = requests.post(url, json=payload)
        response.raise_for_status()
    except requests.exceptions.RequestException as e:
        print(f"Failed to send Telegram alert: {e}")


def format_logs(logs, max_lines=10):
    """Format logs for Telegram with proper escaping"""
    lines = logs.split('\n')[-max_lines:]
    escaped_lines = [html.escape(line) for line in lines]
    return '\n'.join(escaped_lines)


def main():
    check_interval = 300  # 5 minutes

    while True:
        issues = get_container_statuses()

        if issues:
            message = "<b>🚨 Docker Alert</b>\n\n"
            for issue in issues:
                if issue['status'] == 'exited':
                    formatted_logs = format_logs(issue['logs'])
                    message += (
                        f"🛑 Container <code>{issue['name']}</code> has exited\n"
                        f"Exit code: {issue['exit_code']}\n"
                        f"<b>Last logs:</b>\n<pre>{formatted_logs}</pre>\n\n"
                    )
                elif issue['status'] == 'unhealthy':
                    health_logs = format_logs(issue['health_logs'])
                    container_logs = format_logs(issue['container_logs'])
                    message += (
                        f"⚠️ Container <code>{issue['name']}</code> is unhealthy\n"
                        f"<b>Health check error:</b>\n<pre>{health_logs}</pre>\n"
                        f"<b>Container logs:</b>\n<pre>{container_logs}</pre>\n\n"
                    )

            send_telegram_alert(message.strip())

        time.sleep(check_interval)


if __name__ == "__main__":
    main()