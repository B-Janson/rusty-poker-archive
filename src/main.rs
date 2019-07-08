use arrayvec::ArrayVec;
use arr_macro::arr;
use std::fmt;
use rand::prelude::*;

#[derive(Debug, Clone)]
struct Card {
    suit: String,
    value: u32,
}

impl Card {
    fn new(suit: String, value: u32) -> Card {
        Card {
            suit,
            value
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", value_to_name(self.value), self.suit)
    }
}

#[derive(Debug)]
struct Player {
    name: String,
    hand: [Card; 2],
}

impl Player {
    fn new(name: String) -> Player {
        Player {
            name,
            hand: arr![Card::new({"".to_string()}, {0}); 2]
        }
    }
}

const SUITS: [&str; 4] = ["Hearts", "Diamonds", "Spades", "Clubs"];
const NUM_OF_PLAYERS : usize = 6;

fn value_to_name(value: u32) -> String {
    match value {
        1 => "Ace".to_string(),
        2 => "Two".to_string(),
        3 => "Three".to_string(),
        4 => "Four".to_string(),
        5 => "Five".to_string(),
        6 => "Six".to_string(),
        7 => "Seven".to_string(),
        8 => "Eight".to_string(),
        9 => "Nine".to_string(),
        10 => "Ten".to_string(),
        11 => "Jack".to_string(),
        12 => "Queen".to_string(),
        13 => "King".to_string(),
        _ => "None".to_string()
    }
}

fn main() {
    let mut rng = thread_rng();

    let mut cards: [Card; 52] = arr![Card::new({"".to_string()}, {0}); 52];
    let mut shared_cards: [Card; 5] = arr![Card::new({"".to_string()}, {0}); 5];
    let mut i = 0;

     // Generate all cards
    for suit in SUITS.iter() {
        for value in 1..=13 {
            let new_card = Card::new(suit.to_string(), value);
            cards[i] = new_card;
            i = i + 1;
        }
    }

    // Shuffle the cards
    cards.shuffle(&mut rng);

    // Display the shuffled deck
    for i in 0..52 {
        println!("{:}", cards[i]);
    }

    // Initialise the array of players
    let mut players: [Player; NUM_OF_PLAYERS] = arr![Player::new({"".to_string()}); 6];

    for i in 0..NUM_OF_PLAYERS {
        let player_name = format!("Player {}", (i+1));
        let new_player = Player::new(player_name.to_string());
        players[i] = new_player;
    }

    // Give each player their starting cards
    for i in 0..NUM_OF_PLAYERS * 2 {
        let player = i % NUM_OF_PLAYERS;
        let card = i / NUM_OF_PLAYERS;
        players[player].hand[card] = cards[i].clone();
    }

    for i in players.iter() {
        println!("{:#?}", i);
    }




}
