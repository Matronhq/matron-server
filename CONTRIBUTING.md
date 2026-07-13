# Contributing to Matron Server

Thanks for your interest in contributing!

## Where to contribute

Contributions are handled through GitHub on
[Matronhq/matron-server](https://github.com/Matronhq/matron-server):

- **Bugs and feature requests:** open an
  [issue](https://github.com/Matronhq/matron-server/issues).
- **Changes:** open a pull request against the `main` branch. Please mark
  work-in-progress PRs as drafts.

## Upstream first

Most of the server code in this repository comes from upstream
[Tuwunel](https://github.com/matrix-construct/tuwunel), and this fork is kept
intentionally close to it. Substantive changes to server behavior are usually
better contributed upstream, where they benefit everyone and flow back into
this fork on the next sync. Matron-specific changes here should stay focused
on public metadata, documentation, packaging, and release automation.

## Code standards

Changes must pass the lints (clippy, rustc, rustdoc) and be formatted with the
**nightly** `cargo fmt`, matching upstream Tuwunel's conventions.

## Licensing

By sending a pull request or patch, you agree that your changes may be
licensed under the Apache-2.0 license, and that your conduct is in line with
the [Code of Conduct](CODE_OF_CONDUCT.md).
