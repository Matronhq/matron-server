# Matron Server

A fast and reliable Matrix homeserver written in Rust.

Forked from [Tuwunel](https://github.com/matrix-construct/tuwunel) (which continues from conduwuit/Conduit). Rebranded and cleaned up for professional use.

## Part of the Matron ecosystem

| Project | Description |
|---------|-------------|
| [Matron Desktop](https://github.com/matronhq/matron-desktop) | Desktop client |
| [Matron Web](https://github.com/matronhq/matron-web) | Web client |
| [Matron iOS](https://github.com/matronhq/matron-ios) | iOS client |
| **Matron Server** | Matrix homeserver (this repo) |
| [Dev Boxer](https://github.com/matronhq/dev-boxer) | One-command dev environment setup |

## Features

- Single binary, easy to deploy
- Low resource usage (runs well on small VPSes)
- RocksDB storage backend
- End-to-end encryption support
- No federation by default (private instances)
- Compatible with all Matrix clients

## Quick start with Docker

```bash
docker run -d \
  --name matron-server \
  -p 6167:6167 \
  -v matron_data:/var/lib/matron-server \
  -e MATRON_SERVER_SERVER_NAME="your.domain.com" \
  -e MATRON_SERVER_ALLOW_REGISTRATION="true" \
  ghcr.io/matronhq/matron-server:latest
```

Or use [Dev Boxer](https://github.com/matronhq/dev-boxer) for a fully automated setup including the Matrix bridge and clients.

## Configuration

Copy the example config and edit:

```bash
cp matron-server-example.toml matron-server.toml
```

At minimum, set `server_name` and `database_path`:

```toml
[global]
server_name = "your.domain.com"
database_path = "/var/lib/matron-server"
```

See `matron-server-example.toml` for all options, or the full [documentation](https://matronhq.github.io/matron-server/).

## Building from source

```bash
cargo build --release
```

The binary will be at `target/release/matron-server`.

## Environment variables

Configuration can also be set via environment variables with the `MATRON_SERVER_` prefix:

```bash
MATRON_SERVER_SERVER_NAME="your.domain.com"
MATRON_SERVER_PORT=6167
MATRON_SERVER_ALLOW_REGISTRATION=false
```

For backwards compatibility, `TUWUNEL_`, `CONDUWUIT_`, and `CONDUIT_` prefixes are also accepted.

## Migration

- **From Tuwunel/conduwuit:** Drop-in replacement. Point at the same database directory.
- **From Synapse/Conduit:** Not yet supported.

## License

Apache-2.0. See [LICENSE](LICENSE).
