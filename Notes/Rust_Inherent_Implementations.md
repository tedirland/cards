
# Rust Inherent Implementations

## Inherent Implementations

Inherent implementations in Rust allow you to define methods and associated functions for a struct (or an enum). These implementations provide behavior that is tied directly to the type.

- **Inherent implementation**: The `impl` block is used to add methods and associated functions to a struct.
- **Methods**: Functions that operate on an instance of the struct. They have a `self`, `&self`, or `&mut self` parameter as their first argument.
- **Associated Functions**: Functions defined within the `impl` block that do not take `self` as a parameter. They are similar to static methods in other languages.

### Methods vs. Associated Functions

- **Methods**:
  - Operate on a specific instance of a struct.
  - They are defined with the first argument being `&self`, `&mut self`, or `self`.
  - Examples:
    - Shuffling a deck of cards (`&mut self`).
    - Adding or removing a card from a deck (`&mut self`).
    - Checking if a card exists in the deck (`&self`).

- **Associated Functions**:
  - Do not operate on a specific instance; instead, they are typically used for constructors or utility functions.
  - They do not have a `self` parameter.
  - Examples:
    - Creating a new deck of cards (`new` function).
    - Creating a deck with a specific number of cards.
    - Creating an empty deck.

### Example `impl` Block for a Deck of Cards

Here’s an example of how you might implement a `Deck` struct:

```rust
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    // Associated function: constructor for a new deck
    fn new() -> Self {
        Deck {
            cards: Vec::new(), // or populate with cards
        }
    }

    // Method: shuffle the deck
    fn shuffle(&mut self) {
        // Implementation for shuffling the deck
    }

    // Method: add a card to the deck
    fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    // Method: remove a card from the deck
    fn remove_card(&mut self) -> Option<Card> {
        self.cards.pop() // Removes the last card
    }

    // Method: check if a card exists in the deck
    fn contains_card(&self, card: &Card) -> bool {
        self.cards.contains(card)
    }
}
```

### Returning Values

In Rust, functions can return values in two ways:

- **Explicit Return**: Using the `return` keyword.
  ```rust
  fn example() -> Deck {
      return Deck::new();
  }
  ```

- **Implicit Return**: By omitting the `return` keyword and semicolon. This is idiomatic in Rust and preferred in most cases.
  ```rust
  fn example() -> Deck {
      Deck::new() // No semicolon, so this is returned
  }
  ```

- Both approaches are equivalent, but the implicit return is more concise and aligns with Rust's conventions.

### Summary

- Use **methods** when you need to operate on or modify a specific instance of a struct.
- Use **associated functions** for constructors or other functionality that is not tied to a specific instance.
- Prefer **implicit returns** for simplicity and idiomatic Rust code.
