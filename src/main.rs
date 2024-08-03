use rand::thread_rng;
#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

// we are writing an inherent implementation
impl Deck {
    // Self is a reference to the type that is mentioned in the partent implementation block. It's like this in JS
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamonds", "Clubs"];
        let values = [
            "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack",
            "Queen", "King", "Ace",
        ];
        let mut cards = vec![];
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }
        Deck { cards }
    }

    fn shuffle(&self) {
        let rng = thread_rng();
    }
}

fn main() {
    // :? = debug formatter
    // Formatters are often used in Rust for example when displaying decimals
    let deck = Deck::new();

    println!("Here's your deck: {:#?}", deck)
}
