# World Transmuter

## Building

Have the Rust toolchain installed:

```sh
# with cargo
cargo run
```

Full build with nix:

```sh
# flakes and submodules >_>
# binary in result/bin/world-transmuter-cli
nix build '.?submodules=1'# 

# build and run binary
nix run '.?submodules=1'#
```
