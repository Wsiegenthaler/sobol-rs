# sobol

![Crates.io](https://img.shields.io/crates/v/sobol)
[![documentation](https://docs.rs/sobol/badge.svg)](https://docs.rs/sobol)
![minimum rustc 1.33](https://img.shields.io/badge/rustc-1.33+-red.svg)
![Rust](https://github.com/Wsiegenthaler/sobol-rs/workflows/Rust/badge.svg)
[![Coverage Status](https://coveralls.io/repos/github/Wsiegenthaler/sobol-rs/badge.svg?branch=master)](https://coveralls.io/github/Wsiegenthaler/sobol-rs?branch=master)
[![License](https://img.shields.io/badge/License-BSD%203--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)

*A Sobol sequence generator for Rust*

This crate provides Sobol low-discrepancy quasirandom sequences which are useful for integration and other tasks. The sequence can cover a domain evenly with as little as a few points and will continue to progressively fill the space as more points are added.

For efficiency, *sobol* employs the recursive variant of the gray code optimization proposed by *Antonov-Saleev*.

Below are examples of 2-dimensional points drawn from Sobol and Uniform generators respectively. See an animated visualization [here](http://wsiegenthaler.github.io/lobos/web-example.html).
<p align="center">
  <img src="http://wsiegenthaler.github.io/lobos/sobol.png" alt="Sobol" width="49%">
  <img src="http://wsiegenthaler.github.io/lobos/uniform.png" alt="Uniform" width="49%">
</p>

## Usage

```shell
cargo install sobol
```

Print the first 100 points from a 3-dimensional sequence:

```rust
extern crate sobol;
  
use sobol::Sobol;
use sobol::params::JoeKuoD6;

fn main() {
    let params = JoeKuoD6::minimal();
    let seq = Sobol::<f32>::new(3, &params);
    
    for point in seq.take(100) {
        println!("{:?}", point);
    }
}
```

In this example each component of the sequence is a 32-bit float but *sobol* also supports Rust's other numeric primitives. Floating point sequences span the unit hypercube (i.e. `[0,1)`) while integer valued sequences span the natural domain of the selected type. For example, `u16` typed sequences will have components between 0 and 65,536.

## Initialization Values

Initialization values (aka "parameters") supporting up to 21,201 dimensions are provided courtesy of Stephen Joe and Frances Kuo ([source](http://web.maths.unsw.edu.au/~fkuo/sobol)) and are accessible via `sobol::params::JoeKuoD6`. Custom initialization values can be used by implementing the `sobol::SobolParams` trait.

If imported into your project, the provided `JoeKuoD6` parameters are automatically embedded into your project binary. To reduce the amount of data added to your project, `JoeKuoD6` provides three otherwise identical parameter sets which can be selected from according to the dimensionality required by your sequences:

| Source | Supported Dims | Approx. Size |
| ------ | -------------- | ------------ |
| `JoeKuoD6::minimal()` | 100  | 1kb |
| `JoeKuoD6::standard()` | 1,000 | 20kb |
| `JoeKuoD6::extended()` | 21,201  | 690kb |

## See also

* [lobos](https://github.com/wsiegenthaler/lobos) - A Sobol sequence generator for Scala and Javascript

## References

* Joe, Stephen, and Frances Y. Kuo. ["Notes on Generating Sobol Sequences."](http://web.maths.unsw.edu.au/~fkuo/sobol/joe-kuo-notes.pdf) (n.d.): n. pag. Aug. 2008. Web.

* "[Sobol Sequence.](http://en.wikipedia.org/wiki/Sobol_sequence)" Wikipedia. Wikimedia Foundation, n.d. Web. 25 Feb. 2015.

## License

Everything in this repo is BSD License unless otherwise specified

sobol-rs (c) 2020 Weston Siegenthaler
