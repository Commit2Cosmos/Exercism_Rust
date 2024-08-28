use std::collections::HashMap;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.

#[allow(dead_code)]
#[derive(PartialEq, PartialOrd, Debug, Eq, Hash, Ord, Copy, Clone)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

#[derive(Debug, PartialEq)]
pub enum Suit {
    Clubs,
    Spades,
    Hearts,
    Diamonds
}

#[allow(dead_code)]
#[derive(PartialEq, PartialOrd, Debug, Ord, Eq)]
pub enum HandType {
    //* sorted hand highest -> lowest */
    HighCard(Card, Card, Card, Card, Card),
    //* pair, remaining (in sorted order) */
    Pair(Card, Card, Card, Card),
    //* highest pair, lowest pair, remaining */
    TwoPair(Card, Card, Card),
    //* three, remaining (in sorted order) */
    Three(Card, Card, Card),
    //* highest card in straight (!ace can be lowest!) */
    Straight(Card),
    //* cards of the same suit in sorted order */
    Flush(Card, Card, Card, Card, Card),
    //* three, pair */
    FullHouse(Card, Card),
    //* four, remaining */
    Four(Card, Card),
    //* highest card in straight (!ace can be lowest!) */
    StraightFlush(Card)
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut c = hands.iter().enumerate().map(|(i, x)| {
        let (values, suits) = x.split_whitespace().map(|c| {
            let mut c = c.chars();
            (char_to_card(c.next().unwrap()), char_to_suit(c.last().unwrap()))
        })
        .unzip::<Card, Suit, Vec<Card>, Vec<Suit>>();

        (i, find_combinations(values, suits))
    })
    .collect::<Vec<_>>();

    println!("{:?}", c);

    c.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let idx: Vec<_> = c.iter().filter_map(|x| {
        if x.1 == c[0].1 {
            Some(x.0)
        } else {
            None
        }
    })
    .collect();

    println!("{:?}", idx);

    idx.iter().map(|&x| hands[x]).collect()
}


fn find_combinations<'a>(cards: Vec<Card>, suits: Vec<Suit>) -> HandType {

    let is_flush = suits.iter().all(|x| *x == suits[0]);
    
    let mut counts: HashMap<Card, u8> = HashMap::new();

    for card in cards {
        *counts.entry(card).or_insert(0) += 1;
    }

    let mut counts: Vec<_> = counts.iter().collect();
    counts.sort_by(|a, b| b.1.cmp(a.1).then_with(|| b.0.cmp(a.0)));

    let (cards, counts): (Vec<Card>, Vec<u8>) = counts.iter().cloned().unzip();

    match counts.as_slice() {
        [4, 1] => HandType::Four(cards[0], cards[1]),
        [3, 2] => HandType::FullHouse(cards[0], cards[1]),
        [3, 1, 1] => HandType::Three(cards[0], cards[1], cards[2]),
        [2, 2, 1] => HandType::TwoPair(cards[0], cards[1], cards[2]),
        [2, 1, 1, 1] => HandType::Pair(cards[0], cards[1], cards[2], cards[3]),
        _ => {
            let is_straight = (0..4).all(|i| {
                cards[i] as u8 == cards[i+1] as u8 + 1
            });

            if cards == vec![Card::Ace, Card::Five, Card::Four, Card::Three, Card::Two] {
                if is_flush {
                    return HandType::StraightFlush(cards[1]);
                } else {
                    return HandType::Straight(cards[1]);
                }
            }

            match (is_flush, is_straight) {
                (true, true) => HandType::StraightFlush(cards[0]),
                (true, false) => HandType::Flush(cards[0], cards[1], cards[2], cards[3], cards[4]),
                (false, true) => HandType::Straight(cards[0]),
                (false, false) => HandType::HighCard(cards[0], cards[1], cards[2], cards[3], cards[4])
            }
        }
    }
}


fn char_to_card(c: char) -> Card {
    match c {
        '2' => Card::Two,
        '3' => Card::Three,
        '4' => Card::Four,
        '5' => Card::Five,
        '6' => Card::Six,
        '7' => Card::Seven,
        '8' => Card::Eight,
        '9' => Card::Nine,
        '1' => Card::Ten,
        'J' => Card::Jack,
        'Q' => Card::Queen,
        'K' => Card::King,
        'A' => Card::Ace,
        _ => unreachable!()
    }
}

fn char_to_suit(c: char) -> Suit {
    match c {
        'C' => Suit::Clubs,
        'S' => Suit::Spades,
        'H' => Suit::Hearts,
        'D' => Suit::Diamonds,
        _ => unreachable!()
    }
}