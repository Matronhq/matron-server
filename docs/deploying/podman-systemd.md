# matron-server in Podman systemd

Copy [matron-server.container](matron-server.container) to ~/.config/containers/systemd/matron-server.container.
Reload daemon:
```
systemctl --user daemon-reload
```
Start the service:
```
systemctl --user start matron-server
```

To check the logs, run:
```
journalctl -eu matron-server.container --user
```
