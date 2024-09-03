# rust-cloudflare-dynamic-public-ip

Update public ip in cloudflare's DNS records automatically. You never know when your ISP is pushing updates to your router and cycle your public IP, breaking DNS records, this is my solution.

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
