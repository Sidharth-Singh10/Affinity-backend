# Affinity Backend

## Introduction
This repository contains the backend for the **Affinity** project, built using the [Axum](https://github.com/tokio-rs/axum) framework in Rust. The backend manages API requests and communicates with a PostgreSQL database, ensuring high performance and scalability.

## Tech Stack
- **Language**: Rust
- **Framework**: Axum
- **Database**: PostgreSQL
- **Containerization**: Docker
- **CI/CD**: Jenkins, Docker Compose

## Setup

### Prerequisites
Make sure you have the following installed:
- [Rust](https://www.rust-lang.org/tools/install)
- [PostgreSQL](https://www.postgresql.org/)
- [Docker](https://www.docker.com/) (optional, if running in a container)

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Sidharth-Singh10/Affinity-backend
   cd Affinity-backend
2. **Build and run the backend:**
   ```
   cargo build
   cargo run
   ```
### Running in Docker
   ```
   docker-compose up --build
   ```
## Contributing
   Read [CONTRIBUTING.md](https://github.com/Sidharth-Singh10/Affinity-backend/blob/main/CONTRIBUTING.md)
