# Contributing to clau.rs

Thank you for your interest in contributing to clau.rs! This document provides guidelines and information for contributors.

## Code of Conduct

By participating in this project, you agree to abide by our code of conduct: be respectful, inclusive, and constructive in all interactions.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/clau.rs.git`
3. Create a new branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test`
6. Commit your changes
7. Push to your fork
8. Open a pull request

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Claude Code CLI installed
- Valid Anthropic API key (for integration tests)

### Building

```bash
cargo build
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with verbose output
cargo test -- --nocapture
```

### Code Style

- Follow Rust standard style guidelines
- Use `cargo fmt` before committing
- Ensure `cargo clippy` passes without warnings
- Add documentation for public APIs
- Include tests for new functionality

## Project Structure

- `clau-core/`: Core types and traits
- `clau-runtime/`: Process management and runtime
- `clau-mcp/`: MCP protocol implementation
- `clau-macros/`: Procedural macros
- `clau/`: Main SDK crate
- `examples/`: Usage examples

## Pull Request Process

1. Ensure all tests pass
2. Update documentation as needed
3. Add an entry to CHANGELOG.md (if applicable)
4. Ensure your commits are well-described
5. Link any related issues

## Areas for Contribution

- Implementing missing features from the spec
- Adding more examples
- Improving documentation
- Writing tests
- Performance optimizations
- Bug fixes

## Questions?

Feel free to open an issue for questions or discussions about potential contributions.