#!/bin/bash

# Kova Core Deployment Script
# This script deploys the Kova Core system to various environments

set -e

# Configuration
PROJECT_NAME="kova-core"
VERSION=${1:-"latest"}
ENVIRONMENT=${2:-"development"}
DOCKER_REGISTRY=${3:-"kovasystems"}

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    if ! command -v docker &> /dev/null; then
        log_error "Docker is not installed"
        exit 1
    fi
    
    if ! command -v kubectl &> /dev/null; then
        log_warn "kubectl is not installed, skipping Kubernetes deployment"
    fi
    
    if ! command -v cargo &> /dev/null; then
        log_error "Cargo is not installed"
        exit 1
    fi
    
    log_info "Prerequisites check completed"
}

# Build the project
build_project() {
    log_info "Building Kova Core..."
    
    # Build in release mode
    cargo build --release
    
    if [ $? -eq 0 ]; then
        log_info "Build completed successfully"
    else
        log_error "Build failed"
        exit 1
    fi
}

# Run tests
run_tests() {
    log_info "Running tests..."
    
    cargo test
    
    if [ $? -eq 0 ]; then
        log_info "Tests passed"
    else
        log_error "Tests failed"
        exit 1
    fi
}

# Build Docker image
build_docker_image() {
    log_info "Building Docker image..."
    
    docker build -t ${DOCKER_REGISTRY}/${PROJECT_NAME}:${VERSION} .
    
    if [ $? -eq 0 ]; then
        log_info "Docker image built successfully"
    else
        log_error "Docker image build failed"
        exit 1
    fi
}

# Deploy to Docker
deploy_docker() {
    log_info "Deploying to Docker..."
    
    # Stop existing container
    docker stop ${PROJECT_NAME} 2>/dev/null || true
    docker rm ${PROJECT_NAME} 2>/dev/null || true
    
    # Run new container
    docker run -d \
        --name ${PROJECT_NAME} \
        -p 8080:8080 \
        -p 8081:8081 \
        -e RUST_LOG=info \
        ${DOCKER_REGISTRY}/${PROJECT_NAME}:${VERSION}
    
    if [ $? -eq 0 ]; then
        log_info "Docker deployment completed"
    else
        log_error "Docker deployment failed"
        exit 1
    fi
}

# Deploy to Kubernetes
deploy_kubernetes() {
    if ! command -v kubectl &> /dev/null; then
        log_warn "kubectl not available, skipping Kubernetes deployment"
        return
    fi
    
    log_info "Deploying to Kubernetes..."
    
    # Create namespace if it doesn't exist
    kubectl create namespace kova-system --dry-run=client -o yaml | kubectl apply -f -
    
    # Apply Kubernetes manifests
    kubectl apply -f k8s/namespace.yaml
    kubectl apply -f k8s/configmap.yaml
    kubectl apply -f k8s/deployment.yaml
    kubectl apply -f k8s/service.yaml
    
    # Update image
    kubectl set image deployment/${PROJECT_NAME} ${PROJECT_NAME}=${DOCKER_REGISTRY}/${PROJECT_NAME}:${VERSION} -n kova-system
    
    # Wait for rollout
    kubectl rollout status deployment/${PROJECT_NAME} -n kova-system
    
    if [ $? -eq 0 ]; then
        log_info "Kubernetes deployment completed"
    else
        log_error "Kubernetes deployment failed"
        exit 1
    fi
}

# Deploy to cloud
deploy_cloud() {
    case $ENVIRONMENT in
        "aws")
            log_info "Deploying to AWS..."
            # AWS deployment logic
            ;;
        "gcp")
            log_info "Deploying to GCP..."
            # GCP deployment logic
            ;;
        "azure")
            log_info "Deploying to Azure..."
            # Azure deployment logic
            ;;
        *)
            log_warn "Unknown cloud environment: $ENVIRONMENT"
            ;;
    esac
}

# Health check
health_check() {
    log_info "Performing health check..."
    
    # Wait for service to start
    sleep 10
    
    # Check HTTP endpoint
    if curl -f http://localhost:8080/health > /dev/null 2>&1; then
        log_info "Health check passed"
    else
        log_error "Health check failed"
        exit 1
    fi
}

# Cleanup
cleanup() {
    log_info "Cleaning up..."
    
    # Remove old images
    docker image prune -f
    
    # Remove old containers
    docker container prune -f
    
    log_info "Cleanup completed"
}

# Main deployment function
main() {
    log_info "Starting Kova Core deployment..."
    log_info "Version: $VERSION"
    log_info "Environment: $ENVIRONMENT"
    log_info "Docker Registry: $DOCKER_REGISTRY"
    
    check_prerequisites
    build_project
    run_tests
    build_docker_image
    
    case $ENVIRONMENT in
        "development")
            deploy_docker
            health_check
            ;;
        "staging"|"production")
            deploy_kubernetes
            health_check
            ;;
        "cloud")
            deploy_cloud
            ;;
        *)
            log_error "Unknown environment: $ENVIRONMENT"
            exit 1
            ;;
    esac
    
    cleanup
    
    log_info "Deployment completed successfully!"
}

# Run main function
main "$@"
