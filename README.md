# brainview

A simple viewer for surface-based structural neuroimaging data, written in [Rust](https://www.rust-lang.org/).

The `brainview` crate is based on `neuroformats` and `three-d`.

# Usage

This is WIP, come back another day. 

If you insist and have a recent Rust toolchain installed, you can build the current prototype from source and run it like this:

```
git clone https://github.com/dfsp-spirit/brainview-rs
cd brainview-rs

cargo build --release
cargo run
```

This will open a 3D window and show a rotating brain mesh, with vertex colors representing cortical thickness.



