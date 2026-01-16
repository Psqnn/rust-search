# rust-search

**🚀 High-performance full-text search engine powered by sled BwTree.** Open source alternative to everything.sh. **Dual-licensed: GPL-3.0 + Commercial.**

[![GitHub stars](https://img.shields.io/github/stars/psqnn/rust-search)](https://github.com/psqnn/rust-search)
[![License: Dual](https://img.shields.io/badge/License-Dual-brightgreen.svg)](https://github.com/psqnn/rust-search/blob/main/LICENSE.GPL-3.0)

## ✨ Features
- **Blazing fast** indexing with async Tokio
- **Minimal memory**
- **Single file** database (`results.db`)
- **Cli-usage** In roadmap Gui
## 🎯 Quickstart
```bash
git clone https://github.com/rust-search
cd rust-search
```
or compile from source
```bash
# Build from source
git clone https://github.com/psqnn/rust-search
cd rust-search
cargo build --release

# Index your files
./target/release/rust-search index /home

# Search instantly
./target/release/rust-search search "filename"
./target/release/rust-search search "*.rs"
./target/release/rust-search search "function async"
```

## 🏗️ Architecture
```
results.db (sled BwTree)
├── files: path → FileInfo {size, hash, mtime}
├── terms: word → [file_id] (inverted index)
└── metadata: stats, config
```

**Search pipeline:**
```
query → tokenize → lookup terms → retrieve file_ids → rank → results
```

##  Performance
- **Fast indexing** with async I/O (Tokio)
- **Minimal storage** efficient B-Tree layout
- **Quick queries** in-memory term lookups
- **Responsive CLI** real-time progress

##  Building from Source

```bash
# Requirements: Rust 1.70+
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone & build
git clone https://github.com/psqnn/rust-search
cd rust-search
cargo build --release

# Binary location
./target/release/rust-search
```

## Usage

### Index files
```bash
# Index entire filesystem
rust-search index /

# Index home directory  
rust-search index ~

# Custom database location
rust-search index /home --db ./custom.db
```

### Search queries
```bash
# Simple keyword search
rust-search search "setup.py"

# Filename filtering
rust-search search "filename:*.py"

# Code search
rust-search search "function async"

# View database statistics
rust-search stats

# Optimize database
rust-search vacuum
```

## 🛠️ Roadmap
1. **✅** Core indexing engine
2. **⏳** Snowball stemmer (Russian/English)
3. **⏳** Advanced BM25 ranking
4. **⏳** Tauri GUI desktop application
5. **⏳** Incremental index updates
6. **⏳** Fuzzy search matching

## 💼 Licensing

**Dual-licensed for maximum flexibility:**

### **GPL-3.0 (Open Source)**
Free for open-source, non-commercial, and academic projects.

**Key terms:**
- ✅ Use and modify freely
- ✅ Distribute source code
- ⚠️ Copyleft: Derivative works must use GPL-3.0

**[LICENSE.GPL-3.0](./LICENSE.GPL-3.0)**

### 💰 **Commercial License**
For companies and proprietary products.

**Key terms:**
- ✅ Closed-source permitted
- ✅ No GPL restrictions
- ✅ Priority support
- ✅ Custom features available

**Pricing:** $99/year OR $699/perpetual license

**[Commercial License](./LICENSE.COMMERCIAL.md)** | **ppasa5684@gmail.com**

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

```bash
1. Fork the repository
2. Create your feature branch (git checkout -b feature/amazing-feature)
3. Commit your changes (git commit -m 'Add amazing feature')
4. Push to the branch (git push origin feature/amazing-feature)
5. Open a Pull Request
```

All contributions must comply with GPL-3.0 licensing.

## 📬 Contact & Support
- **Commercial inquiries:** ppasa5684@gmail.com
- **GitHub:** [psqnn/rust-search](https://github.com/psqnn/rust-search)
- **Issues:** [GitHub Issues](https://github.com/psqnn/rust-search/issues)

## 📄 License
Dual-licensed under [GPL-3.0](./LICENSE.GPL-3.0) and [Commercial License](./LICENSE.COMMERCIAL.md)

---

**⭐ Star on GitHub if you find this useful!**

**rust-search: The modern alternative to everything.sh** 🔥
