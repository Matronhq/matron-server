# Matron Server

Matron Server is Matron's Matrix homeserver distribution, tracking
[Tuwunel](https://github.com/matrix-construct/tuwunel) upstream.

This fork intentionally keeps Tuwunel's native binary, crate, and configuration
prefixes for now. Use `TUWUNEL_` environment variables and Tuwunel's upstream
configuration docs unless a Matron-specific deployment wrapper says otherwise.

## Part of the Matron ecosystem

| Project | Description |
|---------|-------------|
| [Matron Desktop](https://github.com/matronhq/matron-desktop) | Desktop client |
| [Matron Web](https://github.com/matronhq/matron-web) | Web client |
| [Matron iOS](https://github.com/matronhq/matron-ios) | iOS client |
| **Matron Server** | Matrix homeserver (this repo) |
| [Dev Boxer](https://github.com/matronhq/dev-boxer) | One-command dev environment setup |

## Upstream

This repository is kept close to upstream Tuwunel. The codebase, binary name,
database path, and environment variable prefix still use upstream `tuwunel` and
`TUWUNEL_` names so the fork can be updated with minimal conflicts.

Matron-specific changes should stay small and documented. If Matron later needs
its own binary names or config prefixes, add that as a compatibility layer rather
than a repo-wide source rename.

## Container Image

The `ghcr.io/matronhq/matron-server:latest` image is mirrored from
`ghcr.io/matrix-construct/tuwunel:latest`.

Use Tuwunel's native environment variables when configuring the container:

```bash
docker run -d \
  --name matron-server \
  -p 6167:6167 \
  -v matron_server_data:/var/lib/tuwunel \
  -e TUWUNEL_SERVER_NAME="matrix.example.com" \
  -e TUWUNEL_DATABASE_PATH="/var/lib/tuwunel" \
  -e TUWUNEL_ALLOW_REGISTRATION="false" \
  ghcr.io/matronhq/matron-server:latest
```

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
`TUWUNEL_` prefix. For compatibility with upstream history, `CONDUWUIT_` and
`CONDUIT_` prefixes are still accepted by the server.

## Development

This repository tracks Tuwunel upstream first. Keep Matron-specific changes
focused on public metadata, documentation, packaging, and release automation.

Useful local checks:

```bash
git diff --check
cargo metadata --no-deps --locked --format-version 1
```

## License

Apache-2.0. See [LICENSE](LICENSE).
