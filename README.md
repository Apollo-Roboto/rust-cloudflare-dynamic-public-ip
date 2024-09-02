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

# see the affected DNS records
cargo run -- info

# monitor changes and update cloudflare DNS record
cargo run -- monitor
```
