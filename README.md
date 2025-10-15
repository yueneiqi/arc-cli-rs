# arc-cli-rs

`arc-cli-rs` is a Rust implementation of a command line interface for the [Arc Browser](https://arc.net/).

This project is not affiliated with Arc Browser or The Browser Company.

## Credits

This is a Rust port of the original [arc-cli](https://github.com/GeorgeSG/arc-cli) by [Georgi Gardev](https://github.com/GeorgeSG). The original TypeScript/Node.js implementation inspired by and reuses some code from the [Raycast Arc extension](https://www.raycast.com/the-browser-company/arc).

## Installation

### From Source

Required: Rust toolchain (cargo)

```bash
cargo build --release
```

The binary will be available at `target/release/arc-cli-rs`

## Usage

```
arc-cli-rs [command]

Commands:
  arc-cli-rs arc-version                      Show Arc version
  arc-cli-rs list-spaces                      List spaces             [aliases: ls]
  arc-cli-rs select-space <space-id>          Select a space          [aliases: s]
  arc-cli-rs select-space-name <space-name>   Select a space by name  [aliases: sn]
  arc-cli-rs list-tabs                        List tabs               [aliases: lt]
  arc-cli-rs new-tab <url>                    Open URL in a new tab   [aliases: nt]
  arc-cli-rs select-tab <window-id> <tab-id>  Select tab              [aliases: st]
  arc-cli-rs select-tab-name <tab-name>       Select tab by name      [aliases: stn]
  arc-cli-rs reload-tab <window-id> <tab-id>  Reload tab              [aliases: rt]
  arc-cli-rs close-tab <window-id> <tab-id>   Close tab               [aliases: ct]
  arc-cli-rs new-little-arc <url>             Open URL in a new Little Arc window
                                                                  [aliases: nla]

Options:
  --help     Show help
  --version  Show version number
```

### Select Tab by Name

The `select-tab-name` command supports flexible matching options:

```bash
# Default: Case-insensitive partial match
arc-cli-rs stn github

# Case-sensitive partial match
arc-cli-rs stn -c GitHub

# Case-insensitive exact match
arc-cli-rs stn -e "GitHub Dashboard"

# Case-sensitive exact match
arc-cli-rs stn -ce "GitHub Dashboard"
```

Options:
- `-c, --case-sensitive`: Enable case-sensitive matching
- `-e, --exact`: Enable exact match (no partial matching)

## License

MIT
