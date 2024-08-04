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

### Deal Method

- Takes in a mutable reference to self and a usize representing the number of cards you'd like to deal
- Will need to look into the standard library docs '[https://doc.rust-lang.org/stable/std/]'
- We'll use the split_off vector method '[https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.split_off]'
- Implementation:

```rust
fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
```
