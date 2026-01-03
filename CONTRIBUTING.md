# Contributing to File Sorter

Thank you for your interest in contributing! This document provides guidelines for contributing to the project.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/file-sorter.git`
3. Install dependencies: `npm install`
4. Create a branch: `git checkout -b feature/your-feature-name`

## Development Setup

### Prerequisites
- Node.js v18+
- Rust (latest stable)
- Platform-specific image libraries (see README.md)

### Running Locally
```bash
./dev.sh
```

## Code Style

### Rust
- Follow standard Rust conventions
- Run `cargo fmt` before committing
- Run `cargo clippy` to catch common issues
- Add tests for new functionality

### TypeScript/Preact
- Use functional components with hooks
- Type all props and state
- Follow existing component patterns

## Testing

All new features should include tests:

```bash
# Run Rust tests
cd src-tauri && cargo test

# Add integration tests for new processors
```

## Pull Request Process

1. Update README.md if adding new features
2. Add tests for new functionality
3. Ensure all tests pass
4. Update DEVELOPMENT.md if changing architecture
5. Write clear commit messages
6. Reference any related issues

## Commit Messages

Use conventional commits format:
- `feat: Add new feature`
- `fix: Fix bug in image converter`
- `docs: Update README`
- `test: Add collision tests`
- `refactor: Simplify queue processor`

## Reporting Bugs

Use GitHub Issues and include:
- OS and version
- Steps to reproduce
- Expected vs actual behavior
- Sample files if applicable (without sensitive content)

## Feature Requests

Open an issue with:
- Clear description of the feature
- Use cases
- Potential implementation approach

## Questions?

Open a discussion on GitHub or contact the maintainers.

Thank you for contributing! 🎉
