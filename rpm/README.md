# Matron Server for Red Hat

Information about downloading and deploying the Red Hat package. This may also be
referenced for other `rpm`-based distros such as CentOS.

### Installation

It is recommended to see the [generic deployment guide](../deploying/generic.md)
for further information if needed as usage of the RPM package is generally
related.

No `rpm` repository is currently offered yet, it is in the works/development.

### Configuration

When installed, the example config is placed at `/etc/matron-server/matron-server.toml`
as the default config. The config mentions things required to be changed before
starting.

You can tweak more detailed settings by uncommenting and setting the config
options in `/etc/matron-server/matron-server.toml`.

### Running

The package uses the [`matron-server.service`](../configuration/examples.md#example-systemd-unit-file)
systemd unit file to start and stop Matron Server. The binary is installed at `/usr/sbin/matron-server`.

This package assumes by default that Matron Server will be placed behind a reverse
proxy. The default config options apply (listening on `localhost` and TCP port
`8008`). Matrix federation requires a valid domain name and TLS, so you will
need to set up TLS certificates and renewal for it to work properly if you
intend to federate.

Consult various online documentation and guides on setting up a reverse proxy
and TLS. Caddy is documented at the [generic deployment guide](../deploying/generic.md#setting-up-the-reverse-proxy)
as it's the easiest and most user friendly.
