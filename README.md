
![Cargo Build Badge](https://github.com/dupeiran001/rust-ray-tracing-in-one-weekend/actions/workflows/cargo_test_build.yaml/badge.svg)
![GitHub Page Deploy Badge](https://github.com/dupeiran001/rust-ray-tracing-in-one-weekend/actions/workflows/deploy_mdbook.yml/badge.svg)

# Introduce

[Ray-tracing in one weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) is a great introduction
to the computer graphics. It's originally based on c++, but I'm a big fan of rust. So I'd like to use rust to implement
the code in that book. We should be able to finish it in one weekend, having a tracer and producing some great images.

Rust is a bit different with c++, as it has a more strict static compiler. We'll need more effort to **fight with compiler**!
My rust code maybe is not the most elegant solution, so **any contribution is highly welcomed**! You are required to have some
basic knowledge of Rust, which can be learned from [The Book](https://doc.rust-lang.org/book/)

# Usage

This is a cargo project, and the **./book** directory is a mdbook project. The online preview of this book can be
found here: [(Rust) ray-tracing in one weekend](https://dupeiran001.github.io/rust-ray-tracing-in-one-weekend/), or
you can clone this repository and build your own static copy:

```bash
git clone https://github.com/dupeiran001/rust-ray-tracing-in-one-weekend
cd book
mdbook serve --open
```

And the source code can be run using:

```bash
cargo run
```

which will generate the output to the stdout, and can be redirecting to a file using:

```bash
cargo build
./target/debug/ray_tracing > image.ppm
```
on linux or
```bash
cargo build
./target/debug/ray_tracing.exe > image.ppm
```

which will generate an output picture called image.ppm.
of course, any marked commit can be used to generate the output of the corresponding chapter.

# Contribution

Any Contribution is Highly Welcomed!! No matter it's just a space alignment error or a code playground mark error, every contribution will be carefully treated!
