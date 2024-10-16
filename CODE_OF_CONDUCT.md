Welcome to the Affinity Backend repository! We strive to maintain a collaborative, inclusive, and respectful environment for all contributors. This Code of Conduct outlines our expectations and responsibilities to ensure a positive experience for everyone involved.  

---

## 1. **Introduction**  
This repository contains the backend for the Affinity project, built using the **Axum framework** in **Rust**. It manages API requests and communicates with a **PostgreSQL** database, focusing on performance and scalability.  

---

## 2. **Our Values**  
We believe in:  
- **Respect and Empathy:** Treat others with kindness, empathy, and respect.
- **Collaboration and Growth:** We value constructive feedback and collaboration that helps us grow as a community and project.
- **Inclusion and Diversity:** Everyone is welcome, regardless of background, identity, or expertise.

---

## 3. **Unacceptable Behavior**  
- Use of offensive, derogatory, or discriminatory language.
- Harassment, bullying, or personal attacks.
- Trolling or disruptive comments.
- Sharing sensitive information without consent.
- Failing to respect project boundaries, maintainers, and contributors’ decisions.

---

## 4. **Reporting Issues**  
If you encounter behavior that does not align with this Code of Conduct, please report it by:  
- **Contacting Maintainers:** You can open an issue marked as “Code of Conduct” or contact a maintainer privately.
- **Anonymous Reporting:** (Add process here if applicable, e.g., via email or form.)

All reports will be reviewed and handled with confidentiality and care.  

---

## 5. **Contribution Guidelines**  
To ensure smooth collaboration, follow these steps:  
1. **Read** [CONTRIBUTING.md](./CONTRIBUTING.md) before making contributions.  
2. **Fork** the repository and make changes via feature branches.  
3. **Pull Requests:** Ensure all PRs are reviewed and meet project guidelines.  

---

## 6. **Project Setup & Tech Stack**  

**Tech Stack:**  
- **Language:** Rust  
- **Framework:** Axum  
- **Database:** PostgreSQL  
- **Containerization:** Docker  
- **CI/CD Tools:** GitHub Actions, Jenkins, Docker Compose  

### Prerequisites  
Make sure you have the following installed:  
- Rust  
- PostgreSQL  
- Docker (optional, for containerized setup)  

### Installation  
1. Clone the repository:  
   ```bash
   git clone https://github.com/Sidharth-Singh10/Affinity-backend  
   cd Affinity-backend  
   ```  
2. Build and run the backend:  
   ```bash
   cargo build  
   cargo run  
   ```  

### Running in Docker  
- The **Dockerfile** and **docker-compose.yml** require updates (e.g., setting environment variables).  
- Ensure the **database URL** is correctly configured in the backend environment.  

### Running in Kubernetes  
1. Install with Helm:  
   ```bash
   helm install <app-name> ./hell_charts  
   ```  
2. Before deploying, configure the necessary values in `values.json`.  

---

## 7. **Frontend Repository**  
You can find the Affinity Frontend repository here:  
[Affinity Frontend](https://github.com/rishyym0927/Affinity_frontend)  

---

## 8. **Enforcement**  
Any violation of this Code of Conduct may result in:  
- **Warning** from maintainers.  
- **Temporary suspension** from contributing to the project.  
- **Permanent removal** from the project in extreme cases.

---

By contributing to this project, you agree to abide by this Code of Conduct and foster a positive community.  

Thank you for helping make the Affinity Backend a welcoming space!
