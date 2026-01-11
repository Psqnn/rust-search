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


