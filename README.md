# brainview

A high-level library and a simple viewer (binary) for surface-based structural neuroimaging data, written in [Rust](https://www.rust-lang.org/).

The `brainview` crate is based on [neuroformats](https://github.com/dfsp-spirit/neuroformats) and [three-d](https://github.com/asny/three-d) by Asger Nyman Christiansen. It can visualize brain surface meshes and related per-vertex data computed from magnetic resonance images (MRI) in [FreeSurfer](http://freesurfer.net/), [CAT12](http://www.neuro.uni-jena.de/cat/) and other neuroimaging software packages which can output or convert to a supported file format.

## Why brainview?

The goal of the `brainview` crate is to provide a very high-level wrapper around `three-d` that allows neuroscientists using Rust for scientific data analyses to look at their data and statisical results directly in Rust -- without having to export their data first and then load them again in a separate viewer application, which is time-consuming and error-prone when looking at many results.

Ideally, you should be able to load and visualize your neuroimaging data with two function calls.


## Usage

This is WIP, come back another day.

Have a look at [src/main.rs](./src/main.rs) to see the client code used to create the prototype below. 


## Running the current prototype

If you insist and have a recent [Rust toolchain installed](https://www.rust-lang.org/tools/install), you can build the current prototype from source and run it like this:

```
git clone https://github.com/dfsp-spirit/brainview-rs
cd brainview-rs/
cargo build --release

cargo run
```

This will open a 3D window and show a rotating brain mesh, with vertex colors representing cortical thickness:

![Vis](./resources/web/brainview-rs.jpg?raw=true "Brain visualizationin Rust.")


