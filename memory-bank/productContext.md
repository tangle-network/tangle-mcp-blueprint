# Product Context

This file provides a high-level overview of the project and the expected product that will be created. Initially it is based upon projectBrief.md (if provided) and all other available project-related information in the working directory. This file is intended to be updated as the project evolves, and should be used to inform all other modes of the project's goals and context.
YYYY-MM-DD HH:MM:SS - Log of updates made will be appended as footnotes to the end of this file.

-

## Project Goal

The blueprint's goal is to run an MCP server (built in Typescript) inside a Docker container. It provides two main jobs:

1. `create_project`: starts a container running the MCP server.
2. `destroy_project`: deletes the container and its state.

## Key Features

1. Modular Job Execution System

   - Job Router for mapping numeric job IDs to logic handlers
   - BlueprintRunner for core execution orchestration
   - TangleProducer for streaming finalized blocks/events
   - TangleConsumer for signing and sending results

2. Docker Integration (via Docktopus)

   - Container lifecycle management
   - Volume and port mapping support
   - Restart policy configuration
   - Custom networking options
   - Resource tier management (Small/Medium/Large)
   - Health check monitoring system

3. Standardized Project Structure

   - Binary crate for runner initialization
   - Library crate for business logic
   - Contracts directory for Solidity integration
   - Optional frontend support

4. Resource Management

   - Three-tier resource allocation system
   - Automated health check monitoring
   - SSE endpoint for container status
   - Standardized cleanup procedures

5. Container Health Monitoring
   - Real-time health status tracking
   - Configurable check intervals and timeouts
   - Automatic failure detection and reporting
   - Integration with job lifecycle management

## Overall Architecture

The project follows a microservice architecture with:

1. Core Components:

   - Job Router (control flow)
   - BlueprintRunner (execution engine)
   - Context Management (state & configuration)

2. Integration Layer:

   - Producer/Consumer pattern
   - Docker container orchestration
   - Smart contract integration

3. Development Structure:
   - Strict directory organization
   - Clear separation of concerns
   - Standardized testing approach

[2024-04-25 16:15:06] - Updated product context with resource management features and health monitoring system
