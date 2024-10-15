# Affinity Backend

## Introduction
Welcome to the **Affinity Backend** repository! This project serves as the backend for the **Affinity** application, built using the [Axum](https://github.com/tokio-rs/axum) framework in Rust. The backend is designed to efficiently manage API requests and communicate with a PostgreSQL database, ensuring high performance and scalability.

---

## Tech Stack
- **Programming Language**: Rust
- **Web Framework**: Axum
- **Database**: PostgreSQL
- **Containerization**: Docker
- **CI/CD Tools**: GitHub Actions, Jenkins, Docker Compose

---

## Setup

### Prerequisites
Before you begin, ensure you have the following software installed:
- [Rust](https://www.rust-lang.org/tools/install)
- [PostgreSQL](https://www.postgresql.org/)
- [Docker](https://www.docker.com/) (optional, if you prefer running the application in a container)

### Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/Sidharth-Singh10/Affinity-backend
   cd Affinity-backend
   ```

2. **Build and run the backend**:
   ```bash
   cargo build
   cargo run
   ```

### Running in Docker
To run the backend using Docker, ensure you complete the following:
- The `Dockerfile` is currently incomplete; please add the required environment variables.
- The `docker-compose.yml` file also needs to be completed for proper setup.
- Ensure that the database URL is properly configured in the backend's environment variables.

### Running in Kubernetes
To deploy the application in a Kubernetes environment:
```bash
helm install <app-name> ./helm_charts
```
- This command will set up the application without requiring a separate database setup.
- Make sure to configure `values.json` before running the command to customize your deployment.

---

## Contributing
We welcome contributions! Please read our [CONTRIBUTING.md](https://github.com/Sidharth-Singh10/Affinity-backend/blob/main/CONTRIBUTING.md) for guidelines on how to contribute to this project.

---

## Affinity Project Overview

1. **Architecture** (deprecated)
   - Below is the architectural diagram of the Affinity project:
   ![Affinity Project Architecture](https://github.com/user-attachments/assets/402a9b69-eccd-478b-9c0e-50810200c28d)

2. **Related Links**:
   - Frontend Repository: [Affinity Frontend](https://github.com/rishyym0927/Affinity_frontend)

---

## Our Valuable Contributors ❤️✨

Thank you to all our contributors for their hard work and dedication!  
[![Contributors](https://contrib.rocks/image?repo=Sidharth-Singh10/Affinity-backend)](https://github.com/Sidharth-Singh10/Affinity-backend/graphs/contributors)
