# Laskea

[![Continuous Integration](https://github.com/Michael-F-Bryan/laskea/actions/workflows/main.yml/badge.svg)](https://github.com/Michael-F-Bryan/laskea/actions/workflows/main.yml)

[(Online Demo)](https://michael-f-bryan.github.io/laskea)

An incremental computation engine powered by [Salsa][salsa].

## Etymology

As you've probably guessed, *Laskea* isn't a word the typical English speaker
will see around the place. I've been hanging out with some Finns lately,
so I thought it might be cute to ~~bastardize~~ reuse one of their words.

*Laskea* means *evaluate* or *compute* in Finnish.

## Architecture

This project consists of 3 parts,

- `engine/` - the actual evaluation engine
- `bindings/` - glue for making the evaluation engine available to JavaScript
- `frontend/` - the React UI

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE.md) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT.md) or
   http://opensource.org/licenses/MIT)

at your option.

It is recommended to always use [cargo-crev][crev] to verify the
trustworthiness of each of your dependencies, including this one.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

This crate intends to be free of soundness bugs. The developers will
do their best to avoid them, and welcome help analyzing and fixing them.

[API Docs]: https://michael-f-bryan.github.io/{{project-name}}
[crev]: https://github.com/crev-dev/cargo-crev
[salsa]: https://crates.io/crates/salsa
