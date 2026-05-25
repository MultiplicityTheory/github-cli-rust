# GitHub CLI (Rust)

A high-performance, memory-safe, and idiomatic Rust implementation of the GitHub CLI (`gh`). This tool provides a command-line interface for interacting with GitHub, mirroring the core functionality of the official Go-based CLI while leveraging Rust's performance and safety.

## 🚀 Key Features

- **Blazing Fast**: Native Rust implementation for rapid startup and execution.
- **Smart Repo Resolution**: Automatically detects the upstream GitHub repository from your local Git remotes.
- **Interactive TUI**: Rich, searchable selection lists for Pull Requests and Issues using `inquire`.
- **Beautiful Formatting**: Clean, professional table outputs powered by `tabled`.
- **Secure Authentication**: Built-in support for OAuth2 device flow and secure token storage.
- **Async Foundation**: Built on `tokio` and `reqwest` for efficient, non-blocking API interactions.

## 🛠️ Built-in Commands

| Command | Description |
| :--- | :--- |
| `auth login` | Log in to your GitHub account using the device flow. |
| `auth status` | View your current authentication status and logged-in user. |
| `repo view` | View details about a repository (current or specified). |
| `repo clone` | Clone a GitHub repository locally. |
| `issue list` | List open issues in a repository. |
| `issue view` | View details and body of an issue (interactive selection if ID omitted). |
| `pr list` | List open pull requests in a repository. |
| `pr view` | View details and body of a pull request (interactive selection if ID omitted). |

## 📦 Installation

### Prerequisites

- **Rust**: [Install Rust](https://www.rust-lang.org/tools/install) (version 1.80+)
- **Git**: Ensure the `git` binary is in your PATH.

### Building from Source

1. Clone this repository (or navigate to the project folder).
2. Build the production binary:

```bash
cd github-cli-rust
cargo build --release
```

The optimized binary will be available at `./target/release/gh-rs`.

## ⚙️ User Setup Guide

### 1. Initial Login

Start by authenticating with your GitHub account:

```bash
./target/release/gh-rs auth login
```

Follow the on-screen instructions to visit the verification URL and enter the provided code.

### 2. Check Status

Verify that you are successfully logged in:

```bash
./target/release/gh-rs auth status
```

### 3. Basic Usage

View the current repository (auto-detected from your CWD):

```bash
./target/release/gh-rs repo view
```

List open pull requests:

```bash
./target/release/gh-rs pr list
```

View a specific issue interactively:

```bash
./target/release/gh-rs issue view
```

## 🛡️ Security & Privacy

- **Safe Storage**: Tokens are stored in a `hosts.yml` file following XDG Base Directory specifications.
- **No Telemetry by Default**: All local data stays on your machine. (Optional background telemetry is available but strictly controlled).

## 📄 License

MIT License. See `LICENSE` for details.
