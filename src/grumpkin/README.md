Grumpkin curve implementation in Rust
-----

Adapted from https://github.com/jules/grumpkin

This repository implements the Grumpkin curve for use in Rust, by building off of the code provided by ZCash and Privacy & Scaling Explorations. It should specifically work within Halo2 and proof systems which make use of its components.
Assembly

This repository extends the use of hand-optimized assembly found in the halo2curves repo. To include it, simply use the asm feature when importing this crate.