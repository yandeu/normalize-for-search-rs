# Normalize for Search (Rust)

[![dependency status](https://deps.rs/repo/github/yandeu/normalize-for-search-rs/status.svg)](https://deps.rs/repo/github/yandeu/normalize-for-search-rs)
[![CI](https://github.com/yandeu/normalize-for-search-rs/actions/workflows/main.yml/badge.svg)](https://github.com/yandeu/normalize-for-search-rs/actions/workflows/main.yml)

## Install

```toml
[dependencies]
normalize-for-search = { git = "https://github.com/yandeu/normalize-for-search-rs" }
```

## Example

```rust
use normalize_for_search::normalize_for_search as nfs;
use regex::Regex;

fn main() {
    assert_eq!(nfs("Jérôme Müller "), "jerome muller");
}
```
