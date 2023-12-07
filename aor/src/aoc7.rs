use crate::download;


#[derive(Debug, PartialEq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn value(&self) -> u32 {
        match self {
            HandType::HighCard => 0,
            HandType::OnePair => 1,
            HandType::TwoPair => 2,
            HandType::ThreeOfAKind => 3,
            HandType::FullHouse => 4,
            HandType::FourOfAKind => 5,
            HandType::FiveOfAKind => 6,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Card {
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
    Ace,
}

impl Card {
    fn value(&self) -> u32 {
        match self {
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::Ten => 10,
            Card::Jack => 1,
            Card::Queen => 12,
            Card::King => 13,
            Card::Ace => 14,
        }
    }
}

struct Hand {
    cards: Vec<Card>,
    bid: u64,
}

impl Hand {
    fn new(cards: Vec<Card>, bid: u64) -> Hand {
        assert_eq!(cards.len(), 5);
        Hand { cards, bid }
    }

    fn get_hand_type(&self) -> HandType {
        // get counts of each card
        let mut counts = [0; 13];
        for card in &self.cards {
            counts[*card as usize] += 1;
        }
        let joker_count = counts[Card::Jack as usize];
        // println!("{} {} {:?}", Card::Jack.value(), joker_count, counts);
        // if there are any jokers, add them to the arg with highest count
        if joker_count > 0 {
            let mut max_count = 0;
            let mut max_count_index = 0;
            for (i, &count) in counts.iter().enumerate() {
                // skip joker card
                if i == Card::Jack as usize {
                    continue;
                }
                if count > max_count {
                    max_count = count;
                    max_count_index = i;
                }
            }
            counts[max_count_index] += joker_count;
            counts[Card::Jack as usize] = 0;
        }
        // if there are any 5s, it's a five of a kind
        if counts.contains(&5) {
            return HandType::FiveOfAKind;
        }
        // if there are any 4s, it's a four of a kind
        if counts.contains(&4) {
            return HandType::FourOfAKind;
        }
        // if there are any 3s, either a full house or three of a kind
        if counts.contains(&3) {
            // if there are any 2s, it's a full house
            if counts.contains(&2) {
                return HandType::FullHouse;
            }
            // otherwise, it's a three of a kind
            return HandType::ThreeOfAKind;
        }
        // if there are any 2s, either a two pair or one pair
        if counts.contains(&2) {
            // if there are any other 2s, it's a two pair
            if counts.iter().filter(|&&x| x == 2).count() == 2 {
                return HandType::TwoPair;
            }
            // otherwise, it's a one pair
            return HandType::OnePair;
        }
        return HandType::HighCard;
    }
}


pub fn main() {
    let input = download::get_input("https://adventofcode.com/2023/day/7/input").expect("Couldn't download input");
    // hand
    let mut hands: Vec<Hand> = Vec::new();
    for line in input.lines() {
        // parse hand and bid
        let mut hand_and_bid = line.split_whitespace();
        let hand = hand_and_bid.next().unwrap();
        let bid = hand_and_bid.next().unwrap().parse::<u64>().unwrap();
        // parse hand
        let cards = hand.chars().map(|c| match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card"),
        }).collect::<Vec<Card>>();
        let hand = Hand::new(cards, bid);
        hands.push(hand);
    }
    // sort hands by comparing hand type and then cards in order
    hands.sort_by(|a, b| {
        let a_type = a.get_hand_type();
        let b_type = b.get_hand_type();
        if a_type.value() > b_type.value() {
            return std::cmp::Ordering::Greater;
        } else if a_type.value() < b_type.value() {
            return std::cmp::Ordering::Less;
        }
        // if hand types are equal, compare cards
        for i in 0..5 {
            if a.cards[i] != b.cards[i] {
                if a.cards[i].value() > b.cards[i].value() {
                    return std::cmp::Ordering::Greater;
                } else if a.cards[i].value() < b.cards[i].value() {
                    return std::cmp::Ordering::Less;
                }
            }
        }
        return std::cmp::Ordering::Equal;
    });
    let mut total_winnings: u64 = 0;
    // enumerate over each sorted hand
    for (i, hand) in hands.iter().enumerate() {
        total_winnings += hand.bid * (i as u64 + 1);
    }
    println!("Total winnings: {}", total_winnings);
}