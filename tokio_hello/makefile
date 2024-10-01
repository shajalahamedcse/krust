# Variables
DOCKER_IMAGE := pinger-operator
KIND_CLUSTER_NAME := my-cluster
VERSION := 0.0.3

# Phony targets
.PHONY: all setup-cluster build-image load-image deploy clean

# Default target
all: setup-cluster build-image load-image deploy

# Set up the Kind cluster
setup-cluster:
	@ echo "Setting up Kind cluster..."
	@ kind create cluster --name $(KIND_CLUSTER_NAME) --config deployment/kind-config.yaml
	@ echo "Kind cluster setup complete."

# Build the Docker image
build-image:
	@ echo "Building Docker image..."
	@ docker build -t ${DOCKER_IMAGE}:${VERSION} .
	@ echo "Docker image build complete."

# Load the image into Kind
load-image:
	@ echo "Loading image into Kind cluster..."
	@ kind load docker-image ${DOCKER_IMAGE}:${VERSION} --name $(KIND_CLUSTER_NAME)
	@ echo "Image loaded into Kind cluster."

# Deploy the application
deploy:
	@ echo "Deploying application..."
	@ kubectl apply -f deployment/pinger-crd.yaml
	@ kubectl apply -f deployment/pinger-instance.yaml
	@ kubectl apply -f deployment/controller-deployment.yaml
	@ kubectl apply -f deployment/operator-rbac.yaml
	@ echo "Application deployed."

run-controller:
	@echo "Applying CRD and RBAC rules..."
	@ kubectl apply -f deployment/pinger-crd.yaml
	@ kubectl apply -f deployment/operator-rbac.yaml
	@ echo "Building and running controller..."
	@ cargo run --release
# Clean up
clean:
	@echo "Cleaning up..."
	kind delete cluster --name $(KIND_CLUSTER_NAME)
	@echo "Cleanup complete."