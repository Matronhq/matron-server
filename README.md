# Matron Server

> **Status: legacy.** Matron's current stack uses
> [matron-journal](https://github.com/Matronhq/matron-journal) as its sync
> server; the Matrix transport this homeserver served has been retired
> ([matron-bridge](https://github.com/Matronhq/matron-bridge) is journal-only,
> and [dev-boxer](https://github.com/Matronhq/dev-boxer) no longer installs a
> homeserver — see its "Upgrading from a Matrix-era install" section for
> migration). This repo is kept as a mirror of upstream Tuwunel for existing
> Matrix-era deployments.

<!-- ANCHOR: catchphrase -->

Matron Server is a fork of
[Tuwunel](https://github.com/matrix-construct/tuwunel), a Matrix homeserver
written in Rust.

<!-- ANCHOR_END: catchphrase -->

<!-- ANCHOR: body -->

It was the homeserver behind [Matron](https://matron.chat)'s Matrix transport:
clients connected to it over the Matrix protocol, and
[matron-bridge](https://github.com/Matronhq/matron-bridge) relayed messages
between Matrix rooms and Claude Code sessions. That transport has been
retired; this repository stays close to upstream so existing Matrix-era
deployments can keep pulling a current server image.

## Versions

This tree tracks Tuwunel **1.6.0**; upstream is at 1.8.x (latest release
v1.8.3). The `ghcr.io/matronhq/matron-server:latest` image is a weekly re-tag
of `ghcr.io/matrix-construct/tuwunel:latest` (see
[`.github/workflows/sync-image.yml`](.github/workflows/sync-image.yml)) and
does **not** correspond to the source in this repo.

## Naming

The fork keeps Tuwunel's native names throughout: the crate and binary are
`tuwunel`, the default database path uses `/var/lib/tuwunel`, and
configuration uses `TUWUNEL_` environment variables (`CONDUWUIT_` and
`CONDUIT_` prefixes are also accepted for compatibility with upstream
history). Follow Tuwunel's upstream configuration docs unless a
Matron-specific deployment wrapper says otherwise.

## Part of the Matron ecosystem

| Project | Description |
|---------|-------------|
| [Matron Desktop](https://github.com/Matronhq/matron-desktop) | Desktop client |
| [Matron Web](https://github.com/Matronhq/matron-web) | Web client |
| [Matron Apple](https://github.com/Matronhq/matron-apple) | iPhone and Mac client |
| [Matron Android](https://github.com/Matronhq/matron-android) | Android client |
| **Matron Server** | Matrix homeserver (this repo — legacy) |
| [Matron Journal](https://github.com/Matronhq/matron-journal) | Sync server — the current Matron backbone |
| [matron-bridge](https://github.com/Matronhq/matron-bridge) | Ran Claude Code sessions and bridged them to Matrix (now journal-only) |
| [Dev Boxer](https://github.com/Matronhq/dev-boxer) | One-command dev environment setup |

## Container image

The `ghcr.io/matronhq/matron-server:latest` image is mirrored weekly from
`ghcr.io/matrix-construct/tuwunel:latest` — it is upstream's current build,
not a build of this tree (see [Versions](#versions)).

Use Tuwunel's native environment variables when configuring the container.
The server listens on port 8008 by default; this example moves it to 6167 to
match [`docs/deploying/docker-compose.yml`](docs/deploying/docker-compose.yml):

```bash
docker run -d \
  --name matron-server \
  -p 6167:6167 \
  -v matron_server_data:/var/lib/tuwunel \
  -e TUWUNEL_SERVER_NAME="matrix.example.com" \
  -e TUWUNEL_PORT="6167" \
  -e TUWUNEL_DATABASE_PATH="/var/lib/tuwunel" \
  -e TUWUNEL_ALLOW_REGISTRATION="false" \
  ghcr.io/matronhq/matron-server:latest
```

## Build from source

The crate and binary are named `tuwunel` (see [Naming](#naming)):

```bash
cargo build --release
```

The resulting binary is at `target/release/tuwunel`. For toolchain details,
feature flags, and deeper build documentation, see the upstream
[Tuwunel docs](https://matrix-construct.github.io/tuwunel/).

## Configuration

Most operators should follow the upstream
[Tuwunel configuration docs](https://matrix-construct.github.io/tuwunel/configuration.html).
At minimum, set `server_name` and `database_path`:

```toml
[global]
server_name = "matrix.example.com"
database_path = "/var/lib/tuwunel"
```

Configuration can also be supplied through environment variables with the
`TUWUNEL_` prefix.

## Development

CI in this repository runs metadata and hygiene checks only
(`cargo metadata`, whitespace, workflow YAML validation, string scans) —
it does not build or test the server source. For project layout and build documentation,
see [development.md](development.md) and the mdBook under
[`docs/`](docs/). Substantive changes to server behavior belong upstream —
see [CONTRIBUTING.md](CONTRIBUTING.md).

## Links

- [CONTRIBUTING.md](CONTRIBUTING.md) — how and where to contribute
- [RELEASE.md](RELEASE.md) — Tuwunel 1.6.0 release notes (the version this tree tracks)
- [`docs/`](docs/) — mdBook documentation source
- [Tuwunel](https://github.com/matrix-construct/tuwunel) — upstream project

<!-- ANCHOR_END: body -->

<!-- ANCHOR: footer -->

## License

Apache-2.0. See [LICENSE](LICENSE).

<!-- ANCHOR_END: footer -->
