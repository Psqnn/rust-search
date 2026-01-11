# rust-search

sled BwTree full-text search engine. Open source alternative to everything.sh.

## Features
- 100MB RAM indexes 1TB disk  
- BM25 ranking (stemmer ready)
- Tokio async indexing
- results.db (single file)

## Quickstart
```bash
cargo install rust-search
rust-search index /home
rust-search search "filename:setup.py"
```

## Architecture
```
results.db (sled BwTree)
├── files: path → FileInfo {size, hash, mtime}
├── terms: word → [file_id]
└── index: file_id → inverted index
```

## Benchmarks (planned)
```
Query: 10ms (1TB → top 100)
Index: 500GB/hour (SSD)
RAM: 100MB fixed
```

## Build
```bash
git clone https://github.com/YOUR/rust-search
cd rust-search
cargo build --release
```

## Roadmap
1. FileInfo struct (models.rs)
2. Snowball stemmer (Russian/English)
3. BM25 ranker
4. CLI search
5. GUI integrations

## License
MIT License

Copyright (c) 2026 Psqnn

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
