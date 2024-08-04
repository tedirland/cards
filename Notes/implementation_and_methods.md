## Inherent Implementations

- Add a function to a struct
- Used to define `methods` and `associated functions`
- `Associated functions` are like Class Methods in other languages
- `Methods` operate on a specific instance of a struct
- You define a method by giving it a first argument of `&self`

### When to use

- `Associated Functions` should be used when you have functionality not tied to a specific instance
  -Examples: create a full deck, deck with n cards, empty deck

- `Methods` should be used when you need to read of change fields on a specific instance

  - Examples: shuffling cards, adding a card, removing a card, checking if a card exists

- Example Impl block for a deck of cards:

```rust
impl Deck{
    fn new() -> Self {
        //stuff...
    }
    fn shuffle(&self) {
        //stuff...
    }
}
```

### Returning

```rust
// Can be done by using return keyword
return Deck;

// Or by reoving return keyword and semicolon. Called implicit return and is used far more frequently in Rust
Deck
```

- Either is equivalent
