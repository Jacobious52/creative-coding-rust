# Creative Coding in Rust

This is just a collection of creative coding bits and pieces for learning Rust and being creative in a **cough** ~~nicer programming language than C++, Java, JS~~. Using the neat [nannou](https://nannou.cc) framework as it's reminiscent of [OpenFrameworks](https://openframeworks.cc) and [P5](https://p5js.org). These are in no way good code or good rust standards.. just playing around :)

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

Mouse pos in window determines "travel" speed forward

### Water

Simplex Noise to make random waves. (Not anything like water really).

### Falling Pixels

Per pixel (basic) falling sand and water simulation. 

Left mouse for water, right mouse for sand. Hold control for dirty pixel cache debug view (used to keep performance reasonable (~167fps 2020 RTX S / Ryzen 5 3600) with high numbver of pixels).
