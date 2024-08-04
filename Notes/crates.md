# Project Structure in Rust

- Organized into modules
- Every crate has a `root` module and may have some additional submodules
- Similar to namespaces
- Our own project can have submodules
- The process for using external crates is different that using submodules inside our own project

```rust

// external crate
let rng = rand::thread_rng()
let rng = rand::seq::SliceRandom;

// Can also create an 'alias' with the use keyword

use rand::thread_rng;

//internal module
mod games;

let deck = games::Deck::new();

```

## \*\*Crates

- Equivalent to packages in other languages
- `use` Pulls specific things into the scope of a file

### Standard Library

- The standard library is included with every project without any additional install

### External Crates

- Have to be installed via cargo add command or from within the Cargo.toml file
- Available crates are listed on '[(https://crates.io/)]'
- Docs can be found here '[(https://docs.rs/)]'

### Rand Crate

- rand is a very commonly used package that provides random number generation '[https://docs.rs/rand/0.8.5/rand/]'

```

```
