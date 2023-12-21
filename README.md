# nt

[![Cargo Build & Test](https://github.com/kojix2/nt/actions/workflows/ci.yml/badge.svg)](https://github.com/kojix2/nt/actions/workflows/ci.yml)

`nt` is a tool that runs your command and then sends a desktop notification.

## Installation

```sh
cargo install --git https://github.com/kojix2/nt
```

## Usage

```sh
nt your cmd here
```

Examples

```sh
nt sudo apt update -yq
```

```
nt wget https://large.file.com/largefile.tar.gz
```

This runs your command and then a desktop notification will pop up. The notification tells you if your command worked or didn't work.

## License

'nt' is under the MIT License.
