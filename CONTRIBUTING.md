# Contributing to LendSphere

Thank you for your interest in contributing to LendSphere! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Documentation](#documentation)
- [Security](#security)
- [Issue Reporting](#issue-reporting)
- [Pull Request Process](#pull-request-process)

## Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inclusive environment for all contributors. Please read and follow our [Code of Conduct](CODE_OF_CONDUCT.md).

## Getting Started

### Prerequisites

- Node.js 16+ 
- Rust 1.70+ (for smart contracts)
- Docker
- Git

### Development Setup

1. **Fork the Repository**
   ```bash
   # Fork the repository on GitHub
   git clone https://github.com/YOUR_USERNAME/LendSphere.git
   cd LendSphere
   ```

2. **Install Dependencies**
   ```bash
   # Install frontend dependencies
   cd frontend
   npm install
   
   # Install contract dependencies
   cd ../contracts
   cargo install soroban-cli
   ```

3. **Set Up Development Environment**
   ```bash
   # Start local development network
   npm run dev:local
   
   # Run tests
   npm run test
   ```

## Development Workflow

### 1. Create an Issue

Before starting work, create an issue to discuss your proposed changes. This helps avoid duplicate work and ensures alignment with project goals.

### 2. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-bug-fix
```

### 3. Make Changes

Follow our coding standards and ensure your changes are well-tested.

### 4. Test Your Changes

```bash
# Run all tests
npm run test

# Run contract tests
cd contracts && cargo test

# Run frontend tests
cd frontend && npm test
```

### 5. Submit a Pull Request

Create a pull request with:
- Clear description of changes
- Reference to related issues
- Test results
- Documentation updates if needed

## Coding Standards

### Smart Contracts (Rust/Soroban)

- Follow Rust best practices and idioms
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Include comprehensive comments for public functions
- Add unit tests for all public functions
- Use meaningful variable and function names

```rust
// Good example
pub fn calculate_interest_rate(
    env: &Env,
    cash: BigInt,
    borrows: BigInt,
) -> Result<BigInt, ContractError> {
    // Implementation with proper error handling
}

// Bad example
pub fn calc_ir(c: BigInt, b: BigInt) -> BigInt {
    // Poor naming and no error handling
}
```

### Frontend (TypeScript/React)

- Use TypeScript for type safety
- Follow React best practices
- Use meaningful component and variable names
- Include PropTypes or TypeScript interfaces
- Add comments for complex logic

```typescript
// Good example
interface UserPosition {
  suppliedAmount: BigInt;
  borrowedAmount: BigInt;
  healthFactor: number;
}

const PositionCard: React.FC<{ position: UserPosition }> = ({ position }) => {
  return (
    <Card>
      <CardContent>
        <p>Supplied: {position.suppliedAmount.toString()}</p>
        <p>Borrowed: {position.borrowedAmount.toString()}</p>
      </CardContent>
    </Card>
  );
};
```

### Code Style

- Use 2 spaces for indentation
- Keep lines under 100 characters
- Use descriptive commit messages
- Follow conventional commits format

## Testing

### Smart Contract Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_supply_tokens() {
        let env = Env::default();
        // Test implementation
    }
}
```

### Frontend Testing

```typescript
import { render, screen } from '@testing-library/react';
import Dashboard from './Dashboard';

test('renders dashboard', () => {
  render(<Dashboard />);
  expect(screen.getByText('Dashboard')).toBeInTheDocument();
});
```

### Test Coverage

- Aim for >80% code coverage
- Test all public functions and components
- Include edge cases and error scenarios
- Use integration tests for critical workflows

## Documentation

### Code Documentation

- Document all public APIs
- Include parameter descriptions
- Provide usage examples
- Update README for new features

### API Documentation

- Use OpenAPI/Swagger for backend APIs
- Include request/response examples
- Document error codes and messages

## Security

### Security Best Practices

- Never commit sensitive information
- Use environment variables for secrets
- Follow security guidelines for smart contracts
- Report security vulnerabilities privately

### Smart Contract Security

- Use established patterns and libraries
- Implement proper access controls
- Add emergency pause mechanisms
- Conduct thorough testing

### Reporting Security Issues

If you discover a security vulnerability, please email us at security@lendsphere.xyz with details.

## Issue Reporting

### Bug Reports

Use the bug report template and include:
- Clear description of the issue
- Steps to reproduce
- Expected vs actual behavior
- Environment details
- Screenshots if applicable

### Feature Requests

Use the feature request template and include:
- Clear description of the feature
- Use case and benefits
- Implementation suggestions
- Potential challenges

## Pull Request Process

### Before Submitting

1. **Code Review**: Self-review your changes
2. **Testing**: Ensure all tests pass
3. **Documentation**: Update relevant documentation
4. **Formatting**: Run code formatters and linters

### Pull Request Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] All tests pass
- [ ] New tests added
- [ ] Manual testing completed

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] Security considerations addressed
```

### Review Process

1. **Automated Checks**: CI/CD pipeline runs tests and linting
2. **Peer Review**: At least one maintainer reviews the PR
3. **Testing**: Changes are tested in staging environment
4. **Merge**: PR is merged after approval

## Getting Help

- **Discord**: Join our community for discussions
- **GitHub Issues**: For bug reports and feature requests
- **Documentation**: Check our docs for guidance
- **Email**: Contact us at contribute@lendsphere.xyz

## Recognition

Contributors are recognized in:
- README contributors section
- Release notes
- Community announcements
- Annual contributor awards

## License

By contributing to LendSphere, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to LendSphere! Your contributions help make decentralized lending accessible to everyone.
