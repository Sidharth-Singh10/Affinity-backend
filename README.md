![AffinityBanner](https://github.com/user-attachments/assets/e207e037-b436-4007-a0a6-8bc73e0a99dd)

Welcome to the **Affinity** backend repository! This project is powered by the blazing-fast [Axum](https://github.com/tokio-rs/axum) framework, built using **Rust** to handle API requests and efficiently communicate with a **PostgreSQL** database. Our backend is optimized for high performance and scalability, designed to meet the demands of modern applications.


## PerOxO Module – Microservices & Actor Concurrency Engine
PerOxO is a core backend engine within the Affinity system designed for high-throughput real-time features like messaging and presence.

 Key Highlights:
- **Actor Model Concurrency:** Built using Rust’s asynchronous capabilities to ensure isolated, concurrent actor execution per user session.
- **Microservice Architecture:** Operates as an independent service, enabling decoupled scaling and service separation.
- **gRPC Communication:** Interfaces with other Affinity services using performant, schema-defined gRPC calls.
- **WebSocket Integration:** Handles persistent user connections, routes messages, and manages session lifecycles.
This module lives as a subproject under ./per_oxo/ and can be built, tested, and deployed independently.

## Tech Stack
- **Language**: Rust
- **Framework**: Axum

- **Database**: PostgreSQL,ScyllaDB
- **Containerization**: Docker
- **CI/CD**: GitHub Actions, Jenkins,
- **Orchestration**: Kubernetes with Helm
 
## 📋 Setup

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
   ```

2. **Build and run the backend:**

   ```bash
   cargo build
   cargo run
   ```


### Running in Docker 🐳

To run the project in Docker, ensure that the environment variables for database connection are properly set. Follow these steps:


1. **Using Docker Compose:**

   Update the `docker-compose.yml` file with the correct environment variables, and then:

   ```bash
   docker-compose up
   ```

### Running on Kubernetes with Helm ⛵

Deploy the project on a Kubernetes cluster using Helm:

1. **Configure `values.yaml`:**

   Make sure your `values.yaml` file is properly set up with the required database configuration.

2. **Install the application with Helm:**

   ```bash
   helm install <app-name> ./helm_charts
   ```


## 🌱 Contributing

We welcome contributions! Please check out our [CONTRIBUTING.md](https://github.com/Sidharth-Singh10/Affinity-backend/blob/main/CONTRIBUTING.md) to get started. Contributions are expected to follow good practices, maintain code quality, and align with the project’s objectives.


## 🏗️ Affinity Project Overview
1. **PerOxO Architecture**:

 ![Screenshot_20250618_112813](https://github.com/user-attachments/assets/f8ab4a55-ebd2-4242-855d-d2beea062b6d)
2. **Affinity Architecture** (deprecated)
   
   ![Architecture](https://github.com/user-attachments/assets/402a9b69-eccd-478b-9c0e-50810200c28d)

3. **BackendDevOps**

 ![BackendDevOps](/BackendOps.png)

3. **Frontend Repository**:  
   The frontend code for this project is available at:  
   [Affinity-Frontend](https://github.com/rishyym0927/Affinity_frontend)

4. **Discord Bot Repository**
   The code of discord bot used in our server is available at:                                                                                                              
   [Discord-bot](https://github.com/Sidharth-Singh10/affinity-bot)


## ❤️ Our Valuable Contributors
<div align="center">
  <img src="https://github.com/user-attachments/assets/c7f169de-e8e9-418e-a08e-c69b34c23a41" alt="Affinity Architecture" width="200"/>
</div>



[![Contributors](https://contrib.rocks/image?repo=Sidharth-Singh10/Affinity-backend)](https://github.com/Sidharth-Singh10/Affinity-backend/graphs/contributors)

 <img src="https://user-images.githubusercontent.com/74038190/212284100-561aa473-3905-4a80-b561-0d28506553ee.gif" width="900">

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/Sidharth-Singh10/Affinity-backend/blob/main/LICENSE) file for details.


This version includes all necessary build, run, and deployment instructions formatted as code blocks for ease of use.
