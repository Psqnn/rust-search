# Contributing to rust-search

Thank you for your interest in contributing to rust-search! 🚀

We welcome contributions from everyone, whether it's code, documentation, bug reports, or feature suggestions.

## Code of Conduct

Be respectful, inclusive, and constructive. We have zero tolerance for harassment, discrimination, or toxic behavior.

## Getting Started

### Prerequisites
- Rust 1.70+
- Git
- Cargo

### Local Setup
```bash
# Fork & clone
git clone https://github.com/YOUR_USERNAME/rust-search
cd rust-search

# Install dependencies
cargo build

# Run tests
cargo test

# Run linter
cargo clippy

# Format code
cargo fmt
```

## How to Contribute

### 1. Bug Reports

Found a bug? Please open an issue:

```
Title: [BUG] Short description
Description:
- What happened
- What you expected
- Steps to reproduce
- Environment (OS, Rust version)
```

**Example:**
```
Title: [BUG] Indexing crashes on symlinks
Description:
When indexing /home with symlinks, the program crashes with "permission denied"
Rust: 1.75.0
OS: Linux (Ubuntu 22.04)
```

### 2. Feature Requests

Have an idea? Open a discussion:

```
Title: [FEATURE] Short description
Description:
- Problem you're solving
- Proposed solution
- Alternatives considered
```

### 3. Code Contributions

#### Step 1: Create a Feature Branch
```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/bug-description
```

**Naming convention:**
- `feature/stemmer-russian` ✅
- `fix/index-crash` ✅
- `docs/api-reference` ✅
- `refactor/db-structure` ✅

#### Step 2: Make Your Changes
```bash
# Make changes
vim src/searcher.rs

# Run tests
cargo test

# Check formatting
cargo fmt --check
cargo clippy

# Fix formatting
cargo fmt
```

#### Step 3: Commit
```bash
git add .
git commit -m "feat: add Russian stemming support"
```

**Commit message format:**
```
<type>: <description>

<optional body>

Fixes #123
```

**Types:**
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation
- `style:` Formatting/cleanup
- `refactor:` Code restructuring
- `test:` Test additions/fixes
- `perf:` Performance improvements

**Examples:**
```
feat: implement BM25 ranking algorithm
fix: handle symlinks in indexing
docs: add architecture guide
perf: optimize term lookup with caching
```

#### Step 4: Push & Create PR
```bash
git push origin feature/your-feature-name
```

Then open a Pull Request on GitHub with:
- Clear description of changes
- References to related issues (`Fixes #123`)
- Screenshots/benchmarks (if applicable)

#### Step 5: Code Review
- Address feedback promptly
- Keep commits clean and organized
- Run tests before pushing updates

## Code Standards

### Rust Style
```rust
// ✅ Good
pub fn search_files(query: &str, limit: usize) -> Vec<FileResult> {
    // Implementation
}

// ❌ Bad
pub fn search_files(q:&str,l:usize)->Vec<FileResult>{
    // Implementation
}
```

### Documentation
```rust
/// Searches the index for matching files.
///
/// # Arguments
/// * `query` - The search query
/// * `limit` - Maximum results to return
///
/// # Returns
/// Vector of matching FileResult ordered by relevance
///
/// # Examples
/// ```
/// let results = search_files("setup.py", 10);
/// ```
pub fn search_files(query: &str, limit: usize) -> Vec<FileResult> {
    // Implementation
}
```

### Testing
```bash
# Unit tests
cargo test

# Integration tests
cargo test --test '*'

# With output
cargo test -- --nocapture

# Specific test
cargo test search_files
```

## Testing Requirements

All contributions must include tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_finds_exact_match() {
        // Test implementation
    }

    #[test]
    fn test_empty_query_returns_nothing() {
        // Test implementation
    }
}
```

## Documentation

- Update README.md for user-facing changes
- Add doc comments to public APIs
- Update ARCHITECTURE.md for architectural changes
- Include examples in doc comments

## Licensing

**Important:** All contributions must comply with dual-licensing:

1. **Your code is GPL-3.0** (open source part)
2. **We may include in Commercial License** (with your acknowledgment)

By submitting a PR, you agree that:
- Your code is original work
- You grant us rights to use it under GPL-3.0 and Commercial License
- You won't claim compensation later

## Commits to Avoid

❌ Large refactors without discussion
❌ Breaking API changes without issue
❌ Adding heavy dependencies without discussion
❌ Removing tests or documentation
❌ Formatting-only commits mixed with logic

## Priority Areas

We especially welcome contributions in:
1. **Snowball stemmer** (Russian/English)
2. **BM25 ranking** improvements
3. **GUI** (Tauri desktop)
4. **Performance** optimizations
5. **Documentation** & examples

## Questions?

- 💬 Open a GitHub Discussion
- 📧 Email: ppasa5684@gmail.com
- 🐛 Found a security issue? Email immediately (don't open public issue)

## Recognition

Contributors will be recognized in:
- CONTRIBUTORS.md
- GitHub releases
- Project website

---

**Thank you for making rust-search better!** 🙏
