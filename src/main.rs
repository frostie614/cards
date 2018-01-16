#![warn(dead_code)]

extern crate rand;

use rand::{thread_rng, Rng};
use std::fmt;



fn main() {

}

#[derive(Copy, Clone, Debug)]
struct Card{
	suit:Suit,
	pip:Pip
}

#[derive(Clone, Debug)]
struct Deck(Vec<Card>);

#[derive(Copy, Clone, Debug)]
enum Suit {Hearts, Diamonds, Spades, Clubs}

#[derive(Copy, Clone, Debug)]
enum Pip{Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} of {:?}", self.pip, self.suit)
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for card in self.0.iter() {
            writeln!(f, "{}", card);
        }
        write!(f, "")
    }
}


impl Deck {

	fn new()-> Deck{
		let mut deck:Vec<Card> = Vec::with_capacity(52);
		for suit in vec![Suit::Hearts, Suit::Diamonds, Suit::Spades, Suit::Clubs]{
			for pip in vec![Pip::Ace, Pip::Two, Pip::Three, Pip::Four, Pip::Five, Pip::Six, Pip::Seven, Pip::Eight, Pip::Nine, Pip::Ten, Pip::Jack, Pip::Queen, Pip::King]{
				deck.push(Card::new(suit, pip));
			}
		}
		Deck(deck)
	}

	fn deal(&mut self) -> Option<Card>{
		self.0.pop()
	}

	fn shuffle(&mut self){
		thread_rng().shuffle(&mut self.0);
	}
}

impl Card{

	fn new(suit:Suit, pip:Pip) -> Card{
		Card{suit, pip}
	}

	fn value(&self) -> u8 {
        match self.pip {
            Pip::Ace => 1,
            Pip::Two => 2,
            Pip::Three => 3,
            Pip::Four => 4,
            Pip::Five => 5,
            Pip::Six => 6,
            Pip::Seven => 7,
            Pip::Eight => 8,
            Pip::Nine => 9,
            Pip::Ten => 10,
            Pip::Jack => 11,
            Pip::Queen => 12,
            Pip::King => 13
        }
    }
}