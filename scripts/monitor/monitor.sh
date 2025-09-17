#!/bin/bash

# Kova Core Monitoring Script
# This script monitors the health and performance of the Kova Core system

set -e

# Configuration
PROJECT_NAME="kova-core"
MONITORING_INTERVAL=${1:-30} # seconds
LOG_FILE="/var/log/kova-core/monitor.log"
METRICS_FILE="/var/log/kova-core/metrics.json"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1" | tee -a $LOG_FILE
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1" | tee -a $LOG_FILE
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1" | tee -a $LOG_FILE
}

log_metric() {
    echo -e "${BLUE}[METRIC]${NC} $1" | tee -a $LOG_FILE
}

# Create log directory
mkdir -p /var/log/kova-core

# Check if service is running
check_service_status() {
    if docker ps | grep -q ${PROJECT_NAME}; then
        echo "running"
    elif kubectl get pods -n kova-system | grep -q ${PROJECT_NAME}; then
        echo "running"
    else
        echo "stopped"
    fi
}

# Check HTTP health endpoint
check_health_endpoint() {
    local endpoint="http://localhost:8080/health"
    
    if curl -f -s $endpoint > /dev/null 2>&1; then
        echo "healthy"
    else
        echo "unhealthy"
    fi
}

# Check API endpoints
check_api_endpoints() {
    local endpoints=(
        "http://localhost:8080/api/v1/sensors"
        "http://localhost:8080/api/v1/validation"
        "http://localhost:8080/api/v1/blockchain"
        "http://localhost:8080/metrics"
    )
    
    local healthy_count=0
    local total_count=${#endpoints[@]}
    
    for endpoint in "${endpoints[@]}"; do
        if curl -f -s $endpoint > /dev/null 2>&1; then
            ((healthy_count++))
        fi
    done
    
    echo "$healthy_count/$total_count"
}

# Get system metrics
get_system_metrics() {
    local metrics=()
    
    # CPU usage
    local cpu_usage=$(top -bn1 | grep "Cpu(s)" | awk '{print $2}' | awk -F'%' '{print $1}')
    metrics+=("cpu_usage:$cpu_usage")
    
    # Memory usage
    local memory_usage=$(free | grep Mem | awk '{printf "%.2f", $3/$2 * 100.0}')
    metrics+=("memory_usage:$memory_usage")
    
    # Disk usage
    local disk_usage=$(df -h / | awk 'NR==2{print $5}' | sed 's/%//')
    metrics+=("disk_usage:$disk_usage")
    
    # Network I/O
    local network_rx=$(cat /proc/net/dev | grep eth0 | awk '{print $2}')
    local network_tx=$(cat /proc/net/dev | grep eth0 | awk '{print $10}')
    metrics+=("network_rx:$network_rx")
    metrics+=("network_tx:$network_tx")
    
    # Process count
    local process_count=$(ps aux | wc -l)
    metrics+=("process_count:$process_count")
    
    echo "${metrics[*]}"
}

# Get application metrics
get_application_metrics() {
    local metrics=()
    
    # Get metrics from application
    local app_metrics=$(curl -s http://localhost:8080/metrics 2>/dev/null || echo "")
    
    if [ -n "$app_metrics" ]; then
        # Parse application metrics
        local sensor_count=$(echo "$app_metrics" | grep "sensor_count" | awk '{print $2}' || echo "0")
        local validation_count=$(echo "$app_metrics" | grep "validation_count" | awk '{print $2}' || echo "0")
        local blockchain_count=$(echo "$app_metrics" | grep "blockchain_count" | awk '{print $2}' || echo "0")
        
        metrics+=("sensor_count:$sensor_count")
        metrics+=("validation_count:$validation_count")
        metrics+=("blockchain_count:$blockchain_count")
    fi
    
    echo "${metrics[*]}"
}

# Check log files for errors
check_log_errors() {
    local error_count=0
    
    # Check Docker logs
    if docker ps | grep -q ${PROJECT_NAME}; then
        error_count=$(docker logs ${PROJECT_NAME} 2>&1 | grep -i error | wc -l)
    fi
    
    # Check Kubernetes logs
    if kubectl get pods -n kova-system | grep -q ${PROJECT_NAME}; then
        local pod_name=$(kubectl get pods -n kova-system | grep ${PROJECT_NAME} | awk '{print $1}')
        error_count=$(kubectl logs $pod_name -n kova-system 2>&1 | grep -i error | wc -l)
    fi
    
    echo $error_count
}

# Generate metrics JSON
generate_metrics_json() {
    local timestamp=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
    local service_status=$(check_service_status)
    local health_status=$(check_health_endpoint)
    local api_status=$(check_api_endpoints)
    local system_metrics=$(get_system_metrics)
    local app_metrics=$(get_application_metrics)
    local error_count=$(check_log_errors)
    
    cat > $METRICS_FILE << EOF
{
    "timestamp": "$timestamp",
    "service_status": "$service_status",
    "health_status": "$health_status",
    "api_status": "$api_status",
    "error_count": $error_count,
    "system_metrics": {
        $(echo $system_metrics | sed 's/\([^:]*\):\([^ ]*\)/"\1": \2/g' | sed 's/ /, /g')
    },
    "application_metrics": {
        $(echo $app_metrics | sed 's/\([^:]*\):\([^ ]*\)/"\1": \2/g' | sed 's/ /, /g')
    }
}
EOF
}

# Send alert
send_alert() {
    local message="$1"
    local severity="$2"
    
    # Log alert
    log_error "ALERT [$severity]: $message"
    
    # Send email (if configured)
    if [ -n "$ALERT_EMAIL" ]; then
        echo "$message" | mail -s "Kova Core Alert [$severity]" $ALERT_EMAIL
    fi
    
    # Send webhook (if configured)
    if [ -n "$ALERT_WEBHOOK" ]; then
        curl -X POST -H "Content-Type: application/json" \
            -d "{\"text\":\"$message\",\"severity\":\"$severity\"}" \
            $ALERT_WEBHOOK
    fi
}

# Check thresholds and send alerts
check_thresholds() {
    local system_metrics=$(get_system_metrics)
    local error_count=$(check_log_errors)
    
    # Check CPU usage
    local cpu_usage=$(echo $system_metrics | grep -o 'cpu_usage:[0-9.]*' | cut -d: -f2)
    if (( $(echo "$cpu_usage > 80" | bc -l) )); then
        send_alert "High CPU usage: ${cpu_usage}%" "WARNING"
    fi
    
    # Check memory usage
    local memory_usage=$(echo $system_metrics | grep -o 'memory_usage:[0-9.]*' | cut -d: -f2)
    if (( $(echo "$memory_usage > 85" | bc -l) )); then
        send_alert "High memory usage: ${memory_usage}%" "WARNING"
    fi
    
    # Check disk usage
    local disk_usage=$(echo $system_metrics | grep -o 'disk_usage:[0-9]*' | cut -d: -f2)
    if [ "$disk_usage" -gt 90 ]; then
        send_alert "High disk usage: ${disk_usage}%" "CRITICAL"
    fi
    
    # Check error count
    if [ "$error_count" -gt 10 ]; then
        send_alert "High error count: $error_count errors" "WARNING"
    fi
    
    # Check service status
    local service_status=$(check_service_status)
    if [ "$service_status" != "running" ]; then
        send_alert "Service is not running: $service_status" "CRITICAL"
    fi
    
    # Check health endpoint
    local health_status=$(check_health_endpoint)
    if [ "$health_status" != "healthy" ]; then
        send_alert "Health endpoint is not responding: $health_status" "CRITICAL"
    fi
}

# Display status
display_status() {
    local service_status=$(check_service_status)
    local health_status=$(check_health_endpoint)
    local api_status=$(check_api_endpoints)
    local error_count=$(check_log_errors)
    
    echo "=========================================="
    echo "Kova Core Monitoring Status"
    echo "=========================================="
    echo "Service Status: $service_status"
    echo "Health Status: $health_status"
    echo "API Status: $api_status"
    echo "Error Count: $error_count"
    echo "Timestamp: $(date)"
    echo "=========================================="
}

# Main monitoring loop
main() {
    log_info "Starting Kova Core monitoring..."
    log_info "Monitoring interval: ${MONITORING_INTERVAL} seconds"
    
    while true; do
        # Generate metrics
        generate_metrics_json
        
        # Check thresholds and send alerts
        check_thresholds
        
        # Display status
        display_status
        
        # Log metrics
        log_metric "Metrics updated: $METRICS_FILE"
        
        # Wait for next check
        sleep $MONITORING_INTERVAL
    done
}

# Handle signals
trap 'log_info "Monitoring stopped"; exit 0' SIGINT SIGTERM

# Run main function
main "$@"
