# rust-auto-public-ip-update

## Build

```
cargo build
```

## Run

Create a `.env` file with the following secrets:
```env
CLOUDFLARE_TOKEN=
DOMAIN=
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
