# System Patterns

This file documents recurring patterns and standards used in the project.
It is optional, but recommended to be updated as the project evolves.
YYYY-MM-DD HH:MM:SS - Log of updates made.

-

## Coding Patterns

1. Blueprint Runner Pattern

   - Uses builder pattern for initialization
   - Centralizes runtime configuration
   - Manages producer/consumer lifecycle

2. Context Pattern

   - Wraps BlueprintEnvironment with #[config]
   - Implements required traits (TangleClientContext, ServicesContext)
   - Manages internal clients and state

3. Docker Container Management (via Docktopus)
   - Fluent builder pattern for container configuration
   - Resource tiers (Small/Medium/Large) for allocation
   - Health check integration (30s timeout, 3 retries)
   - Port and volume binding standardization
   - Lifecycle: create → start → monitor → cleanup

## Architectural Patterns

1. Microservice Architecture

   - Modular job execution
   - Clear separation of concerns
   - Standardized communication patterns

2. Producer/Consumer Pattern

   - Event streaming from chain
   - Result submission back to chain
   - Async job processing

3. Container-Based Deployment
   - Docker integration via Docktopus
   - Standardized container lifecycle
   - Resource management tiers
   - Health monitoring

## Testing Patterns

1. Integration Testing

   - Uses TangleTestHarness
   - Simulates full node and runtime
   - Supports service setup and job execution

2. Container Testing

   - Integration tests using local Docker daemon
   - Lifecycle verification
   - Resource cleanup
   - Health check validation

3. Job Testing
   - Input/output validation
   - Service identity filtering
   - Event handling verification

## Resource Management Patterns

1. Container Resource Tiers

   - Small: 1 CPU, 1GB RAM, 5GB Storage
   - Medium: 2 CPU, 2GB RAM, 10GB Storage
   - Large: 4 CPU, 4GB RAM, 20GB Storage

2. Health Check Strategy
   - HTTP endpoint monitoring
   - 1s interval checks
   - 3s timeout per check
   - 3 retries before failure
   - 30s overall timeout

2024-04-24 18:46:00 - Added container resource management patterns and health check strategies
