#!/bin/bash
# Argent Container Build Script
# Builds Docker and prepares Firecracker images

set -e

VERSION="0.1.1"
IMAGE_NAME="argent/argent"
DOCKER_BUILDKIT=1

echo "=== Argent Container Build v${VERSION} ==="

# Build Docker image
echo "[1/3] Building Docker image..."
docker build \
    --build-arg BUILDKIT_INLINE_CACHE=1 \
    -t ${IMAGE_NAME}:${VERSION} \
    -t ${IMAGE_NAME}:latest \
    -f container/Dockerfile \
    .

echo "[2/3] Scanning for vulnerabilities..."
docker run --rm -v /var/run/docker.sock:/var/run/docker.sock \
    aquasec/trivy image --severity HIGH,CRITICAL \
    ${IMAGE_NAME}:${VERSION} || true

echo "[3/3] Testing container..."
docker run --rm ${IMAGE_NAME}:${VERSION} --help

echo "=== Build Complete ==="
echo "Docker: ${IMAGE_NAME}:${VERSION}"
echo ""
echo "To run:"
echo "  docker run -it ${IMAGE_NAME}:${VERSION}"
echo ""
echo "For Firecracker, use firecracker-containerd or cloud-hypervisor"
