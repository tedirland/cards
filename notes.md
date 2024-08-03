# Rust Learning Notes

## Traits in Rust

- A trait in Rust is a set of functions that types can implement. It's more similar to an interface in other languages rather than inheritance.
- Traits can enforce that a type implementing a specific trait must also implement another trait using trait bounds. For example, `trait MyTrait: Debug {}` ensures that any type implementing `MyTrait` must also implement `Debug`.

## Arrays vs. Vectors

- Use arrays when you know the size of the list won't change, as they offer slight performance advantages due to their fixed size.
- Arrays signal to other developers that the list size is static and won't change during runtime.
- Vectors in Rust can also be created using `Vec::new()` or `vec![]`—this is common in Rust code.
- Vectors are more beneficial when the data structure is designed to grow or shrink over time, even if the initial size is known.

## Debug Formatter

- `:?` = Debug formatter, used to print a data structure in a human-readable format.
- Any type that is used with the `Debug` formatter must implement the `Debug` trait.
- If you attempt to print a custom struct using `println!("{:?}", my_struct);` but receive a compilation error, it's likely because the `Debug` trait has not been derived. You can resolve this by adding `#[derive(Debug)]` at the top of the struct definition.
- Adding a pound `:#?` will add more readable formatting

## Bindings and Immutability

- In Rust, variables are called bindings and they are immutable by default.
- This means the `value` can't be changed nor can the binding be `reassigned`
- They must be explicitly marked with the `mut` keyword for the value to be mutable.
- Immutability by default is a key safety feature in Rust, particularly in concurrent programming. It ensures that data cannot be modified unexpectedly, reducing the chances of race conditions.
- Rust’s ownership and borrowing system ensures safe access to data across threads, and when mutability is needed, synchronization primitives like `Mutex` or `RwLock` are used to enforce exclusive access.

## Extending Vectors with Traits

- You can extend the functionality of vectors by implementing custom traits. For example, you could create a trait to calculate the average of a vector's elements:

  ```rust
  trait Average {
      fn average(&self) -> f64;
  }

  impl Average for Vec<i32> {
      fn average(&self) -> f64 {
          let sum: i32 = self.iter().sum();
          let count = self.len();
          sum as f64 / count as f64
      }
  }
  ```
