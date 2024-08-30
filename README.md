# rust-auto-public-ip-update

## Build and test

```
cargo build
cargo test
```

## Run

Create a `.env` file with the following secrets:
```env
CLOUDFLARE_TOKEN=
CLOUDFLARE_ZONE_ID=
```

```bash
# display help
cargo run -- --help

# get the current ip
cargo run -- current

# monitor changes and update cloudflare dns record
cargo run -- monitor
```
