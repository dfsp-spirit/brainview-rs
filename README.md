# brainview

A simple viewer for surface-based structural neuroimaging data, written in [Rust](https://www.rust-lang.org/).

The `brainview` crate is based on [neuroformats](https://github.com/dfsp-spirit/neuroformats) and [three-d](https://github.com/asny/three-d) by Asger Nyman Christiansen.

# Usage

This is WIP, come back another day. 

If you insist and have a recent Rust toolchain installed, you can build the current prototype from source and run it like this:

```
git clone https://github.com/dfsp-spirit/brainview-rs && cd brainview-rs/
cargo run
```

This will open a 3D window and show a rotating brain mesh, with vertex colors representing cortical thickness. The current version of this crate is basically a slighty modified version of the [three-d triangle example](https://github.com/asny/three-d/tree/0.6.0/examples/triangle).

![Vis](./resources/web/brainview-rs.jpg?raw=true "Brain visualizationin Rust.")



