# Pinger Kubernetes Controller

This project implements a Kubernetes controller in Rust that manages "Pinger" custom resources. Each Pinger resource specifies a URL that the controller will periodically ping and update its status.

## Project Structure

- `src/main.rs`: Contains the main controller logic, including the reconciliation loop and error handling.
- `src/lib.rs`: Defines the Pinger custom resource structure.
- `src/error.rs`: Defines custom error types for the project.
- `Cargo.toml`: Specifies the project dependencies.
- `pinger.yml`: An example Pinger custom resource definition.

## Dependencies

This project uses the following main dependencies:

- `kube`: For interacting with the Kubernetes API
- `tokio`: For asynchronous runtime
- `serde`: For serialization and deserialization
- `reqwest`: For making HTTP requests

For a full list of dependencies, see the `Cargo.toml` file.

## Building the Project

To build the project, run:
