# Create Coding in Rust

This is just a collection of creative coding bits and pieces for learning Rust. Using the neat [nannou](https://nannou.cc) framework as it's reminiscent of [OpenFrameworks](https://openframeworks.cc) and [P5](https://p5js.org).

## Each cargo workspace is a separate showcase / example

### Build all

Recommend using `--release` builds ;)

```bash
cargo build --release
```

### Run an example

```bash
EXAMPLE=starfield // set this to an example of your choice
cargo run -p $EXAMPLE --release
```

### Starfield

Starfield from The Coding Train's challenge with a bit of fading in for the stars

### Water

Simplex Noise to make random waves. (Not anything like water really).
