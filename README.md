![🚧 Under construction 👷‍♂️](https://i.imgur.com/LEP2R3N.png)

# DevPulse

The DevPulse project is a comprehensive suite designed to analyze developer performance and repository contributions. It
includes an API that provides endpoints to assess code churn, identify top contributors, and evaluate individual
developer metrics across various repositories. The API supports multiple protocols including HTTP, gRPC, and WebSocket,
ensuring flexibility and scalability for diverse integration needs. DevPulse aims to offer actionable insights into
codebase health and developer efficiency, facilitating better project management and development practices. The project
may also be extended to include a CLI for enhanced accessibility and utility.

## 📋 TODO List

- [x] ✅ Setup CI for API release
- [ ] 🔄 Add versioning to the `http` API routes
- [ ] ⚙️ Develop `integrations/azure` for Azure DevOps
- [ ] 🐙 Develop `integrations/github` for GitHub Actions
- [ ] 💻 Develop `integrations/vscode` for Visual Studio Code
- [ ] 🚀 Setup CI client release using `openapi-generator` for `http` API

# Build Statuses

<details>
<summary>Integration CI - Annotation Service</summary>

| Branch    | Status                                                                                                                           | Details                                                                                                           |
|-----------|----------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------|
| `main`    | ![Build Status](https://dev.azure.com/geoffreygarrett/devpulse/_apis/build/status%2Fgeoffreygarrett.devpulse?branchName=main)    | [View Build Logs](https://dev.azure.com/geoffreygarrett/devpulse/_build/latest?definitionId=3&branchName=main)    |
| `develop` | ![Build Status](https://dev.azure.com/geoffreygarrett/devpulse/_apis/build/status%2Fgeoffreygarrett.devpulse?branchName=develop) | [View Build Logs](https://dev.azure.com/geoffreygarrett/devpulse/_build/latest?definitionId=3&branchName=develop) |

</details>

<details>
<summary>Rust Client CI</summary>

| Branch    | Status                                                                                                                 | Details                                                                                                                 |
|-----------|------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------|
| `main`    | ![Rust Status](https://github.com/geoffreygarrett/devpulse/actions/workflows/rust-client.yml/badge.svg?branch=main)    | [View Build Logs](https://github.com/geoffreygarrett/devpulse/actions/workflows/rust-client.yml?query=branch%3Amain)    |
| `develop` | ![Rust Status](https://github.com/geoffreygarrett/devpulse/actions/workflows/rust-client.yml/badge.svg?branch=develop) | [View Build Logs](https://github.com/geoffreygarrett/devpulse/actions/workflows/rust-client.yml?query=branch%3Adevelop) |

</details>

<details>
<summary>Python Client CI</summary>

| Branch    | Status                                                                                                                     | Details                                                                                                                   |
|-----------|----------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------|
| `main`    | ![Python Status](https://github.com/geoffreygarrett/devpulse/actions/workflows/python-client.yml/badge.svg?branch=main)    | [View Build Logs](https://github.com/geoffreygarrett/devpulse/actions/workflows/python-client.yml?query=branch%3Amain)    |
| `develop` | ![Python Status](https://github.com/geoffreygarrett/devpulse/actions/workflows/python-client.yml/badge.svg?branch=develop) | [View Build Logs](https://github.com/geoffreygarrett/devpulse/actions/workflows/python-client.yml?query=branch%3Adevelop) |

</details>

<details>
<summary>C# Client CI</summary>

| Branch    | Status                                                                                                                 | Details                                                                                                                   |
|-----------|------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------|
| `main`    | ![C# Status](https://github.com/geoffreygarrett/devpulse/actions/workflows/csharp-client.yml/badge.svg?branch=main)    | [View Build Logs](https://github.com/geoffreygarrett/devpulse/actions/workflows/csharp-client.yml?query=branch%3Amain)    |
| `develop` | ![C# Status](https://github.com/geoffreygarrett/devpulse/actions/workflows/csharp-client.yml/badge.svg?branch=develop) | [View Build Logs](https://github.com/geoffreygarrett/devpulse/actions/workflows/csharp-client.yml?query=branch%3Adevelop) |

</details>

<details>
<summary>TypeScript Client CI</summary>

| Branch    | Status                                                                                                                             | Details                                                                                                                       |
|-----------|------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------|
| `main`    | ![TypeScript Status](https://github.com/geoffreygarrett/devpulse/actions/workflows/typescript-client.yml/badge.svg?branch=main)    | [View Build Logs](https://github.com/geoffreygarrett/devpulse/actions/workflows/typescript-client.yml?query=branch%3Amain)    |
| `develop` | ![TypeScript Status](https://github.com/geoffreygarrett/devpulse/actions/workflows/typescript-client.yml/badge.svg?branch=develop) | [View Build Logs](https://github.com/geoffreygarrett/devpulse/actions/workflows/typescript-client.yml?query=branch%3Adevelop) |

</details>

## Prerequisites

- Rust and Cargo: Install from the [official Rust website](https://www.rust-lang.org/tools/install).
- Cargo Shuttle: Install using `cargo install cargo-shuttle`.

## Setup

Clone the repository and navigate into the project directory:

```bash
git clone https://github.com/geoffreygarrett/devpulse.git
cd devpulse
```

## Running the API Server

To start the API server locally, use the following Cargo Shuttle command:

### Bash

```bash
cargo shuttle run
```

### PowerShell

```powershell
cargo shuttle run
```

This will compile the project and start the server, making the API available locally for integration and testing.

## Testing the API

To test the API, you can send a request to analyze a range of commits in a repository using `curl`. Here’s how you can
do it:

### Bash

```bash
curl --request POST \
  --url http://127.0.0.1:8000/repository/commit-range?format=yaml \
  --header 'Content-Type: application/json' \
  --data '{
    "end_commit": "6b10ce3",
    "repository_url": "https://github.com/bazelbuild/rules_rust",
    "start_commit": "6c2bd67"
}'
```

### PowerShell

```powershell
curl --request POST `
  --url http://127.0.0.1:8000/repository/commit-range?format=yaml `
  --header 'Content-Type: application/json' `
  --data '{
    "end_commit": "6b10ce3",
    "repository_url": "https://github.com/bazelbuild/rules_rust",
    "start_commit": "6c2bd67"
}'
```

## Documentation

For more detailed documentation, including all available endpoints and their parameters, please refer to
the [official DevPulse documentation](https://devpulse.shuttleapp.rs).

## Support

For support, issues, or contributions, please visit
the [project repository](https://github.com/geoffreygarrett/devpulse).
