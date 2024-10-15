# Contributing to Affinity Backend

We are excited that you want to contribute to **Affinity Backend**! This document will guide you through the contribution process, from reporting issues to submitting pull requests. Your contributions are vital for enhancing the project and fostering a collaborative community.

## Table of Contents
- [How to Contribute](#how-to-contribute)
- [Reporting Bugs](#reporting-bugs)
- [Suggesting Features](#suggesting-features)
- [Pull Request Process](#pull-request-process)
- [Code Guidelines](#code-guidelines)

## How to Contribute
You can contribute in several ways:
1. **Reporting Bugs**: Help us identify and resolve issues.
2. **Suggesting New Features**: Share your ideas to enhance functionality.
3. **Submitting Pull Requests**: Contribute code fixes or improvements.
4. **Improving Documentation**: Enhance clarity and comprehensibility of the project's documentation.

We welcome all contributions, but please ensure that your pull requests align with the project's vision and adhere to the coding standards outlined below.

## Reporting Bugs
If you find a bug, please open an issue and include the following information:
- **Title**: A concise and descriptive title of the issue.
- **Description**: A clear and detailed description of the issue.
- **Steps to Reproduce**: Specific instructions to replicate the bug.
- **Expected Behavior**: What you anticipated would happen.
- **Actual Behavior**: What actually happened.
- **Additional Information**: Relevant logs, screenshots, or code snippets to aid in understanding the issue.

### Bug Report Example
```
Title: Incorrect response when updating a resource

Description: When trying to update a resource with valid data, the API returns a 500 error.

Steps to Reproduce:
1. Make a PUT request to /api/resource/1 with valid JSON.
2. Observe the 500 error.

Expected behavior: The resource should be updated successfully with a 200 status code.

Actual behavior: The API returns a 500 error instead.

Environment:
- Rust version: 1.XX
- OS: Ubuntu 20.04
- PostgreSQL version: 13.3
```

## Suggesting Features
Feature suggestions are welcome! Please open an issue and provide:
- **Title**: A brief title for the feature.
- **Description**: A detailed description outlining the feature.
- **Use Cases**: Examples of how the feature could be beneficial.
- **Mockups**: Any design or mockup images that illustrate the idea (optional).

### Feature Request Example
```
Title: Add pagination to GET /api/resources

Description: Implement pagination to reduce the load when fetching large datasets. This will enhance user experience and improve performance for endpoints returning numerous resources.

Use cases:
- This would be particularly useful for endpoints that return a large number of resources, making it easier for clients to handle and process the data.
- Users can navigate through results more efficiently without overwhelming the interface.

Mockup: (Optional, add any design or mockup if relevant)
```

## Pull Request Process
1. **Fork the repository** and create your branch from `main`:
   ```bash
   git checkout -b feature/new-feature
   ```

2. **Make your changes**: Implement your bug fix or feature.

3. **Commit your changes** with clear and descriptive commit messages:
   ```bash
   git commit -m "Add feature: implement pagination"
   ```

4. **Push your branch** to your forked repository:
   ```bash
   git push origin feature/new-feature
   ```

5. **Open a pull request**:
   - Ensure your pull request title and description are clear and informative.
   - Reference any related issues or feature requests in your PR description.
   - Tag reviewers or maintainers, if necessary, for prompt feedback.

## Code Guidelines

### Rust Style Guidelines
- Follow Rust’s official [style guide](https://doc.rust-lang.org/book/ch01-01-installation.html).
- Run `cargo fmt` to format your code automatically before submitting.
- Run `cargo clippy` to maintain idiomatic code and catch common mistakes.

### Commit Message Format
We follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification for writing commit messages:
```
feat: add new feature to API
fix: resolve issue with database connection
docs: update README with usage instructions
style: format code
refactor: optimize performance
```

### Hacktoberfest & GSSOC
- Issues labeled `hacktoberfest` or `gssoc-ext` can be picked up by multiple contributors.
- PRs that are opened earliest and meet the quality standards will be prioritized for merging.

---

Thank you for considering contributing to **Affinity Backend**! Your contributions make a difference, and we appreciate your effort in helping improve our project. If you have any questions or need assistance, feel free to reach out!
