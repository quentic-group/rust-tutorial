# Rust Tutorial

The goal of this repository is to learn Rust by implementing a simple program that reads a file and counts the number of words in it.
This includes the environment of hacking, testing, debugging, building and the deployment. This project is also meant to investigate the integration of deployment and development tool in the Intellij landscape.

# Dev Environment

This project is organized in a monorepo structure, with each subproject in its own directory. This is an builtin feature of Rust, so we can use a `Cargo.toml` file foreach subproject and the whole project, too. Besides the structure of the project differs from the usual Rust project, the usage of `cargo` is the same as in a normal Rust project. It defines the the workspace and the dependencies of each subproject.

# Deployment

The deployment is oriented to the usage of Kubernetes (**k8s**) instead of Docker compose. This is meant to be an a verification if it is possible to use **one** deployment tool for the development, CI , test-stage and production stage. The deployment is done using **kustomize** and **helm**. The deployment is done in a way that it can be used in any cloud provider, but the usage of **k8s** is mandatory. For sure you are can the Docker image in different way, too, but this is out of the scope of this project.

```toml
[workspace]
members = [
    "subproject1",
    "subproject2"
]

[workspace.dependencies]
```

# Subprojects

## Actix Server

This demo is meant to show the usage of Actix server and how to deploy it on a server.

