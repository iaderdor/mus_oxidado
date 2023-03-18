use std::collections::VecDeque;

use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Debug, PartialEq, Eq, Copy, Clone, EnumIter)]
pub enum Suit {
    Oros,
    Copas,
    Espadas,
    Bastos,
}

#[derive(Debug, PartialEq, Eq, EnumIter)]
pub enum Rank {
    As,
    Dos,
    Tres,
    Cuatro,
    Cinco,
    Seis,
    Siete,
    Sota,
    Caballo,
    Rey
}

#[derive(Debug)]
pub struct EmptyDeckError;

#[derive(PartialEq, Eq, Debug)]
pub struct Card {
    suit: Suit,
    rank: Rank
}

impl Card {
  pub fn new(suit: Suit, rank: Rank) -> Self {
    Card { suit, rank }
  }
}

pub struct Deck {
    pub cards: VecDeque<Card>,
}

impl Deck {
  pub fn new() -> Self {
    let mut cards = VecDeque::<Card>:: new();

    for suit in Suit::iter() {
      for rank in Rank::iter() {
        cards.push_back(Card::new(suit, rank));
      }
    }
  
  Deck { cards }
  }

  pub fn shuffle(&mut self) {
    let mut rng = thread_rng();

    self.cards.make_contiguous().shuffle(&mut rng);
  }

  pub fn draw_card(&mut self) -> Result<Card, EmptyDeckError> {
    self.cards.pop_front().ok_or(EmptyDeckError)
  }
}

//#[derive(Debug)]
//pub struct EmptyHandError;
//
//#[derive(Debug)]
//pub struct FullHandError;
//
//#[derive(Debug)]
//pub struct CardNotInHandError;

#[derive(Debug)]
pub enum HandError {
  EmptyHandError,
  FullHandError,
  CardNotInHandError,
}

const MAX_CARDS_IN_HAND: usize = 4;

pub struct Hand {
  cards: Vec<Card>,
}

impl Hand {
  pub fn new() -> Self {
    let cards = Vec::new();
  
  Hand { cards: cards }
  }

  pub fn add_card(&mut self, card: Card) -> Result<(), HandError>{
    if self.cards.len() >= MAX_CARDS_IN_HAND {
      return Err(HandError::FullHandError);
    }

    self.cards.push(card);
    Ok(())
  }

  pub fn add_cards<I>(&mut self, cards: I) -> Result<(), HandError>
  where
    I: Iterator<Item = Card>
  {

    for card in cards {
      self.add_card(card)?
    }

    Ok(())
  }

  pub fn remove_card(&mut self, card: Card) -> Result<(), HandError> {
    if self.cards.len() <= 0 {
      return Err(HandError::EmptyHandError)
    }

    match self.cards.iter().position(|x| *x == card) {
      Some(index) => self.cards.remove(index),
      None => return Err(HandError::CardNotInHandError),
    };

    Ok(())
  }

  pub fn remove_cards<I>(&mut self, cards: I) -> Result<(), HandError>
  where
    I: Iterator<Item = Card>
  {
    for card in cards {
      self.remove_card(card)?
    }

    Ok(())
  }

  pub fn get_cards(&self) -> &Vec<Card>{
    &self.cards
  }
}