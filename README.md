# `ls_rust`

`ls_rust` is a simple Rust-based implementation of the `ls` command, which lists files and directories. This project demonstrates building a Rust application for Linux (musl), packaging it inside a Docker container, and running it with various testing methods.

## Features

- Written in Rust
- Built for `x86_64-unknown-linux-musl` to support static linking
- Dockerized for portability and ease of use

## Prerequisites

To build and run `ls_rust`, you'll need the following tools installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/)
- [task CLI](https://taskfile.dev/installation) (for automation with Taskfile, optional)

## Building and Running

For convenience, the project includes a Taskfile.yaml for automating common tasks like building, running, and testing. If you have the task CLI installed, you can use the following commands:

```
task build-docker-image      # Build the Docker image
task run-docker              # Run the Docker container
task compare-sizes           # Compare the size of native 'ls' and 'ls_rust'
task compare-times           # Compare the execution time of native 'ls' and 'ls_rust'
task full-workflow           # Run the full workflow
```

run `task help` to view all options

See the [Taskfile](https://taskfile.dev) documentation for more details.# ls_rust
