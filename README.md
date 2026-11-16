# kvdiff

Zero-dependency CLI & Rust library for comparing `.env` and key-value configuration files safely in CI pipelines.

## Features
- Ignores comments, blank lines, and `export ` prefixes
- Strips matching single/double quotes consistently
- Supports `--mask-values` to compare secret files without leaking values to logs
- Returns exit code `1` when drift is detected for CI gating

## Usage
```bash
cargo run -- --mask-values staging.env production.env
```

## License
MIT © na-op
