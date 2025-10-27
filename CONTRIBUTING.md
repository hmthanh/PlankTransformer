# Contributing to PlankTransformer

Thank you for your interest in contributing to PlankTransformer! This document provides guidelines and instructions for contributing.

## Code of Conduct

Be respectful and inclusive. We're all here to build something great together.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/PlankTransformer.git
   cd PlankTransformer
   ```
3. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/hmthanh/PlankTransformer.git
   ```
4. **Create a branch** for your changes:
   ```bash
   git checkout -b feature/my-feature
   ```

## Development Setup

### Prerequisites

- Rust 1.70+ (`rustup`)
- For specific platforms, see the respective README files

### Building

```bash
# Build everything
./build-all.sh

# Or build specific components
cd crates/transformer-core && cargo build
cd apps/desktop && cargo build
```

### Running Tests

```bash
# Run all tests
./test-all.sh

# Or test specific components
cd crates/transformer-core && cargo test
```

### Code Style

We follow standard Rust conventions:

```bash
# Format code
cargo fmt --all

# Check with clippy
cargo clippy --all-targets --all-features
```

## Making Changes

### Commit Messages

Write clear, descriptive commit messages:

```
Add feature X to component Y

- Detailed explanation of what changed
- Why the change was needed
- Any breaking changes or migration notes
```

### Pull Request Process

1. **Update your branch** with latest upstream:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Ensure tests pass**:
   ```bash
   cargo test --all
   ```

3. **Format and lint**:
   ```bash
   cargo fmt --all
   cargo clippy --all-targets --all-features -- -D warnings
   ```

4. **Push to your fork**:
   ```bash
   git push origin feature/my-feature
   ```

5. **Create a Pull Request** on GitHub

6. **Respond to review feedback**

### PR Checklist

- [ ] Tests pass locally
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings
- [ ] Documentation updated if needed
- [ ] CHANGELOG.md updated (if applicable)
- [ ] Commit messages are clear

## Areas to Contribute

### High Priority

- [ ] GPU kernel optimizations
- [ ] Performance benchmarks
- [ ] Additional platform examples
- [ ] Documentation improvements

### Good First Issues

Look for issues labeled `good-first-issue` in the GitHub issue tracker.

### Ideas for Contributions

- Implement quantization (int8, fp16)
- Add more transformer architectures
- Improve error messages
- Add GUI applications for desktop
- Create example models
- Write tutorials
- Improve CI/CD pipelines

## Platform-Specific Contributions

### Web (WASM)
- Location: `apps/web/`
- Test: Build with `wasm-pack` and test in browser
- Requirements: Modern browser with WebGPU

### iOS
- Location: `apps/ios/`
- Test: Build and run in Xcode simulator or device
- Requirements: macOS with Xcode

### Android
- Location: `apps/android/`
- Test: Build and run in Android Studio emulator or device
- Requirements: Android Studio, NDK

### Desktop
- Location: `apps/desktop/`
- Test: `cargo run --release`
- Requirements: Platform-specific GPU drivers

## Documentation

- Keep README files up to date
- Add inline documentation for public APIs
- Update DOCUMENTATION.md for architectural changes
- Include examples in docstrings

## Testing Guidelines

### Unit Tests

Place unit tests in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Test code
    }
}
```

### Integration Tests

For complex integrations, create tests in `tests/` directory.

### GPU Tests

Mark GPU-dependent tests with `#[ignore]` for CI:

```rust
#[test]
#[ignore] // Requires GPU
fn test_gpu_feature() {
    // Test code
}
```

## Performance Considerations

- Profile before optimizing
- Benchmark performance-critical code
- Consider memory usage for mobile platforms
- Test on multiple GPU backends when possible

## Security

- Run `cargo audit` before submitting
- Don't commit secrets or API keys
- Be mindful of unsafe code
- Report security issues privately

## Questions?

- Open an issue for questions
- Join our Discord community
- Email: support@planktransformer.dev

## License

By contributing, you agree that your contributions will be licensed under the same terms as the project (MIT OR Apache-2.0).

---

Thank you for contributing to PlankTransformer! 🚀
