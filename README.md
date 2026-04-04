# Matron Server

![GitHub License](https://img.shields.io/github/license/matronhq/matron-server?style=flat%2Dsquare&color=%238A2BE2)
![GitHub Commit Activity](https://img.shields.io/github/commit-activity/m/matronhq/matron-server?style=flat%2Dsquare&link=https%3A%2F%2Fgithub.com%2Fmatronhq%2Fmatron-server%2Fpulse%2Fmonthly&color=%238A2BE2)
![Docker Pulls](https://img.shields.io/docker/pulls/matronhq/matron-server?style=flat%2Dsquare&color=8A2BE2)
![GitHub Repo Stars](https://img.shields.io/github/stars/matronhq/matron-server?style=flat%2Dsquare&link=https%3A%2F%2Fgithub.com%2Fmatronhq%2Fmatron-server&color=%238A2BE2)

<!-- ANCHOR: catchphrase -->

## A fast and reliable Matrix homeserver, forked from Tuwunel.

<!-- ANCHOR_END: catchphrase -->

<!-- ANCHOR: body -->

[![Documentation](https://img.shields.io/badge/documentation%2D_?color=%238A2BE2&style=for-the-badge&logo=mdBook&logoColor=FFFFFF)](https://matron.chat)
[![Support Chat](https://img.shields.io/matrix/matron%3Amatron.chat.svg?color=098A09&style=for-the-badge&label=Support%20Chat&labelColor=8A2BE2&logo=Matrix)](https://matrix.to/#/#matron:matron.chat)

Matron Server is a featureful [Matrix](https://matrix.org/) homeserver you can use instead of Synapse
with your favorite [client](https://matrix.org/ecosystem/clients/),
[bridge](https://matrix.org/ecosystem/bridges/) or
[bot](https://matrix.org/ecosystem/integrations/). It is written entirely in Rust to be a scalable,
low-cost, enterprise-ready, community-driven alternative, fully implementing the
[Matrix Specification](https://spec.matrix.org/latest/) for all but the most niche uses.

This project is forked from [Tuwunel](https://github.com/matrix-construct/tuwunel), which is the
successor to [conduwuit](https://github.com/x86pup/conduwuit).

### Getting Started

- [GitHub Releases](https://github.com/matronhq/matron-server/releases)
- [Sourcecode](https://github.com/matronhq/matron-server/) `git clone https://github.com/matronhq/matron-server.git`
- [DockerHub](https://hub.docker.com/r/matronhq/matron-server) or `docker pull matronhq/matron-server:latest`
- [GHCR](https://github.com/matronhq/matron-server/pkgs/container/matron-server) or `docker pull ghcr.io/matronhq/matron-server:latest`
- Static binaries available as [releases](https://github.com/matronhq/matron-server/releases) or [build artifacts](https://github.com/matronhq/matron-server/actions?query=branch%3Amain).
- Deb and RPM packages available as [releases](https://github.com/matronhq/matron-server/releases) or [build artifacts](https://github.com/matronhq/matron-server/actions?query=branch%3Amain).

**1.** [Configure](https://matron.chat/configuration.html) by
copying and editing the `matron-server-example.toml`. The `server_name` and `database_path` must be
configured. **Most users deploy via docker or a distribution package and should follow the
[appropriate guide](https://matron.chat/deploying.html) instead.**
This is just a summary for the impatient. See the full
[documentation](https://matron.chat/).

> [!TIP]
> Avoid using a sub-domain for your `server_name`. You can always delegate later with a [`.well-known`](https://matron.chat/deploying/root-domain-delegation.html)
> file, but you can never change your `server_name`.

**2.** Setup TLS certificates. Most users enjoy the [Caddy](https://caddyserver.com/) reverse-proxy
which automates their certificate renewal. Advanced users can load their own TLS certificates
using the configuration and Matron Server can be deployed without a reverse proxy. Example
`/etc/caddy/Caddyfile` configuration with [Element](https://github.com/element-hq/element-web/releases)
unzipped to `/var/www/element`:
```
example.com, example.com:8448 {
    reverse_proxy localhost:8008
}
web.example.com {
    root * /var/www/element/
    file_server
}
```
`caddy reload --config /etc/caddy/Caddyfile`

**3.** Start the server, connect your client and register your username. The first registration is
granted server admin.

> [!TIP]
> Configure a secret `registration_token` and set `allow_registration = true`


### Migrating to Matron Server

| Can I migrate from | |
|-----------------|-----------|
| Tuwunel? | Yes. This is a direct fork. |
| conduwuit? | Yes. This will be supported at a minimum for one year, but likely indefinitely. |
| Synapse? | Not yet. |
| Conduit? | Not right now. |
| Any other fork of Conduit? | No. The migration must be explicitly listed in this table. |
> [!CAUTION]
> **Never switch between different forks of Conduit or you will corrupt your database.**
> All derivatives of Conduit share the same linear database version without any awareness of other
> forks. The database will permanently corrupt and we will not be able to help you.

#### Migrating from Tuwunel or conduwuit

Migrating from Tuwunel or conduwuit to Matron Server _just works_. In technical parlance it is a "binary swap."
All you have to do is update to the latest Matron Server and change the path to the executable to
`matron-server`.

Environment variables with prefixes `TUWUNEL_`, `CONDUWUIT_`, and `CONDUIT_` are still recognized
for backwards compatibility. If you were a Tuwunel or conduwuit user we recommend against changing
anything at all initially. This will keep things simple.


### Upgrading & Downgrading Matron Server

We strive to make moving between versions of Matron Server safe and easy. Downgrading Matron Server is always
safe but often prevented by a guard. An error will indicate the downgrade is not possible and a
newer version which does not error must be sought.

#### Branches

The main branch is always _reasonably safe_ to run. We understand the propensity for users to simply clone
the main branch to get up and running, and we're obliged to ensure it's always viable. Nevertheless, only
tagged releases are true releases.

#### Container Tracking

> [!IMPORTANT]
> **We strongly advise tracking the `:latest` tag when automatically updating.**

Tracking `:latest` gives us the necessary discretion to keep you on the appropriate stable version.
We discourage tracking the main branch unless frequent restarts are acceptable. Alternatively,
tracking the `:preview` tag provides the latest release-candidate becoming equivalent to `:latest`
after a release.

### Getting Help & Support

For support, join the Matrix room at [#matron:matron.chat](https://matrix.to/#/#matron:matron.chat).

For security disclosures or private discussion, please open a GitHub issue or contact the maintainers directly.

<!-- ANCHOR_END: body -->

<!-- ANCHOR: footer -->

<!-- ANCHOR_END: footer -->
