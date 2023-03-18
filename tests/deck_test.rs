use mus_oxidado::deck::*;

#[test]
fn deck_must_have_40_cards() {
  let deck = Deck::new();
  assert_eq!(
    deck.cards.len(),
    40,
  )
}

#[test]
fn deck_must_be_shuffled() {
  let mut deck = Deck::new();
  let shuffled_deck = Deck::new();
  deck.shuffle();

  for idx in 0..40 {
    let card = deck.cards.get(idx);
    let shuffled_card = shuffled_deck.cards.get(idx);

    if card != shuffled_card {
      assert!(true);
      return
    }
  }
  assert!(false);
}

#[test]
fn drawing_a_card_removes_a_card_from_the_deck() {
  let mut deck = Deck::new();
  deck.shuffle();
  let _card = deck.draw_card();

  assert_eq!(deck.cards.len(), 39);
}

#[test]
fn drawing_a_card_without_shuffling_gets_cards_in_order() {
  let mut deck = Deck::new();

  let first_card = Card::new(Suit::Oros, Rank::As);
  let fifteenth_card = Card::new(Suit::Copas, Rank::Cinco);
  let twenty_eighth_card = Card::new(Suit::Espadas, Rank::Sota);
  let thirty_seventh_card = Card::new(Suit::Bastos, Rank::Siete);

  // 1st card must be as de oros
  match deck.draw_card()  {
    Ok(drawed_card) => assert_eq!(drawed_card, first_card),
    Err(_) => assert!(false)
  }

  // Next cards must not be empty
  for _ in 0..13 {
    assert!(deck.draw_card().is_ok()) 
  }

  // 15th card must be cinco de copas
  match deck.draw_card()  {
    Ok(drawed_card) => assert_eq!(drawed_card, fifteenth_card),
    Err(_) => assert!(false)
  }

  // Next cards must not be empty
  for _ in 15..27 {
    assert!(deck.draw_card().is_ok()) 
  }

  // 28th card must be sota de espadas
  match deck.draw_card()  {
    Ok(drawed_card) => assert_eq!(drawed_card, twenty_eighth_card),
    Err(_) => assert!(false)
  }

  // Next cards must not be empty
  for _ in 29..37 {
    assert!(deck.draw_card().is_ok())
  }

  // 37th card must be siete de bastos
  match deck.draw_card()  {
    Ok(drawed_card) => assert_eq!(drawed_card, thirty_seventh_card),
    Err(_) => assert!(false)
  }

}

#[test]
fn drawing_more_cards_than_the_deck_returns_err () {
  let mut deck = Deck::new();

  for _ in 0..40 { assert!(deck.draw_card().is_ok()) }

  assert!(deck.draw_card().is_err());
}

#[test]
fn add_a_card_to_a_hand() {
  let mut hand = Hand::new();
  let card = Card::new(Suit::Bastos, Rank::Tres);

  assert!(hand.add_card(card).is_ok());

  assert_eq!(hand.get_cards()[0], Card::new(Suit::Bastos, Rank::Tres));
  assert_eq!(hand.get_cards().len(), 1);
}

#[test]
fn add_multiple_cards_to_a_hand() {
  let mut hand = Hand::new();
  let card1 = Card::new(Suit::Bastos, Rank::Tres);
  let card2 = Card::new(Suit::Bastos, Rank::Tres);
  let card3 = Card::new(Suit::Bastos, Rank::Tres);
  let card4 = Card::new(Suit::Bastos, Rank::Tres);

  let cards = [card1, card2, card3, card4];

  assert!(hand.add_cards(cards.into_iter()).is_ok());
  assert_eq!(hand.get_cards().len(), 4)
}

#[test]
fn hand_cant_have_more_than_4_cards() {
  let mut hand = Hand::new();
  let card1 = Card::new(Suit::Bastos, Rank::Tres);
  let card2 = Card::new(Suit::Bastos, Rank::Tres);
  let card3 = Card::new(Suit::Bastos, Rank::Tres);
  let card4 = Card::new(Suit::Bastos, Rank::Tres);
  let card5 = Card::new(Suit::Bastos, Rank::Tres);

  assert!(hand.add_card(card1).is_ok());
  assert!(hand.add_card(card2).is_ok());
  assert!(hand.add_card(card3).is_ok());
  assert!(hand.add_card(card4).is_ok());
  assert!(hand.add_card(card5).is_err());
}

#[test]
fn remove_card_from_hand() {
  let mut hand =  Hand::new();
  let card_to_add = Card::new(Suit::Bastos, Rank::Tres);
  let card_to_remove = Card::new(Suit::Bastos, Rank::Tres);

  hand.add_card(card_to_add).unwrap();
  assert!(hand.remove_card(card_to_remove).is_ok());
  assert_eq!(hand.get_cards().len(), 0);
}

#[test]
fn remove_several_cards_from_hand() {
  let mut hand = Hand::new();
  let card1 = Card::new(Suit::Bastos, Rank::Tres);
  let card2 = Card::new(Suit::Oros, Rank::Tres);
  let card3 = Card::new(Suit::Espadas, Rank::Tres);
  let card4 = Card::new(Suit::Bastos, Rank::As);

  let cards = [card1, card2, card3, card4];
  hand.add_cards(cards.into_iter()).unwrap();

  let card_to_remove1 = Card::new(Suit::Bastos, Rank::Tres);
  let card_to_remove2 = Card::new(Suit::Oros, Rank::Tres);
  let card_to_remove3 = Card::new(Suit::Espadas, Rank::Tres);

  let cards_to_remove = [
    card_to_remove1,
    card_to_remove2,
    card_to_remove3
    ];

  assert!(hand.remove_cards(cards_to_remove.into_iter()).is_ok());
  assert_eq!(hand.get_cards().len(), 1);
}

#[test]
fn cant_remove_cards_from_an_empty_hand() {
  let mut hand = Hand::new();
  let card = Card::new(Suit::Bastos, Rank::As);

  assert!(hand.remove_card(card).is_err());
}

#[test]
fn cant_remove_a_card_that_is_not_in_hand() {
  let mut hand = Hand::new();
  let card_to_add = Card::new(Suit::Bastos, Rank::Tres);
  let card_to_remove = Card::new(Suit::Oros, Rank::Tres);

  hand.add_card(card_to_add).unwrap();
  assert!(hand.remove_card(card_to_remove).is_err());

}