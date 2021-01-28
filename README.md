# csgo-config-transfer

Small utility to transfer all configs from one steam account (your main) to your smurfs.

## Setup

First, Create a ``.env`` file with the following values (obviously change them)

```toml
STEAM_MAIN_ACCOUNT="91707110"
STEAM_INSTALL_LOC="C:\\Program Files (x86)\\Steam\\userdata"
```

Then run - either download a pre-compiled version and run or just ``cargo run --release`` and that's it!
